use std::collections::HashMap;

use castle_error::CastleError;
use query_parser::{Message};
use std::fmt::Debug;
use crate::{Resolver, Directive, types::result::CastleResult};



pub async fn execute_message<Ctx: Debug, E: Debug>(
    message: &mut Message,
    field_resolvers: &HashMap<Box<str>, Box<dyn Resolver<Ctx, E>>>,
    directives: &HashMap<Box<str>, Box<dyn Directive<Ctx, E>>>,
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
            Ok(data) => { result.data.insert(field_name.clone(), data); },
            Err(e) => { result.errors.push(e); }
        };
    }
    Ok(result)
}
