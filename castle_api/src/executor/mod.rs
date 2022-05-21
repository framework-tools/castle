use std::{collections::HashMap, fmt::Error};

use crate::{context::Context, Directive, Next, Resolver, Value};
use async_recursion::async_recursion;
use castle_error::CastleError;
use castle_query_parser::{Field, Message};
use castle_schema_parser::types::{
    AppliedDirective, FieldDefinition, SchemaDefinition, TypeDefinition,
};

pub async fn execute_message(
    message: &mut Message,
    field_resolvers: &HashMap<Box<str>, Box<dyn Resolver>>,
    directives: &HashMap<Box<str>, Box<dyn Directive>>,
    schema: &SchemaDefinition,
    ctx: &Context,
) -> Result<(Value, Vec<Error>), CastleError> {
    let mut errors = Vec::new();
    let data = evaluate_map(
        message,
        schema.types.get("Root").unwrap(),
        field_resolvers,
        directives,
        schema,
        ctx,
        &mut errors,
    ).await?;

    Ok((Value::Object(data), errors))
}

async fn evaluate_map(
    message: &mut Message,
    type_def: &TypeDefinition,
    field_resolvers: &HashMap<Box<str>, Box<dyn Resolver>>,
    directives: &HashMap<Box<str>, Box<dyn Directive>>,
    schema: &SchemaDefinition,
    ctx: &Context,
    errors: &mut Vec<Error>,
) -> Result<HashMap<Box<str>, Value>, CastleError> {
    let mut map = HashMap::new();

    for (field_name, field) in message.projection.iter() {
        let field_def = type_def.fields.get(field_name).unwrap();

        let resolver = field_resolvers.get(field_name).unwrap();

        match evaluate_field(
            field,
            field_def,
            &field_def.directives[..],
            ctx,
            resolver,
            &directives,
        )
        .await?
        {
            Ok(Value::Void) => {}
            Ok(data) => {
                map.insert(field_name.clone(), data);
            }
            Err(e) => {
                errors.push(e);
            }
        }
    }

    Ok(map)
}

#[async_recursion]
async fn evaluate_field(
    field: &Field,
    field_def: &FieldDefinition,
    remaining_directives: &[AppliedDirective],
    ctx: &Context,
    resolver: &Box<dyn Resolver>,
    directives: &HashMap<Box<str>, Box<dyn Directive>>,
) -> Result<Result<Value, Error>, CastleError> {
    match remaining_directives.get(0) {
        Some(applied_directive) => {
            let directive = match directives.get(&applied_directive.ident) {
                Some(directive) => directive,
                None => {
                    return Err(CastleError::Validation(
                        "Validation did not catch error".into(),
                    ))
                }
            };

            let (sender, mut wait_next) = tokio::sync::mpsc::channel(1);
            let next = Next { sender };
            let mut value_fut = directive
                .field_visitor(field, &applied_directive.inputs, next, ctx);

            loop {
                tokio::select! {
                    Some(sender) = wait_next.recv() => {
                        let _ = sender.send(evaluate_field(
                            field,
                            field_def,
                            &remaining_directives[1..],
                            ctx,
                            resolver,
                            directives
                        ).await?);
                        continue
                    }
                    value = &mut value_fut => return Ok(value)
                }
            }
        }
        None => Ok(resolver.resolve_recursively(field, ctx).await),
    }
}
