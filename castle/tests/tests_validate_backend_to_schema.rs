use castle::validation::validate_backend_fns_with_schema::validate_backend_fns_with_schema::validate_functions_are_defined_in_schema;
use parser_and_schema::parsers::schema_parser::parse_schema::parse_schema;
use std::collections::HashMap;
use parser_and_schema::ast::syntax_definitions::want::Want;
use castle::{resolvers::resolver_type::Args};
use castle::castle_object::resolver_return_types::Value;
use shared::castle_error::CastleError;
use castle::{castle_object::castle_struct::CastleBuilder};

/// Currently Testing:
/// - Breaks if resolver defined in schema is not in the resolver map
/// - Breaks if directive defined in schema does not exist in directive map

#[cfg(test)]
#[test]
fn test_resolver_defined_in_schema_that_does_not_exist_throws_error(){
    use castle::validation::validate_backend_fns_with_schema::validate_backend_fns_with_schema::validate_functions_are_defined_in_schema;

    let schema = "
    fn foo(id: Int) -> Int
    fn me (name: String) -> String 
    ";

    let parsed_schema = parse_schema(schema).unwrap();
    fn random_resolver<C, R>(wants: Option<HashMap<Box<str>, Want>>, args: Args, context: C) -> Result<Value<R>, CastleError> {
        Ok(Value::String("hello".to_string()))
    }
    let mut builder: CastleBuilder<(), ()> = CastleBuilder::new();
    builder.add_resolver("random_resolver".into(), random_resolver);
    let result = validate_functions_are_defined_in_schema(
        &parsed_schema.functions,
        &builder.resolver_map.resolvers, 
        true
    );
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

    fn random_directive<C, R>(wants: Option<HashMap<Box<str>, Want>>, args: Args, context: C) -> Result<Value<R>, CastleError>  {
        Ok(Value::String("hello".to_string()))
    }

    let parsed_schema = parse_schema(schema).unwrap();

    let mut builder: CastleBuilder<(), ()> = CastleBuilder::new();
    builder.add_directive("random_directive".into(), random_directive);

    let result = validate_functions_are_defined_in_schema(
        &parsed_schema.directives,
        &builder.directives, 
        false
    );
    
    if result.is_err() {
        match result {
            Err(CastleError::UndefinedDirective(_)) => {}, //passes
            _ => panic!("Expected error to be of type UndefinedDirective, found: {:?}", result),
        }
    } else {
        panic!("No error thrown");
    }
}


