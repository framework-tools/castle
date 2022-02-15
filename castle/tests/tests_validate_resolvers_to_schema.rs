use std::collections::{HashMap, HashSet};

use castle::{validation::validate_schema_with_functions::validate_schema_with_resolvers::{validate_schema_with_resolvers, validate_schema_with_directives}, resolvers::resolvers::{Resolver, ResolverInfo}, directives::directives::{Wants, Args}};
use parser_and_schema::{parsers::schema_parser::{parse_schema::parse_schema, types::{type_system::Type, primitive_type::PrimitiveType}}, ast::syntax_definitions::{fn_definition::FnDefinition, argument::{ArgumentOrTuple, IdentifierAndTypeArgument, IdentifierAndValueArgument}, directive_definition::{DirectiveDefinition, DirectiveOnValue, }}};
use shared::CastleError;

#[cfg(test)]
#[test]
fn test_resolver_defined_in_schema_that_does_not_exist_throws_error(){
    use std::collections::HashSet;

    use castle::resolvers::resolvers::{ResolverMap, Args};
    use parser_and_schema::ast::syntax_definitions::argument::IdentifierAndTypeArgument;

    let schema = "
    fn foo(id: Int) -> Int
    fn me (name: String) -> String 
    ";

    let parsed_schema = parse_schema(schema).unwrap();
    fn random_resolver(wants: &Option<Wants>, args: &Args, context: &()) -> Result<String, CastleError> {
        Ok("hello".to_string())
    }
    let random_resolver_definition = FnDefinition {
        args: HashMap::new(),
        name: "random_resolver".into(),
        return_type: Type::PrimitiveType(PrimitiveType::String),
    };
    let random_resolver_info = ResolverInfo {
        resolver: random_resolver,
        resolver_definition: random_resolver_definition,
    };
    let mut resolver_map= HashMap::new();
    resolver_map.insert("random_resolver".into(), random_resolver_info);
    let result = validate_schema_with_resolvers(&resolver_map, &parsed_schema);
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


    let name_arg = ("name".into(), Type::PrimitiveType(PrimitiveType::String));
    let anything_arg = ("anything".into(), Type::PrimitiveType(PrimitiveType::String));
    let mut arguments = HashMap::new();
    arguments.insert("name".into(), name_arg);
    arguments.insert("anything".into(), anything_arg);
    let function_definition = FnDefinition::new("me".into(), arguments, Type::PrimitiveType(PrimitiveType::String));

    fn me(wants: &Option<Wants>, args: &HashMap<Box<str>, IdentifierAndValueArgument>, context: &()) -> Result<String, CastleError> {
        Ok("".to_string())
    }
    let resolver = ResolverInfo::new(function_definition, me);
    let mut resolvers = HashMap::new();
    resolvers.insert("me".into(), resolver);
    let result = validate_schema_with_resolvers(&resolvers, &parsed_schema);

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

