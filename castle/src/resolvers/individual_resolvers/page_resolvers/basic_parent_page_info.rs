use std::collections::{HashMap, HashSet};

use parser_and_schema::ast::syntax_definitions::want::Wants;
use shared::CastleError;

use crate::{castle_object::resolver_return_types::Value, resolvers::{resolve_query_wants::resolve_all_wants, resolver_type::{ResolverArguments, Args}, resolver_map::ResolverMap, generic_resolver_fn::generic_resolver, dummy_data_for_tests::get_requested_fields_from_db_dummy}};

/// type BasicPageInfo {
///    title: String,
///    icon: Option<String>,
///    emoji: Option<String>
/// }

pub fn basic_parent_page_info<'a, C, R>(wants: Option<&Wants>, args: &Args, resolver_map: &ResolverMap<C, R>, context: &C) 
-> Result<Value<R>, CastleError> {
    let mut possible_fields = HashSet::new();
    possible_fields.insert("title".into());
    possible_fields.insert("icon".into());
    possible_fields.insert("emoji".into());

    //this dummy data is strictly for the test & will be replaced with
    //two steps: sending the wants to the DB & then receiving their values
    let dummy_data = get_requested_fields_from_db_dummy(&mut possible_fields, wants, args, context, Value::Object(HashMap::new()))?;
    let resolved_wants = generic_resolver(wants, &possible_fields, args, resolver_map, context, Value::Object(dummy_data))?;
    return Ok(resolved_wants)
}