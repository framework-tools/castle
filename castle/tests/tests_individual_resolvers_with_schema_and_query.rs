use castle::{castle_object::castle_struct::{Castle, CastleBuilder}, validation::validate_query_with_schema::validate_query_with_schema::validate_query_with_schema, resolvers::resolve_query_wants::resolve_all_wants};
use parser_and_schema::parsers::query_parser::parse_query::parse_query;
use shared::CastleError;


//Should be a test for each resolver
// These tests show how the resolver will work
// As well as creating the query that will be used
// for every specific case from the front end

#[test]
fn test_page_info_resolvers() -> Result<(), CastleError> {
    let mut builder: CastleBuilder<(), ()> = CastleBuilder::new();
    builder.apply_current_schema();
    builder.add_all_resolvers();

    //parses schema, validates schema with itself and resolvers, then builds castle
    let castle = builder.build_and_validate()?;

    let query = "
        page_info() {
            id,
            basic_page_info() {
                title,
                icon,
                emoji,
            },
            description,
            parent_id,
            basic_parent_page_info() {
                title,
                icon,
                emoji,
            },
            blocks
        }
    ";

    let parsed_query = parse_query(query)?;
    validate_query_with_schema(&parsed_query, &castle.parsed_schema)?;

    let actual = resolve_all_wants(parsed_query.wants, &castle.resolver_map, ())?;
    
    let expected
    return Ok(())
}

let mut block = EnumResolverValue {
    identifier: "Block::ContentBlock".into(),
    enum_parent: "Block".into(),
    variant: "ContentBlock".into(),
    fields: HashMap::new(),
};
block.fields.insert("uuid".into(), Value::Uuid(Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap()));

let (_, dummy_data): (HashSet<Box<str>>, Value<R>) = create_possible_fields_and_dummy_data(vec![
    ("id".into(), Value::Uuid(Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())),
    ("description".into(), Value::String("this is a description".to_string())),
    ("parent_id".into(), Value::Uuid(Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())),
    ("blocks".into(), Value::Vec(vec![
        Value::EnumValue(block)
    ])),
