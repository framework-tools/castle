use std::collections::{HashMap, HashSet};

use parser_and_schema::ast::syntax_definitions::want::{Wants, Want};
use shared::CastleError;

use crate::castle_struct::resolver_return_types::Value;

use super::{resolvers::{Args, ResolverMap}, get_dummy_data_for_tests::get_requested_value_or_fields_from_db_dummy};

/// this takes in a top level want (object projection or single field)
/// the possible fields for this specific resolver/want are passed in
/// this will then check which fields are in the want and resolve them (throws error if want is a single field and not found in possible fields)
/// the requested wants will get sent to the database and the results will be returned
/// for now dummy data will be used
/// once data has been received, insert all data into the return_value
/// if there is a inner object projection, this calls a new resolver (handled in fn generic_resolve_wants)
pub fn generic_resolver<C, R>(
    wants: Option<&Wants>, 
    possible_fields: &HashSet<Box<str>>,
    args: &Args, 
    resolver_map: &ResolverMap<C, R>, 
    context: &C,
    dummy_data: Value<R> // this is dummy data and will be changed
) -> Result<Value<R>, CastleError> {
    //currently dummy data below
    let fields_with_values_from_db: HashMap<Box<str>, Value<R>> = get_requested_value_or_fields_from_db_dummy(possible_fields, wants, args, context, dummy_data)?; 
    let return_value = generic_resolve_wants(wants, fields_with_values_from_db, args, resolver_map, context);
    return Ok(return_value)
}

/// Takes in fields with values from database and returns the resolved fields once they have been processed
/// if wants is none, this means we are handling a single field -> return the value from the database
/// if wants is some, we are handling an object projection or a match 
/// for each field in wants, match its type
/// if single field, insert the value into resolved_field HashMap with the field name as the key
/// if object projection, call a new resolver using the field name as the key for the resolver in the resolver map
///     (this resolver will call this function recursively and will return the resolved inner object)
/// if match, determine which condition is correct and then resolve the inner object
/// return the resolved fields wrapped in Value Enum 
fn generic_resolve_wants<C, R> (
    wants: Option<&Wants>, 
    fields_with_values_from_db: HashMap<Box<str>, Value<R>>,
    args: &Args, 
    resolver_map: &ResolverMap<C, R>, 
    context: &C
) -> Value<R> {
    let wants = wants.unwrap();
    let mut resolved_fields= HashMap::new();
    for (identifier, value) in fields_with_values_from_db {
        let current_want = wants.get(&identifier).unwrap();
        generic_resolve_current_want(current_want, &mut resolved_fields, identifier.clone(), value, args, resolver_map, context);
    }
    let return_value = Value::Object(resolved_fields);
    return return_value
}

fn generic_resolve_current_want<C, R> (
    current_want: &Want,
    resolved_fields: &mut HashMap<Box<str>, Value<R>>,
    identifier: Box<str>,
    value: Value<R>,
    args: &Args, 
    resolver_map: &ResolverMap<C, R>, 
    context: &C
) {
    match current_want {
        Want::SingleField(_) => { 
            resolved_fields.insert(identifier.into(), value); 
        },
        Want::ObjectProjection(fields, args) => {
            //needs to call resolver to resolve want
            let inner_resolver = resolver_map.resolvers.get(&identifier).unwrap();
            let context = context;
            let inner_return_value = inner_resolver(Some(fields), args, resolver_map, context);
            resolved_fields.insert(identifier.into(), inner_return_value);
        },
        Want::Match(_) => {} //ignore match for now
    }
}

