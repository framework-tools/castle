use std::collections::HashMap;

use castle_error::CastleError;
use query_parser::{Projection, Message};
use std::fmt::Debug;
use crate::{Resolver, Directive, Value};



pub async fn execute_message<Ctx: Debug>(
    message: Message,
    field_resolvers: &HashMap<Box<str>, Box<dyn Resolver<Ctx>>>,
    directives: &HashMap<Box<str>, Box<dyn Directive<Ctx>>>,
    ctx: &Ctx,
) -> Result<Value<Ctx>, CastleError> {
    for projection in message.projections {
        for (field_name, field) in projection.iter() {
            let resolver = match field_resolvers.get(field_name) {
                Some(resolver) => resolver,
                None => return Err(CastleError::Validation("Should never happen, lol".into())),
            };
            let value = resolver.resolve(field, ctx).await?;

        }
    }
    return Ok(Value::Empty);
}