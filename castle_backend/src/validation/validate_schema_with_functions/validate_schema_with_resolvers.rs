use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::{schema_definition::SchemaDefinition, directive_definition::{DirectiveDefinition}};
use shared::CastleError;

use crate::resolvers::resolvers::Resolver;



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
/// 
/// 

pub fn validate_schema_with_resolvers_and_directives<C, O>(
    parsed_schema: &SchemaDefinition,
    resolvers: HashMap<Box<str>, Resolver<C, O>>,
    directives: HashMap<Box<str>, DirectiveDefinition>
) -> Result<(), CastleError> {
    validate_schema_with_resolvers(resolvers, parsed_schema)?;
    validate_schema_with_directives(directives, parsed_schema)?;
    return Ok(())
}

/// Steps For valiate_schema_with_resolvers():
/// - Generate resolvers
/// - For each fn in schema.functions
///     - Check if fn has a resolver by using the identifier of the fn
///     - If hashmap returns None for the identifier, throw error
///     - Else, unwrap the resolver and continue
///     - For the resolver, check the fn definition in schema & fn definition in resolvers is identical
///     - Else throw error
pub fn validate_schema_with_resolvers<C, O>(resolvers: HashMap<Box<str>, Resolver<C, O>>, parsed_schema: &SchemaDefinition ) -> Result<(), CastleError> {
    for resolver_in_schema in parsed_schema.functions.values() {
        let resolver = resolvers.get(&resolver_in_schema.name);
        if resolver.is_none() {
            return Err(CastleError::UndefinedResolver("Resolver not found for function".into()))
        } else {
            let resolver = resolver.unwrap();
            if &resolver.resolver_definition != resolver_in_schema {
                return Err(CastleError::ResolverDoesNotMatchSchemaFunction("Resolver definition does not match function definition".into()))
            }
        }
    }
    return Ok(())
}

/// Steps For validate_schema_with_directives():    
/// - For each field in schema.fields
///    - Check if field has a directive by using the identifier of the field
///   - If hashmap returns None for the identifier, throw error
///   - Else, unwrap the directive and continue
///     - For the directives, check the fn definition in schema & directive definition in resolvers is identical
///     - Else throw error
///   - If no errors, return Ok(())
pub fn validate_schema_with_directives(directives: HashMap<Box<str>, DirectiveDefinition>, parsed_schema: &SchemaDefinition) -> Result<(), CastleError> {
    for directive_in_schema in parsed_schema.directives.values() {
        let directive = directives.get(&directive_in_schema.function.name);
        if directive.is_none() {
            return Err(CastleError::UndefinedDirective("Directive not found for field".into()))
        } else {
            let directive = directive.unwrap();
            if directive != directive_in_schema {
                return Err(CastleError::DirectiveDoesNotMatchSchemaDirective("Directive definition does not match field definition".into()))
            }
        }
    }
    return Ok(())
}