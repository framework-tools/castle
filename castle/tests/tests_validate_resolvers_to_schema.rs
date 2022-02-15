use std::collections::{HashMap, HashSet};

use castle::{validation::validate_schema_with_functions::validate_schema_with_resolvers::{validate_schema_with_resolvers, validate_schema_with_directives}, resolvers::resolvers::Resolver};
use parser_and_schema::{parsers::schema_parser::{parse_schema::parse_schema, types::{type_system::Type, primitive_type::PrimitiveType}}, ast::syntax_definitions::{fn_definition::FnDefinition, argument::ArgumentOrTuple, directive_definition::{DirectiveDefinition, DirectiveOnValue, }}};
use shared::CastleError;

#[cfg(test)]
#[test]
fn test_resolver_defined_in_schema_that_does_not_exist_throws_error(){
    use std::collections::HashSet;

    use castle::resolvers::resolvers::ResolverMap;
    use parser_and_schema::ast::syntax_definitions::argument::IdentifierAndTypeArgument;

    let schema = "
    fn foo(id: Int) -> Int
    fn me (name: String) -> String 
    ";

    let parsed_schema = parse_schema(schema).unwrap();
    let resolver_map = HashSet::new();
    let result = validate_schema_with_resolvers(resolver_map, &parsed_schema);
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
fn test_directive_defined_in_schema_that_does_not_exist_throw_error(){
    let schema = "
    directive @test(arg: String) on FIELD  
    ";

    let parsed_schema = parse_schema(schema).unwrap();

    let directives = HashSet::new();
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