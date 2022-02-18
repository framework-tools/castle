use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::want::Wants;
use shared::CastleError;

use crate::{castle_object::resolver_return_types::Value, resolvers::{resolve_query_wants::resolve_all_wants, resolver_type::{ResolverArguments, Args}, resolver_map::ResolverMap}};

/// 
pub fn page_info<'a, C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) 
-> Result<Value<R>, CastleError> {
    let possible_fields = [
        "basic_page_info", //this will be an object projection
        "description",
        "parent",
        "blocks"
    ];

    let mut resolved_wants = HashMap::new();
    if wants.is_some() {
        let wants = wants.unwrap();
        resolved_wants = resolve_all_wants(wants, resolver_map, context)?;
    }
    return Ok(resolved_wants)
}