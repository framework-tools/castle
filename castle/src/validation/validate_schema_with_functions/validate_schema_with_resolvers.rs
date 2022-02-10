use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::{schema_definition::SchemaDefinition, directive_definition::DirectiveDefinition};
use shared::CastleError;

use crate::{resolvers::resolvers::{Resolver, generate_resolvers}, directives::directives::Directive};

/// Cross-Validation
///  For Functions:
///  - Resolvers
///  - Directives
/// 
/// Every resolver correlates to a function (inside Castle)
/// 
/// pub struct Resolver {
///      pub resolver_definition: FnDefinition,
///      unsure about field below currently
///      pub function: fn(args: &HashMap<String, String>) -> Result<String, CastleError>,
///  }
/// 
/// Resolvers is: HashMap<Box<str>, Resolver>
/// Need to check:
/// - Every function in schema has a resolver
/// - Every fields' directives have a resolver
/// 
/// Steps For valiate_schema_with_resolvers():
/// - Generate resolvers
/// - For each fn in schema.functions
///     - Check if fn has a resolver by using the identifier of the fn
///     - If hashmap returns None for the identifier, throw error
///     - Else, unwrap the resolver and continue
///     - Check that the resolver's args match the fn's args
///     - If not, throw error
///     - Check that the resolver's return type is equal to the fn's return type
/// 
/// Steps For validate_schema_with_directives():    
/// - For each field in schema.fields
///    - Check if field has a directive by using the identifier of the field
///   - If hashmap returns None for the identifier, throw error
///   - Else, unwrap the directive and continue
///   - Check that both directives arguments match
///   - Probably need to check return type but need to clarify this with Bert
///   - If no errors, return Ok(())
/// 


pub fn validate_schema_with_resolvers_and_directives(
    parsed_schema: &SchemaDefinition,
    resolvers: HashMap<Box<str>, Resolver>,
    directives: HashMap<Box<str>, Directive>
) -> Result<(), CastleError> {

    let resolvers: HashMap<Box<str>, Resolver> = generate_resolvers()?;
    valiate_schema_with_resolvers()?;
    validate_schema_with_directives()?;
    return Ok(())
}

fn valiate_schema_with_resolvers() -> Result<(), CastleError> {
    return Ok(())
}

fn validate_schema_with_directives() -> Result<(), CastleError> {
    return Ok(())
}