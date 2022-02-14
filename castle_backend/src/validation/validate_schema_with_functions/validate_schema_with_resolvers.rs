use std::collections::{HashMap, HashSet};

use parser_and_schema::ast::syntax_definitions::{schema_definition::SchemaDefinition, directive_definition::{DirectiveDefinition}};
use shared::CastleError;

use crate::{resolvers::resolvers::{Resolver, ResolverMap}, directives::directives::DirectiveMap};



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

pub fn validate_schema_with_resolvers_and_directives<C, T>(
    parsed_schema: &SchemaDefinition,
    resolvers: &ResolverMap<C, T>,
    directives: &DirectiveMap<C, T>,
) -> Result<(), CastleError> {
    let resolvers_identifiers = get_resolvers_identifiers(&resolvers);
    let directives_identifiers = get_directives_identifiers(&directives);
    validate_schema_with_resolvers(resolvers_identifiers, parsed_schema)?;
    validate_schema_with_directives(directives_identifiers, parsed_schema)?;
    return Ok(())
}

fn get_resolvers_identifiers<C, T>(resolver: &ResolverMap<C, T>) -> HashSet<&Box<str>>{
    let mut resolvers_identifiers = HashSet::new();
    for (resolver_identifier, _) in resolver {
        resolvers_identifiers.insert(resolver_identifier);
    }
    return resolvers_identifiers
}

fn get_directives_identifiers<C, T>(directives: &DirectiveMap<C, T>) -> HashSet<&Box<str>>{
    let mut directives_identifiers = HashSet::new();
    for (directive_identifier, _) in directives {
        directives_identifiers.insert(directive_identifier);
    }
    return directives_identifiers
}

/// Steps For valiate_schema_with_resolvers():
/// - Generate resolvers
/// - For each fn in schema.functions
///     - Check if fn has a resolver by using the identifier of the fn
///     - If hashmap returns None for the identifier, throw error
///     - Else, unwrap the resolver and continue
///     - For the resolver, check the fn definition in schema & fn definition in resolvers is identical
///     - Else throw error
pub fn validate_schema_with_resolvers(resolvers_identifiers: HashSet<&Box<str>>, parsed_schema: &SchemaDefinition ) -> Result<(), CastleError> {
    for resolver_in_schema in parsed_schema.functions.values() {
        let resolver = resolvers_identifiers.get(&resolver_in_schema.name);
        if resolver.is_none() {
            return Err(CastleError::UndefinedResolver("Resolver not found for function".into()))
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
pub fn validate_schema_with_directives(directives_identifiers: HashSet<&Box<str>>, parsed_schema: &SchemaDefinition) -> Result<(), CastleError> {
    for directive_in_schema in parsed_schema.directives.values() {
        let directive = directives_identifiers.get(&directive_in_schema.function.name);
        if directive.is_none() {
            return Err(CastleError::UndefinedDirective("Directive not found for field".into()))
        }
    }
    return Ok(())
}