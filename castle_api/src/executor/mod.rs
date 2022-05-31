use std::{collections::HashMap};


use async_recursion::async_recursion;
use castle_types::{Directive, Context, Value, Next, Message, SchemaDefinition, TypeDefinition, Field, FieldDefinition, AppliedDirective};

use crate::Resolver;

pub async fn execute_message(
    message: &mut Message,
    directives: &HashMap<Box<str>, Box<dyn Directive>>,
    schema: &SchemaDefinition,
    ctx: &Context,
) -> (Value, Vec<anyhow::Error>) {
    let mut errors = Vec::new();
    let data = evaluate_map(
        message,
        schema.types.get("Root").unwrap(),
        directives,
        schema,
        ctx,
        &mut errors,
    )
    .await;

    (Value::Object(data), errors)
}

async fn evaluate_map(
    message: &mut Message,
    type_def: &TypeDefinition,
    directives: &HashMap<Box<str>, Box<dyn Directive>>,
    schema: &SchemaDefinition,
    ctx: &Context,
    errors: &mut Vec<anyhow::Error>,
) -> HashMap<Box<str>, Value> {
    let mut map = HashMap::new();

    for (field_name, field) in message.projection.iter() {
        let field_def = type_def.fields.get(field_name).unwrap();
        // let resolver = field_resolvers.get(field_name).unwrap();

        // match evaluate_field(
        //     field,
        //     field_def,
        //     &field_def.directives[..],
        //     ctx,
        //     resolver,
        //     &directives,
        // )
        // .await
        // {
        //     Ok(Value::Void) => {}
        //     Ok(data) => {
        //         map.insert(field_name.clone(), data);
        //     }
        //     Err(e) => errors.push(e),
        // }
    }

    map
}

#[async_recursion]
async fn evaluate_field(
    field: &Field,
    field_def: &FieldDefinition,
    remaining_directives: &[AppliedDirective],
    ctx: &Context,
    resolver: &Box<dyn Resolver>,
    directives: &HashMap<Box<str>, Box<dyn Directive>>,
) -> Result<Value, anyhow::Error> {
    match remaining_directives.get(0) {
        Some(applied_directive) => {
            let directive = match directives.get(&applied_directive.ident) {
                Some(directive) => directive,
                None => unreachable!(), // we should have already validated the directives
            };

            let (sender, mut wait_next) = tokio::sync::mpsc::channel(1);
            let next = Next { sender };
            let mut value_fut =
                directive.field_visitor(field, &applied_directive.inputs, next, ctx);

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
                        ).await);
                        continue
                    }
                    value = &mut value_fut => return Ok(value?)
                }
            }
        }
        None => Ok(resolver.resolve(field, ctx).await?),
    }
}
