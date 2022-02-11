use castle_backend::validation::validate_query_with_schema::validate_query_with_schema::validate_query_with_schema;
use parser_and_schema::{ast::syntax_definitions::schema_definition::SchemaDefinition, parsers::schema_parser::parse_schema::parse_schema};
use shared::CastleError;



#[test]
fn if_object_projection_identifier_is_not_defined_as_function_in_schema_should_throw_error() -> Result<(), CastleError>{
    let schema = "
    fn lolololol(id: Int) -> User

    type User {
            amount: id,
            currency: String,
        }
    ";

    let query = "
    me() {
        first_name,
        last_name
    }
    ";

    let schema_definition = parse_schema(schema)?;
    let parsed_query = parse_query(query)?;
    let result = validate_query_with_schema(parsed_query, schema_definition);
    if result.is_err() {
        match result {
            Err(CastleError::QueryResolverNotDefinedInSchema(message)) => {
                assert_eq!(message, "no type found for want".into());
            },
            _ => {
                panic!("threw wrong error: {}", result);
            }
        }
    } else {
        panic!("should have thrown error");
    }
}