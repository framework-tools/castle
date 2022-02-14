use std::collections::HashMap;

use castle_backend::{validation::validate_schema_with_functions::validate_schema_with_resolvers::{validate_schema_with_resolvers, validate_schema_with_directives}, resolvers::resolvers::Resolver};
use parser_and_schema::{parsers::schema_parser::{parse_schema::parse_schema, types::{type_system::Type, primitive_type::PrimitiveType}}, ast::syntax_definitions::{fn_definition::FnDefinition, argument::ArgumentOrTuple, directive_definition::{DirectiveDefinition, DirectiveOnValue, }}};
use shared::CastleError;

#[cfg(test)]
#[test]
fn test_resolver_defined_in_schema_that_does_not_exist_throws_error(){
    let schema = "
    fn foo(id: Int) -> Int
    fn me (name: String) -> String 
    ";

    let parsed_schema = parse_schema(schema).unwrap();

    let fn_definition_foo = FnDefinition {
        name: "foo".into(),
        args: Some(vec![
            Argument::IdentifierAndType("id".into(), Type::PrimitiveType(PrimitiveType::Int))
        ]),
        return_type: Some(Type::PrimitiveType(PrimitiveType::Int))
    };
    let foo_resolver = Resolver { 
        resolver_definition: fn_definition_foo
    };
    let mut resolvers = HashMap::new();
    resolvers.insert("foo".into(), foo_resolver);
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

#[test]
fn test_directive_defined_in_schema_that_has_different_arguments(){
    let schema = "
    directive @test(arg: String) on FIELD  
    ";

    let parsed_schema = parse_schema(schema).unwrap();

    
    // missing anything argument
    let directive_definition = FnDefinition::new("test".into(), Some(vec![
        Argument::IdentifierAndType("arg".into(), Type::PrimitiveType(PrimitiveType::Int)), // argument is different
    ]), 
    Some(Type::PrimitiveType(PrimitiveType::String)));
    

    let directive = DirectiveDefinition::new(directive_definition, DirectiveOnValue::Field);
    let mut directives = HashMap::new();
    directives.insert("test".into(), directive);
    let result = validate_schema_with_directives(directives, &parsed_schema);
    
    if result.is_err() {
        match result {
            Err(CastleError::DirectiveDoesNotMatchSchemaDirective(_)) => {}, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", result),
        }
    } else {
        panic!("No error thrown");
    }
}