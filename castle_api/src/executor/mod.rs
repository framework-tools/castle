use std::collections::HashMap;

use crate::{types::result::CastleResult, Directive, Next, Resolver, Value};
use async_recursion::async_recursion;
use castle_error::CastleError;
use castle_query_parser::{Field, Message};
use castle_schema_parser::types::{SchemaDefinition, FieldDefinition, AppliedDirective, TypeDefinition};

pub async fn execute_message<Ctx: Send + Sync, E: Send + Sync + 'static>(
    message: &mut Message,
    field_resolvers: &HashMap<Box<str>, Box<dyn Resolver<Ctx, E>>>,
    directives: &HashMap<Box<str>, Box<dyn Directive<Ctx, E>>>,
    schema: &SchemaDefinition,
    ctx: &Ctx,
) -> Result<CastleResult<Ctx, E>, CastleError> {
    let mut result = CastleResult {
        data: HashMap::new(),
        errors: Vec::new(),
    };
    result.data = evaluate_map(message, schema.types.get("Root").unwrap(), field_resolvers, directives, schema, ctx, &mut result.errors).await?;
    Ok(result)
}

async fn evaluate_map<Ctx: Send + Sync, E: Send + Sync + 'static>(
    message: &mut Message, 
    type_def: &TypeDefinition,
    field_resolvers: &HashMap<Box<str>, Box<dyn Resolver<Ctx, E>>>, 
    directives: &HashMap<Box<str>, Box<dyn Directive<Ctx, E>>>, 
    schema: &SchemaDefinition, 
    ctx: &Ctx,
    errors: &mut Vec<E>,
) -> Result<HashMap<Box<str>, Value<Ctx, E>>, CastleError> {
    let mut map = HashMap::new();

    for (field_name, field) in message.projection.iter() {
        let field_def = type_def
            .fields
            .get(field_name)
            .unwrap();

        let resolver = field_resolvers
            .get(field_name)
            .unwrap();
            
        match evaluate_field(field, field_def, &field_def.directives[..], ctx, resolver, &directives).await? {
            Ok(Value::Void) => {},
            Ok(data) => { map.insert(field_name.clone(), data); },
            Err(e) => { errors.push(e); }
        }
    }
    Ok(map)
}

/// evaluate_field(field, field_def, remaining_directives, field_resolvers, ctx) -> Result<Value<Ctx, E>, E>
/// - match remaining_directives.get(i)
///     - Some(directive)
///         - let directive_args be the arguments of the directive
///         - let (next, wait_next) be a oneshot channel
///         - loop
///             - future select
///                 - sender = wait_next.recv()
///                     - let remaining_directives be a slice of remaining_directives[1..remaining_directives.len() - 1]
///                     - sender.send(evaluate_field(field, field_def, remaining_directives, field_resolvers, ctx))
///                 - value = directive.field_resolver(field_def, directive_args, ctx, next)
///                     - return value
///    - None
///       return resolver.resolve
///             
#[async_recursion]
async fn evaluate_field<Ctx: Send + Sync, E: Send + Sync + 'static>(
    field: &Field,
    field_def: &FieldDefinition,
    remaining_directives: &[AppliedDirective],
    ctx: &Ctx,
    resolver: &Box<dyn Resolver<Ctx, E>>,
    directives: &HashMap<Box<str>, Box<dyn Directive<Ctx, E>>>,
) -> Result<Result<Value<Ctx, E>, E>, CastleError> {
    match remaining_directives.get(0) {
        Some(applied_directive) => {
            let directive = match directives.get(&applied_directive.ident) {
                Some(directive) => directive,
                None => return Err(CastleError::Validation("Validation did not catch error".into())),
            };

            let (sender, mut wait_next) = tokio::sync::mpsc::channel(1);
            let next = Next {
                sender
            };
            let mut value_fut = directive.field_visitor(field, &applied_directive.inputs, next, ctx);
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
        None => Ok(resolver.resolve_recursively(field, ctx).await)
    }
}
