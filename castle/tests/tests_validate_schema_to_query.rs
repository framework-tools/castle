use castle::validation::validate_query_with_schema::validate_query_with_schema::validate_query_with_schema;
use parser_and_schema::{parsers::{schema_parser::parse_schema::parse_schema, query_parser::parse_query::parse_query}};
use shared::CastleError;

/// Currently Testing:
/// - Check top level want in query uses a defined resolver
/// - Should break if mismatched arguments in top level resolver (both ways)
/// - Should break if mismatched fields in return type
/// - Breaks if enums used are invalid (Parent, or variant)

#[test]
fn if_object_projection_identifier_is_not_defined_as_resolver_in_schema_should_throw_error() -> Result<(), CastleError>{
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
            _ => panic!("threw wrong error expected QueryResolverNotDefinedInSchema, got: {:?}", result)
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
            _ => panic!("threw wrong error expected ArgumentsInQueryDoNotMatchResolver, got:{:?}", result)
        }
    } else {
        panic!("should have thrown error");
    }
}

#[test]
fn should_break_if_mismatched_arguments_other_way() -> Result<(), CastleError>{
    let schema = "
    fn me(id: Int, currency: String) -> User
    type User {
            amount: Int,
            currency: String,
        }
    ";

    let query = "
    me(id: 432)
    ";

    let schema_definition = parse_schema(schema)?;
    let parsed_query = parse_query(query)?;
    let result = validate_query_with_schema(&parsed_query, &schema_definition);
    if result.is_err() {
        match result {
            Err(CastleError::ArgumentsInQueryDoNotMatchResolver(_message)) => return Ok(()),
            _ => panic!("threw wrong error expected ArgumentsInQueryDoNotMatchResolver, got: {:?}", result)
        }
    } else {
        panic!("should have thrown error");
    }
}



#[test]
fn should_break_if_same_arguments_but_mismatching_types() -> Result<(), CastleError>{
    let schema = "
    fn me(id: Int) -> User
    type User {
            amount: Int,
            currency: String,
        }
    ";

    let query = "
    me(id: \"This is a string\") {
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
            _ => panic!("threw wrong error expected ArgumentsInQueryDoNotMatchResolver, got: {:?}", result)
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
            Err(CastleError::FieldsInReturnTypeDoNotMatchQuery(_message)) => return Ok(()),
            _ => panic!("threw wrong error expected FieldsInReturnTypeDoNotMatchQuery, got: {:?}", result)
        }
    } else {
        panic!("should have thrown error");
    }
}


#[test]
fn should_break_if_use_undefined_enum_parent_in_query() -> Result<(), CastleError>{
    let schema = "
    fn me(id: Int) -> User
    
    type User {
        name: Name,
        age: Int,
        role: String
    }

    enum Name {
        StandardName,
        UserName
    }

    ";

    let query = "
    me(id: 543) {
        name() match {
            Name::UserName => {
                name
            },
            DoesntExist::DoesntExist => {
                first_name,
                last_name
            },
        }
    }
    ";

    let schema_definition = parse_schema(schema)?;
    let parsed_query = parse_query(query)?;
    let result = validate_query_with_schema(&parsed_query, &schema_definition);
    if result.is_err() {
        match result {
            Err(CastleError::EnumInQueryNotDefinedInSchema(_message)) => return Ok(()),
            _ => panic!("threw wrong error expected EnumInQueryNotDefinedInSchema, got: {:?}", result)
        }
    } else {
        panic!("should have thrown error");
    }
}

#[test]
fn should_break_if_use_undefined_enum_variant_in_query() -> Result<(), CastleError>{
    let schema = "
    fn me(id: Int) -> User
    
    type User {
        name: Name,
        age: Int,
        role: String
    }

    enum Name {
        StandardName,
        UserName
    }

    ";

    let query = "
    me(id: 543) {
        name() match {
            Name::StandardName => {
                first_name,
                last_name
            },
            Name::DoesntExist => {
                first_name,
                last_name
            }
        }
    }
    ";

    let schema_definition = parse_schema(schema)?;
    let parsed_query = parse_query(query)?;
    let result = validate_query_with_schema(&parsed_query, &schema_definition);
    if result.is_err() {
        match result {
            Err(CastleError::EnumInQueryNotDefinedInSchema(_message)) => return Ok(()),
            _ => panic!("threw wrong error expected EnumInQueryNotDefinedInSchema, got: {:?}", result)
        }
    } else {
        panic!("should have thrown error");
    }
}