use castle_backend::validation::validate_query_with_schema::validate_query_with_schema::validate_query_with_schema;
use parser_and_schema::{parsers::{schema_parser::parse_schema::parse_schema, query_parser::parse_query::parse_query}};
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
    let result = validate_query_with_schema(&parsed_query, &schema_definition);
    if result.is_err() {
        match result {
            Err(CastleError::QueryResolverNotDefinedInSchema(_message)) => return Ok(()),
            _ => panic!("threw wrong error: {:?}", result)
        }
    } else {
        panic!("should have thrown error");
    }
}

#[test]
fn should_break_if_mismatched_arguments() -> Result<(), CastleError>{
    let schema = "
    fn me(id: Int) -> User
    type User {
            amount: Int,
            currency: String,
        }
    ";

    let query = "
    me(id: 432, currency: \"USD\") {
        first_name,
        last_name
    }
    ";

    let schema_definition = parse_schema(schema)?;
    let parsed_query = parse_query(query)?;
    let result = validate_query_with_schema(&parsed_query, &schema_definition);
    if result.is_err() {
        match result {
            Err(CastleError::ArgumentsInQueryDoNotMatchResolver(_message)) => return Ok(()),
            _ => panic!("threw wrong error: {:?}", result)
        }
    } else {
        panic!("should have thrown error");
    }
}

#[test]
fn should_break_if_mismatched_fields_in_return_type() -> Result<(), CastleError>{
    let schema = "
    fn me(id: Int) -> User
    
    type User {
        first_name: String,
        age: Int,
        role: String
        }
    ";

    let query = "
    me(id: 543) {
        first_name,
        last_name
    }
    ";

    let schema_definition = parse_schema(schema)?;
    let parsed_query = parse_query(query)?;
    let result = validate_query_with_schema(&parsed_query, &schema_definition);
    if result.is_err() {
        match result {
            Err(CastleError::ArgumentsInQueryDoNotMatchResolver(_message)) => return Ok(()),
            _ => panic!("threw wrong error: {:?}", result)
        }
    } else {
        panic!("should have thrown error");
    }
}


// test match statement compatability with schema -> Need to have a think about the best way to implement this