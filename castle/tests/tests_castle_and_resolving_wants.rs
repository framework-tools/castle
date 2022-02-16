use std::collections::HashMap;

use castle::{validation::validate_query_with_schema::validate_query_with_schema::validate_query_with_schema, resolvers::resolvers::resolve_all_wants};
use parser_and_schema::{ast::syntax_definitions::argument::IdentifierAndValueArgument, parsers::query_parser::parse_query::parse_query};
use shared::CastleError;

// enum IconTypes {
//     Svg {
//         url: String,
//         size: u32,
//     },
//     Emoji {
//         unicode: String,
//     },
// }

// pub enum Value<C> {
//     Null,
//     Bool(bool),
//     Int(i64),
//     UInt(u64),
//     Float(f64),
//     String(String),
//     EnumValue(EnumValue),
//     List(Vec<Value>),
//     Object(HashMap<String, Value>),
//     Custom(Box<C>),
// }






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


