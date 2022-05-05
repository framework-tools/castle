use std::collections::HashMap;

use castle_error::CastleError;
use query_parser::{Message};
use std::fmt::Debug;
use crate::{Resolver, Directive, Value};



pub async fn execute_message<Ctx: Debug, E: Debug>(
    message: &mut Message,
    field_resolvers: &HashMap<Box<str>, Box<dyn Resolver<Ctx, E>>>,
    directives: &HashMap<Box<str>, Box<dyn Directive<Ctx, E>>>,
    ctx: &Ctx,
) -> Result<Result<Value<Ctx, E>, E>, CastleError> {
    for projection in message.projections.iter_mut() {
        for (field_name, field) in projection.iter() {
            let resolver = match field_resolvers.get(field_name) {
                Some(resolver) => resolver,
                None => unreachable!(),
            };
            let value = resolver.resolve(field, ctx).await;
        }
    }




    unimplemented!()
}