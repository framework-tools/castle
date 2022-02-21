use std::collections::{HashMap, HashSet};

use parser_and_schema::ast::syntax_definitions::{want::{Wants, Want}, match_statement::MatchStatement};
use shared::castle_error::CastleError;

use crate::castle_object::resolver_return_types::{Value};

use super::{dummy_data_for_tests::{get_requested_fields_from_db_dummy}, resolver_type::{Args, Resolver}, resolver_map::ResolverMap};

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
    let mut fields_with_values_from_db: HashMap<Box<str>, Value<R>> = get_requested_fields_from_db_dummy(possible_fields, wants, args, context, dummy_data)?; 
    let return_value = generic_resolve_wants(wants, &mut fields_with_values_from_db, args, resolver_map, context);
    return return_value
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
    fields_with_values_from_db: &mut HashMap<Box<str>, Value<R>>,
    args: &Args, 
    resolver_map: &ResolverMap<C, R>, 
    context: &C
) -> Result<Value<R>, CastleError> {
    if wants.is_none() { return Ok(Value::Empty) }
    let wants = wants.unwrap();
    let mut resolved_fields= HashMap::new();
    for (identifier, current_want) in wants {
        let current_value = fields_with_values_from_db.remove(identifier);
        let match_statement_identifier = generic_resolve_current_want(current_want, &mut resolved_fields, identifier.clone(), current_value, args, resolver_map, context)?;
        if match_statement_identifier.is_some() {
            let identifier = match_statement_identifier.unwrap();
            let (identifier, unwrapped_want) = unwrap_and_remove_want_from_top_level_match(identifier, &mut resolved_fields)?;
            resolved_fields.insert(identifier, unwrapped_want);
        }
    }
    let return_value = Value::Object(resolved_fields);
    return Ok(return_value)
}

fn generic_resolve_current_want<C, R> (
    current_want: &Want,
    resolved_fields: &mut HashMap<Box<str>, Value<R>>,
    identifier: Box<str>,
    value: Option<Value<R>>,
    args: &Args, 
    resolver_map: &ResolverMap<C, R>, 
    context: &C
) -> Result<Option<Box<str>>, CastleError>{
    match current_want {
        Want::SingleField(_) => {
            insert_resolved_value_for_single_field(resolved_fields, identifier, value)?;
            return Ok(None)
        },
        Want::ObjectProjection(fields, args) => {
            resolve_inner_object_and_insert_fields(resolved_fields, identifier, fields, args, resolver_map, context)?;
            return Ok(None)
        },
        Want::Match(match_statement) => {
            let name = identifier.clone();
            resolve_match_and_insert_fields(match_statement, resolved_fields, identifier, value, args, resolver_map, context)?;
            return Ok(Some(name))
        }
    }
}

fn insert_resolved_value_for_single_field<R> (
    resolved_fields: &mut HashMap<Box<str>, Value<R>>,
    identifier: Box<str>,
    value: Option<Value<R>>
) -> Result<(), CastleError> {
    if value.is_none() {
        return Err(CastleError::DataForWantNotReturnedByDatabase(format!("1. No value found for field in database. identifier {:?}", identifier).into()))
    } else {
        resolved_fields.insert(identifier, value.unwrap());
        return Ok(())
    }
}

fn resolve_inner_object_and_insert_fields<C, R> (
    resolved_fields: &mut HashMap<Box<str>, Value<R>>,
    identifier: Box<str>,
    fields: &HashMap<Box<str>, Want>,
    args: &Args, 
    resolver_map: &ResolverMap<C, R>, 
    context: &C
) -> Result<(), CastleError>{
    //needs to call resolver to resolve want
    println!("identifier: {:?}", identifier);
    let inner_resolver = resolver_map.resolvers.get(&identifier).unwrap();
    let inner_return_value = inner_resolver(Some(fields), args, resolver_map, context)?;
    resolved_fields.insert(identifier.into(), inner_return_value);
    Ok(())
}

fn resolve_match_and_insert_fields<C, R> (
    match_statement: &MatchStatement,
    resolved_fields: &mut HashMap<Box<str>, Value<R>>,
    identifier: Box<str>,
    value: Option<Value<R>>,
    args: &Args, 
    resolver_map: &ResolverMap<C, R>, 
    context: &C
) -> Result<(), CastleError>{
    println!("identifier {:?}", identifier);
    if value.is_none() {
        return Err(CastleError::DataForWantNotReturnedByDatabase(format!("2. No value found for field in database. identifier {:?}", identifier).into()))
    } else {
        let value = value.unwrap();
        match_condition_insert_resolved_fields(value, match_statement, resolved_fields, identifier, args, resolver_map, context)?;
        return Ok(())
    }
}

fn match_condition_insert_resolved_fields<C, R>(
    value: Value<R>, 
    match_statement: &MatchStatement, 
    resolved_fields: &mut HashMap<Box<str>, Value<R>>,
    identifier: Box<str>,
    args: &Args,
    resolver_map: &ResolverMap<C, R>,
    context: &C
) -> Result<(), CastleError>{
    match value {
        Value::EnumValue(enum_value_from_db) => {
            for arm in match_statement {
                if arm.condition.identifier == enum_value_from_db.identifier {
                    let match_obj_identifier = arm.object_identifier.clone();
                    let fields = match &arm.object {
                        Want::ObjectProjection(fields, .. ) => fields,
                        _ => return Err(CastleError::InvalidMatchStatement(format!("3. Match statement should contain an object. identifier {:?}", identifier).into()))
                    };
                    resolve_inner_object_and_insert_fields(resolved_fields, match_obj_identifier, fields, args, resolver_map, context)?;
                    break;
                }
            }
            return Ok(())
        },
        _ => return Err(CastleError::DataForWantNotReturnedByDatabase(format!("3. No value found for Enum in database. identifier {:?}", identifier).into()))
    }
}

fn unwrap_and_remove_want_from_top_level_match<R>(identifier: Box<str>, resolved_fields: &mut HashMap<Box<str>, Value<R>>) -> Result<(Box<str>, Value<R>), CastleError> {
    let mut unwrapped_want = resolved_fields.remove(&identifier).unwrap();
    return match unwrapped_want {
        Value::Object(mut fields) => {
            let unwrapped_want = fields.remove(&identifier).unwrap();
            match &unwrapped_want {
                Value::Object(fields) => {
                    for key in fields.keys() {
                        return Ok((key.clone(), unwrapped_want))
                    }
                },
                _ => return Err(CastleError::InvalidMatchStatement(format!("4. Match statement should contain an object. identifier {:?}", identifier).into()))
            }
            Ok((identifier, unwrapped_want))
        },
        _ => Err(CastleError::InvalidMatchStatement(format!("4. Match statement should contain an object. identifier {:?}", identifier).into()))
    }
}