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
        generic_resolve_current_want(current_want, fields_with_values_from_db, &mut resolved_fields, identifier.clone(), args, resolver_map, context)?;
    }
    let return_value = Value::Object(resolved_fields);
    return Ok(return_value)
}

fn generic_resolve_current_want<C, R> (
    current_want: &Want,
    fields_with_values_from_db: &mut HashMap<Box<str>, Value<R>>,
    resolved_fields: &mut HashMap<Box<str>, Value<R>>,
    identifier: Box<str>,
    args: &Args, 
    resolver_map: &ResolverMap<C, R>, 
    context: &C
) -> Result<(), CastleError>{
    let identifier_clone = identifier.clone();
    match current_want {
        Want::SingleField(_) => {
            let value = fields_with_values_from_db.remove(&identifier);
            insert_resolved_value_for_single_field(resolved_fields, identifier, value)?;
        },
        Want::ObjectProjection(fields, args) => {
            resolve_inner_object_and_insert_fields(resolved_fields, identifier.clone(), identifier, fields, args, resolver_map, context)?;
        },
        Want::Match(match_statement) => {
            resolve_match_and_insert_fields(match_statement, fields_with_values_from_db, resolved_fields, identifier, args, resolver_map, context)?;
        }
    }
    return Ok(())
}

fn insert_resolved_value_for_single_field<R> (
    resolved_fields: &mut HashMap<Box<str>, Value<R>>,
    identifier: Box<str>,
    value: Option<Value<R>>
) -> Result<(), CastleError> {
    if value.is_none() {
        resolved_fields.insert(identifier, Value::Empty);
    } else {
        resolved_fields.insert(identifier, value.unwrap());
    }
    return Ok(())
}

fn resolve_inner_object_and_insert_fields<C, R> (
    resolved_fields: &mut HashMap<Box<str>, Value<R>>,
    identifier_obj: Box<str>,
    resolver_ident: Box<str>,
    fields: &HashMap<Box<str>, Want>,
    args: &Args, 
    resolver_map: &ResolverMap<C, R>, 
    context: &C
) -> Result<(), CastleError>{
    //needs to call resolver to resolve want
    let inner_resolver = resolver_map.resolvers.get(&resolver_ident);
    if inner_resolver.is_none() {
        return Err(CastleError::QueryResolverNotDefinedInSchema(format!("2. No resolver found for identifier: {:?}", resolver_ident).into()))
    } else {
        let inner_return_value = inner_resolver.unwrap()(Some(fields), args, resolver_map, context)?;
        resolved_fields.insert(identifier_obj.into(), inner_return_value);
        Ok(())
    }
}

fn resolve_match_and_insert_fields<C, R> (
    match_statement: &MatchStatement,
    fields_with_values_from_db: &mut HashMap<Box<str>, Value<R>>,
    resolved_fields: &mut HashMap<Box<str>, Value<R>>,
    identifier: Box<str>,
    args: &Args, 
    resolver_map: &ResolverMap<C, R>, 
    context: &C
) -> Result<(), CastleError>{
    println!("identifier: {:?}", identifier);
    if fields_with_values_from_db.len() == 0 {
        Err(CastleError::DataForWantNotReturnedByDatabase(format!("3. No fields found for match statement. identifier {:?}", identifier).into()))
    } else {
        let mut enum_key: Box<str>= "".into();
        for key in fields_with_values_from_db.keys() {
            enum_key = key.clone();
        }
        let enum_value = fields_with_values_from_db.remove(&enum_key).unwrap();
        match_condition_insert_resolved_fields(enum_value, match_statement, resolved_fields, identifier, args, resolver_map, context)?;
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
) -> Result<(), CastleError> {
    match value {
        Value::EnumValue(enum_value_from_db) => {
            for arm in match_statement {
                if arm.condition.identifier == enum_value_from_db.identifier {
                    let fields = match &arm.object {
                        Want::ObjectProjection(fields, .. ) => fields,
                        _ => return Err(CastleError::InvalidMatchStatement(format!("3. Match statement should contain an object. identifier {:?}", identifier).into()))
                    };
                    let inner_resolver = resolver_map.resolvers.get(&arm.object_identifier);
                    let inner_return_value = inner_resolver.unwrap()(Some(fields), args, resolver_map, context)?;
                    resolved_fields.insert(identifier.clone(), inner_return_value);
                    break;
                }
            }
            return Ok(())
        },
        _ => return Err(CastleError::DataForWantNotReturnedByDatabase(format!("3. No value found for Enum in database. identifier {:?}", identifier).into()))
    }
}

//use identifier to find object in database
//match want to object else throw error
//return inner object
pub fn unwrap_outer_wrapper<R>(resolved_fields: Value<R>, identifier: Box<str>) -> Result<Value<R>, CastleError> {
    let inner_value;
    match resolved_fields {
        Value::Object(mut inner_obj) => {
            inner_value = inner_obj.remove(&identifier).unwrap();
            return Ok(inner_value)
        },
        _ => return Err(CastleError::InvalidMatchStatement(format!("3. Match statement should contain an object. identifier {:?}", identifier).into()))
    }
}