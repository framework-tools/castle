use castle::validation::{validate_query_with_schema::validate_query_with_schema::validate_query_with_schema, self_validation_schema::self_validate_schema::self_validate_schema};
use parser_and_schema::{parsers::{schema_parser::parse_schema::parse_schema, query_parser::parse_query::parse_query}};
use shared::castle_error::CastleError;

/// Currently Testing:
/// - Check top level want in query uses a defined resolver
/// - Should break if mismatched arguments in top level resolver (both ways)
/// - Should break if mismatched fields in return type
/// - Breaks if enums used are invalid (Parent, or variant)
/// - Should break if mismatched fields in return type of inner resolver/object projection
/// - Should pass if inner projection match resolver return type
/// - should break if match arm field does not exist in return type

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
    fn name() -> Name
    
    type User {
        name: Name,
        age: Int,
        role: String
    }

    enum Name {
        StandardName,
        UserName
    }

    type StandardName {
        first_name: String,
        last_name: String
    }

    type UserName {
        username: String
    }

    ";

    let query = "
    me(id: 543) {
        name() match {
            Name::UserName => {
                username
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
    fn name() -> Name
    
    type User {
        name: Name,
        age: Int,
        role: String
    }

    enum Name {
        StandardName,
        UserName
    }

    type StandardName {
        first_name: String,
        last_name: String
    }

    type UserName {
        username: String
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

#[test]
fn should_pass_if_inner_objects_have_correct_return_types_for_each_resolver() -> Result<(), CastleError>{
    let schema = "
    fn me() -> User
    fn name() -> Name
    fn organization() -> Organization
    fn address() -> Address
    
    type User {
        name: Name,
        age: Int,
        role: String,
        organization: Organization
    }

    type Name {
        first_name: String,
        last_name: String,
    }

    type Organization {
        name: String,
        address: Address,
    }

    type Address {
        street: String,
        city: String,
        state: String,
        country: String
    }

    ";

    let query = "
    me() {
        name() {
            first_name,
            last_name
        },
        role,
        organization() {
            name,
            address() {
                city,
                country
            }
        }
    }
    ";

    let schema_definition = parse_schema(schema)?;
    self_validate_schema(&schema_definition)?;
    let parsed_query = parse_query(query)?;
    validate_query_with_schema(&parsed_query, &schema_definition)?;
    Ok(())
}

#[test]
fn should_break_if_inner_objects_have_field_not_defined_in_type() -> Result<(), CastleError>{
    let schema = "
    fn me() -> User
    fn name() -> Name
    fn organization() -> Organization
    fn address() -> Address
    
    type User {
        name: Name,
        age: Int,
        role: String,
        organization: Organization
    }

    type Name {
        first_name: String,
        last_name: String,
    }

    type Organization {
        name: String,
        address: Address,
    }

    type Address {
        street: String,
        city: String,
        state: String,
        country: String
    }

    ";
    //field planet on address is not defined
    let query = "
    me() {
        name() {
            first_name,
            last_name
        },
        role,
        organization() {
            name,
            address() {
                city,
                planet 
            }
        }
    }
    ";

    let schema_definition = parse_schema(schema)?;
    self_validate_schema(&schema_definition)?;
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
fn should_throw_err_if_inner_object_projection_does_not_have_a_defined_resolver() -> Result<(), CastleError> {
    let schema = "
    fn me() -> User
    fn organization() -> Organization
    
    type User {
        age: Int,
        role: String,
        organization: Organization
    }

    type ThisDoesNotExist {
        planet: String,
    }

    type Organization {
        name: String,
        address: Address,
        this_does_not_exist: ThisDoesNotExist,
    }

    type Address {
        street: String,
        city: String,
        state: String,
        country: String
    }

    ";
    //field planet on address is not defined
    let query = "
    me() {
        age,
        role,
        organization() {
            name,
            this_does_not_exist() {
                planet 
            }
        }
    }
    ";

    let schema_definition = parse_schema(schema)?;
    self_validate_schema(&schema_definition)?;
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
fn should_throw_error_if_match_arm_references_type_in_enum_with_wrong_field() -> Result<(), CastleError>{
    let schema = "
    fn name() -> Name
    fn me(id: Int) -> User
    
    type User {
        name: Name,
        age: Int,
        role: String
    }

    enum Name {
        StandardName {
            first_name: String,
            last_name: String
        },
        UserName {
            username: String,
        }
    }

    type StandardName {
        first_name: String,
        last_name: String
    }

    type UserName {
        username: String,
    }

    ";

    let query = "
    me(id: 543) {
        name() match {
            Name::StandardName => {
                first_name,
                last_name
            },
            Name::UserName => {
                not_correct_field
            }
        }
    }
    ";

    let schema_definition = parse_schema(schema)?;
    let parsed_query = parse_query(query)?;
    let result = validate_query_with_schema(&parsed_query, &schema_definition);
    if result.is_err() {
        match result {
            Err(CastleError::EnumVariantDoesNotHaveMatchingType(_message)) => return Ok(()),
            _ => panic!("threw wrong error expected EnumFieldNotDefinedInSchema, got: {:?}", result)
        }
    } else {
        panic!("should have thrown error");
    }
}


