use std::collections::{HashMap, HashSet};

use castle::{validation::validate_backend_fns_with_schema::validate_backend_fns_with_schema::{validate_schema_with_resolvers, validate_schema_with_directives}, resolvers::resolvers::{Resolver, Args}, directives::directives::{Wants}, castle_struct::{castle_struct::Castle, resolver_return_types::ReturnValue}};
use parser_and_schema::{parsers::schema_parser::{parse_schema::parse_schema, types::{type_system::Type, primitive_type::PrimitiveType}}, ast::syntax_definitions::{fn_definition::FnDefinition, argument::{ArgumentOrTuple, IdentifierAndTypeArgument, IdentifierAndValueArgument}, directive_definition::{DirectiveDefinition, DirectiveOnValue, }}};
use shared::CastleError;

/// Currently Testing:
/// - Breaks if resolver defined in schema is not in the resolver map
/// - Breaks if directive defined in schema does not exist in directive map

#[cfg(test)]
#[test]
fn test_resolver_defined_in_schema_that_does_not_exist_throws_error(){
    use std::collections::HashSet;

    use castle::{resolvers::resolvers::{ResolverMap, Args}, castle_struct::castle_struct::Castle};
    use parser_and_schema::ast::syntax_definitions::argument::IdentifierAndTypeArgument;

    let schema = "
    fn foo(id: Int) -> Int
    fn me (name: String) -> String 
    ";

    let parsed_schema = parse_schema(schema).unwrap();
    fn random_resolver(wants: &Option<Wants>, args: &Args, context: &()) -> ReturnValue  {
        ReturnValue::String("hello".to_string())
    }
    let mut builder = Castle::builder();
    builder.add_resolver("random_resolver".into(), random_resolver);
    let result = validate_schema_with_resolvers(&builder.resolvers, &parsed_schema);
    if result.is_err() {
        match result {
            Err(CastleError::UndefinedResolver(_)) => {}, //passes
            _ => panic!("Expected error to be of type UndefinedResolver, found: {:?}", result),
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
    


    fn random_directive(wants: &Option<Wants>, args: &Args, context: &()) -> ReturnValue {
        ReturnValue::String("hello".to_string())
    }

    let parsed_schema = parse_schema(schema).unwrap();

    let mut builder = Castle::builder();
    builder.add_directive("random_directive".into(), random_directive);

    let result = validate_schema_with_directives(&builder.directives, &parsed_schema);
    
    if result.is_err() {
        match result {
            Err(CastleError::UndefinedDirective(_)) => {}, //passes
            _ => panic!("Expected error to be of type UndefinedDirective, found: {:?}", result),
        }
    } else {
        panic!("No error thrown");
    }
}


