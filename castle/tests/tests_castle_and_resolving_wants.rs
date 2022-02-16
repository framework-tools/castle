use std::collections::HashMap;

use castle::{validation::validate_query_with_schema::validate_query_with_schema::validate_query_with_schema, resolvers::resolvers::resolve_all_wants};
use parser_and_schema::{ast::syntax_definitions::argument::IdentifierAndValueArgument, parsers::query_parser::parse_query::parse_query};
use shared::CastleError;









#[cfg(test)]
#[test]
fn testing_castle_builds_and_validates(){
    use castle::{castle_struct::castle_struct::{CastleBuilder, Castle}, resolvers::resolvers::{ResolverMap, Wants, Args}, directives::directives::DirectiveMap};
    use parser_and_schema::{ast::syntax_definitions::fn_definition::FnDefinition, parsers::schema_parser::types::{type_system::Type, primitive_type::PrimitiveType}};
    use shared::CastleError;

    let mut builder = Castle::builder();
    //test resolver
    fn hello(wants: &Option<Wants>, args: &Args, context: &()) -> String {
        "world".to_string()
    }


    builder.add_resolver("hello", hello);
    let schema = "
        fn hello() -> String
    ";

    let builder = builder.add_schema(schema);
    let castle = builder.build_and_validate();
    assert!(castle.is_ok());
}

#[test]
fn testing_castle_can_resolve_single_field_want() -> Result<(), CastleError> {
    use castle::{castle_struct::castle_struct::{CastleBuilder, Castle}, resolvers::resolvers::{ResolverMap, Wants, Args}, directives::directives::DirectiveMap};
    use parser_and_schema::{ast::syntax_definitions::fn_definition::FnDefinition, parsers::schema_parser::types::{type_system::Type, primitive_type::PrimitiveType}};
    use shared::CastleError;

    let mut builder = Castle::builder();
    //test resolver
    fn hello(wants: &Option<Wants>, args: &Args, context: &()) -> String {
        "world".to_string()
    }


    builder.add_resolver("hello", hello);
    let schema = "
        fn hello() -> String
    ";

    let builder = builder.add_schema(schema);
    let castle = builder.build_and_validate()?;

    let query = "
        hello()
    ";

    let parsed_query = parse_query(query)?;
    validate_query_with_schema(&parsed_query, &castle.parsed_schema)?;
    let resolved_wants = resolve_all_wants(parsed_query.wants, &castle.resolvers, ())?;
    let mut expected = HashMap::new();
    expected.insert("hello".into(), "world".to_string());
    assert_eq!(expected, resolved_wants);
    return Ok(());
}

#[test]
fn testing_castle_can_resolve_object_projection_want_with_all_fields() -> Result<(), CastleError> {
    use castle::{castle_struct::castle_struct::{CastleBuilder, Castle}, resolvers::resolvers::{ResolverMap, Wants, Args}, directives::directives::DirectiveMap};
    use parser_and_schema::{ast::syntax_definitions::fn_definition::FnDefinition, parsers::schema_parser::types::{type_system::Type, primitive_type::PrimitiveType}};
    use shared::CastleError;

    let mut builder = Castle::builder();
    //test resolver
    fn get_name(wants: &Option<Wants>, args: &Args, context: &()) -> HashMap<Box<str>, String> {
        //dummy data
        let first_name = ("first_name".into(), "John".to_string()); 
        let middle_name = ("middle_name".into(), "Graham".to_string());
        let last_name = ("last_name".into(), "Doe".to_string());
        let possible_fields = [first_name, middle_name, last_name];

        let wants = wants.as_ref().unwrap();
        let mut resolved_fields= HashMap::new();
        for (identifier, value) in possible_fields{
            if wants.contains_key(&identifier){
                resolved_fields.insert(identifier, value);
            }
        }
        return resolved_fields
    }


    builder.add_resolver("get_name", get_name);
    let schema = "
        fn get_name() -> Name

        type Name {
            first_name: String
            middle_name: String
            last_name: String
        }
    ";

    let builder = builder.add_schema(schema);
    let castle = builder.build_and_validate()?;

    let query = "
        get_name() {
            first_name,
            middle_name,
            last_name
        }
    ";

    let parsed_query = parse_query(query)?;
    validate_query_with_schema(&parsed_query, &castle.parsed_schema)?;
    let resolved_wants = resolve_all_wants(parsed_query.wants, &castle.resolvers, ())?;
    let mut expected_wants = HashMap::new();
    expected_wants.insert("first_name".into(), "John".to_string());
    expected_wants.insert("middle_name".into(), "Graham".to_string());
    expected_wants.insert("last_name".into(), "Doe".to_string());
    let mut expected = HashMap::new();
    expected.insert("get_name".into(), expected_wants);
    assert_eq!(expected, resolved_wants);
    return Ok(());
}

#[test]
fn testing_castle_can_resolve_object_projection_but_subset_of_fields() -> Result<(), CastleError> {
    use castle::{castle_struct::castle_struct::{CastleBuilder, Castle}, resolvers::resolvers::{ResolverMap, Wants, Args}, directives::directives::DirectiveMap};
    use parser_and_schema::{ast::syntax_definitions::fn_definition::FnDefinition, parsers::schema_parser::types::{type_system::Type, primitive_type::PrimitiveType}};
    use shared::CastleError;

    let mut builder = Castle::builder();
    //test resolver
    fn get_name(wants: &Option<Wants>, args: &Args, context: &()) -> HashMap<Box<str>, String> {
        //dummy data
        let first_name = ("first_name".into(), "John".to_string()); 
        let middle_name = ("middle_name".into(), "Graham".to_string());
        let last_name = ("last_name".into(), "Doe".to_string());
        let possible_fields = [first_name, middle_name, last_name];

        let wants = wants.as_ref().unwrap();
        let mut resolved_fields= HashMap::new();
        for (identifier, value) in possible_fields{
            if wants.contains_key(&identifier){
                resolved_fields.insert(identifier, value);
            }
        }
        return resolved_fields
    }


    builder.add_resolver("get_name", get_name);
    let schema = "
        fn get_name() -> Name

        type Name {
            first_name: String
            middle_name: String
            last_name: String
        }
    ";

    let builder = builder.add_schema(schema);
    let castle = builder.build_and_validate()?;

    let query = "
        get_name() {
            first_name
        }
    ";

    let parsed_query = parse_query(query)?;
    validate_query_with_schema(&parsed_query, &castle.parsed_schema)?;
    let resolved_wants = resolve_all_wants(parsed_query.wants, &castle.resolvers, ())?;
    let mut expected_wants = HashMap::new();
    expected_wants.insert("first_name".into(), "John".to_string());
    let mut expected = HashMap::new();
    expected.insert("get_name".into(), expected_wants);
    assert_eq!(expected, resolved_wants);
    return Ok(());
}

#[test]
fn testing_castle_can_resolve_two_single_fields_different_return_types() -> Result<(), CastleError> {
    use castle::{castle_struct::castle_struct::{CastleBuilder, Castle}, resolvers::resolvers::{ResolverMap, Wants, Args}, directives::directives::DirectiveMap};
    use parser_and_schema::{ast::syntax_definitions::fn_definition::FnDefinition, parsers::schema_parser::types::{type_system::Type, primitive_type::PrimitiveType}};
    use shared::CastleError;

    let mut builder = Castle::builder();
    //test resolver
    fn hello(wants: &Option<Wants>, args: &Args, context: &()) -> String {
        "world".to_string()
    }

    fn get_number(wants: &Option<Wants>, args: &Args, context: &()) -> i32 {
        42
    }

    builder.add_resolver("hello", hello);
    builder.add_resolver("get_number", get_number);
    let schema = "
        fn hello() -> String
    ";

    let builder = builder.add_schema(schema);
    let castle = builder.build_and_validate()?;

    let query = "
        hello()
    ";

    let parsed_query = parse_query(query)?;
    validate_query_with_schema(&parsed_query, &castle.parsed_schema)?;
    let resolved_wants = resolve_all_wants(parsed_query.wants, &castle.resolvers, ())?;
    let mut expected = HashMap::new();
    expected.insert("hello".into(), "world".to_string());
    assert_eq!(expected, resolved_wants);
    return Ok(());
}

