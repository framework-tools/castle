use std::collections::HashMap;

use castle_backend::{validation::validate_schema_with_functions::validate_schema_with_resolvers::{validate_schema_with_resolvers, validate_schema_with_directives}, resolvers::resolvers::Resolver};
use parser_and_schema::{parsers::schema_parser::{parse_schema::parse_schema, types::{type_system::Type, primitive_type::PrimitiveType}}, ast::syntax_definitions::{fn_definition::FnDefinition, argument::Argument}};
use shared::CastleError;

#[cfg(test)]
#[test]
fn test_resolver_defined_in_schema_that_does_not_exist_throw_error(){
    let schema = "
    fn me (name: String) -> String 
    ";

    let parsed_schema = parse_schema(schema).unwrap();

    let resolvers = HashMap::new();
    let result = validate_schema_with_resolvers(resolvers, &parsed_schema);
    
    if result.is_err() {
        match result {
            Err(CastleError::UndefinedResolver(_)) => {}, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", result),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn test_resolver_defined_in_schema_that_has_different_arguments(){
    let schema = "
    fn me (name: String, anything: String) -> String 
    ";

    let parsed_schema = parse_schema(schema).unwrap();

    
    // missing anything argument
    let function_definition = FnDefinition::new("me".into(), Some(vec![
        Argument::IdentifierAndType("name".into(), Type::PrimitiveType(PrimitiveType::String)),
    ]), 
    Some(Type::PrimitiveType(PrimitiveType::String)));

    let resolver = Resolver::new(function_definition);
    let mut resolvers = HashMap::new();
    resolvers.insert("me".into(), resolver);
    let result = validate_schema_with_resolvers(resolvers, &parsed_schema);
    
    if result.is_err() {
        match result {
            Err(CastleError::ResolverDoesNotMatchSchemaFunction(_)) => {}, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", result),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn test_resolver_defined_in_schema_that_has_different_return_type(){
    let schema = "
    fn me (name: String) -> String
    ";

    let parsed_schema = parse_schema(schema).unwrap();
    
    let function_definition = FnDefinition::new("me".into(), Some(vec![
        Argument::IdentifierAndType("name".into(), Type::PrimitiveType(PrimitiveType::String)),
    ]), 
    Some(Type::PrimitiveType(PrimitiveType::Int))); //return type is different

    let resolver = Resolver::new(function_definition);
    let mut resolvers = HashMap::new();
    resolvers.insert("me".into(), resolver);
    let result = validate_schema_with_resolvers(resolvers, &parsed_schema);
    
    if result.is_err() {
        match result {
            Err(CastleError::ResolverDoesNotMatchSchemaFunction(_)) => {}, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", result),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn test_directive_defined_in_schema_that_does_not_exist_throw_error(){
    let schema = "
    directive @test(arg: String) on FIELD  
    ";

    let parsed_schema = parse_schema(schema).unwrap();

    let directives = HashMap::new();
    let result = validate_schema_with_directives(directives, &parsed_schema);
    
    if result.is_err() {
        match result {
            Err(CastleError::UndefinedDirective(_)) => {}, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", result),
        }
    } else {
        panic!("No error thrown");
    }
}