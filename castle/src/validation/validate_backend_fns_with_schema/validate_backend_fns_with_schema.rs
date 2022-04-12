use std::{collections::HashMap};
use parser_and_schema::ast::syntax_definitions::{schema_definition::SchemaDefinition};
use shared::castle_error::CastleError;
use crate::{resolvers::{resolver_map::ResolverMap}, directives::{directives::DirectiveMap}};

/// Need to check:
/// - Every function in schema has a resolver
/// - Every fields' directives have a resolver
pub(crate) fn validate_schema_with_resolvers_and_directives<C, R>(
    schema: &SchemaDefinition,
    resolvers: &ResolverMap<C, R>,
    directives: &DirectiveMap<C, R>,
) -> Result<(), CastleError> {
    validate_functions_are_defined_in_schema(
        &schema.functions, 
        &resolvers.resolvers,
        true
    )?; 
    validate_functions_are_defined_in_schema(
        &schema.directives, 
        directives,
        true
    )?; 
    return Ok(())
}

pub fn validate_functions_are_defined_in_schema<T, U>(
    definitions: &HashMap<Box<str>, T>, 
    fns: &HashMap<Box<str>, U>, 
    for_resolver: bool
) -> Result<Vec<()>, CastleError> {
    let result: Result<Vec<()>, CastleError> = fns.into_iter()
        .map(|(key, _)| validate_fn_is_defined_in_schema(key, definitions, for_resolver))
        .collect();
    return result
}

pub(crate) fn validate_fn_is_defined_in_schema<T>(
    key: &Box<str>,
    definitions: &HashMap<Box<str>, T>, 
    for_resolver: bool
) -> Result<(), CastleError> {
    return match definitions.contains_key(key) {
        true => Ok(()),
        false if for_resolver => Err(CastleError::UndefinedResolver(format!("Resolver not found for fn definition in schema: {}", key).into())), 
        _ => Err(CastleError::UndefinedDirective(format!("Directive not found for directive definition in schema: {}", key).into())), 
    }
}

