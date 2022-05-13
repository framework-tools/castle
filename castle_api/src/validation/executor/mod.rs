use std::collections::HashMap;

use castle_error::CastleError;
use castle_query_parser::{Message};
use crate::{Resolver, Directive, types::result::CastleResult, Value};



pub async fn execute_message<Ctx, E>(
    message: &mut Message,
    field_resolvers: &HashMap<Box<str>, Box<dyn Resolver<Ctx, E>>>,
    _directives: &HashMap<Box<str>, Box<dyn Directive<Ctx, E>>>,
    ctx: &Ctx,
) -> Result<CastleResult<Ctx, E>, CastleError> {
    let mut result = CastleResult {
        data: HashMap::new(),
        errors: Vec::new()
    };
    for (field_name, field) in message.projection.iter() {
        let resolver = match field_resolvers.get(field_name) {
            Some(resolver) => resolver,
            None => unreachable!(),
        };
        match resolver.resolve(field, ctx).await {
            Ok(Value::Void) => {},
            Ok(data) => { result.data.insert(field_name.clone(), data); },
            Err(e) => { result.errors.push(e); }
        };
    }
    Ok(result)
}
