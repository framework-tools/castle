use castle_backend::validation::self_validation_schema::self_validate_schema::self_validate_schema;
use parser_and_schema::{parsers::schema_parser::{parse_schema::parse_schema, schema_tests_utils::{create_type_fields_for_tests, create_schema_types_for_test, create_enum_from_vec}, types::{type_system::Type, primitive_type::PrimitiveType, schema_field::SchemaField, schema_type::SchemaType}}, ast::syntax_definitions::{enum_definition::{EnumDefinition, EnumVariant, EnumDataType}, schema_definition::SchemaDefinition}};
use shared::CastleError;

/// It needs to check every type, enum etc that’s used is defined in the schema.
/// 
/// Currently Testing:
/// - Unknown type on SchemaType field
///     - Schema type not defined or enum type
/// - Enum values (tuple and object) has unknown type
/// - Vec Types has unknown type
/// - Option Types has unknown type
/// - Function arguments has unknown type
/// - Function return value has unknown type
/// - Directive arguments has unknown type

#[cfg(test)]
#[test]
fn parser_breaks_if_unknown_schema_type_or_enum() -> Result<(), CastleError> {
    use castle_backend::validation::self_validation_schema::self_validate_schema::self_validate_schema;

    // In the User field organization,
    // Company is an undefined schema type or enum
    // Therefore, this should throw an error to notify the engineer
    let schema = "
        type User {
            organization: Company,
            id: uuid,
            name: String,
            age: Int,
            is_admin: bool,
            location: String,
            log_in_count: Int,
        }

        type Organization {
            id: uuid,
            name: String,
            industry: String,
        }";
    
    let schema_definition = parse_schema(schema)?;
    let actual = self_validate_schema(&schema_definition);
    if actual.is_err() {
        match actual {
            Err(CastleError::UndefinedTypeOrEnumInSchema(_)) => { return Ok(()) }, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", actual),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn can_parse_defined_schema_type_as_type() {
    use std::collections::HashMap;

    let schema = "
        type User {
            id: uuid,
            name: String,
            age: Int,
            is_admin: bool,
            location: String,
            log_in_count: Int
            organization: Organization
        }

        type Organization {
            id: uuid,
            name: String,
            industry: String,
        }";

    let user_fields = create_type_fields_for_tests(vec![
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), Vec::new()),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
        ("age".into(), Type::PrimitiveType(PrimitiveType::Int), Vec::new()),
        ("is_admin".into(), Type::PrimitiveType(PrimitiveType::Bool), Vec::new()),
        ("location".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
        ("log_in_count".into(), Type::PrimitiveType(PrimitiveType::Int), Vec::new()),
        ("organization".into(), Type::SchemaTypeOrEnum("Organization".into()), Vec::new()),
    ]);
    
    let organization_fields: HashMap<Box<str>, SchemaField> = create_type_fields_for_tests(vec![
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), Vec::new()),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
        ("industry".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
    ]);

    let expected = create_schema_types_for_test(vec![
        ("User".into(), SchemaType::new("User".into(), user_fields)),
        ("Organization".into(), SchemaType::new("Organization".into(), organization_fields)),
    ]);

    
    let actual = parse_schema(schema).unwrap();
    assert_eq!(expected, actual.schema_types);
}


#[test]
fn can_parse_defined_schema_enum_as_type_for_field() {
    use std::collections::HashMap;

    let schema = "
        type User {
            role: String,
            organization_type: OrganizationType
        }
        
        enum OrganizationType {
            Company,
            School,
            University,
        }";

    let user_fields = create_type_fields_for_tests(vec![
        ("role".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
        ("organization_type".into(), Type::SchemaTypeOrEnum("OrganizationType".into()), Vec::new()),
    ]);
    
    let organization_type_enums: EnumDefinition = create_enum_from_vec("OrganizationType".into(), 
    vec![
        ("Company".into(), EnumVariant::new("Company".into(), EnumDataType::EnumUnit, Vec::new())),
        ("School".into(), EnumVariant::new("School".into(), EnumDataType::EnumUnit, Vec::new())),
        ("University".into(), EnumVariant::new("University".into(), EnumDataType::EnumUnit, Vec::new())),
    ]);

    let mut enums = HashMap::new();
    enums.insert("OrganizationType".into(), organization_type_enums);

    let schema_types = create_schema_types_for_test(vec![
        ("User".into(), SchemaType::new("User".into(), user_fields))
    ]);

    let mut expected_schema_definition = SchemaDefinition::new();
    expected_schema_definition.schema_types = schema_types;
    expected_schema_definition.enums = enums;
    
    let actual = parse_schema(schema).unwrap();
    assert_eq!(expected_schema_definition, actual);
}

#[test]
fn err_if_parses_enum_with_unknown_tuple_type() -> Result<(), CastleError> {
    let schema = "
        enum FrameworkTypes {
            SomeOtherType(String, DoesntExist)
        }
    ";

    let schema_definition = parse_schema(schema)?;
    let actual = self_validate_schema(&schema_definition);
    if actual.is_err() {
        match actual {
            Err(CastleError::UndefinedTypeOrEnumInSchema(_)) => { return Ok(()) }, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", actual),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn err_if_parses_enum_with_unknown_object_type() -> Result<(), CastleError> {
    let schema = "
        enum FrameworkTypes {
            User {
                id: uuid,
                name: DoesntExist,
                age: Int,
            },
        }
    ";

    let schema_definition = parse_schema(schema)?;
    let actual = self_validate_schema(&schema_definition);
    if actual.is_err() {
        match actual {
            Err(CastleError::UndefinedTypeOrEnumInSchema(_)) => { return Ok(()) }, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", actual),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn breaks_if_function_has_argument_undefined() -> Result<(), CastleError> {
    let schema = "
        fn do_nothing(name: String, id: DoesntExist) -> User
    ";

    let schema_definition = parse_schema(schema)?;
    let actual = self_validate_schema(&schema_definition);
    if actual.is_err() {
        match actual {
            Err(CastleError::UndefinedTypeOrEnumInSchema(_)) => { return Ok(()) }, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", actual),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn breaks_if_function_has_tuple_argument() -> Result<(), CastleError> {
    let schema = "
        fn do_nothing(name: String, id: DoesntExist) -> (String, User)
    ";

    let schema_definition = parse_schema(schema)?;
    let actual = self_validate_schema(&schema_definition);
    if actual.is_err() {
        match actual {
            Err(CastleError::UndefinedTypeOrEnumInSchema(_)) => { return Ok(()) }, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", actual),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn breaks_if_function_has_return_type_undefined() -> Result<(), CastleError> {
    let schema = "
        fn do_nothing(id: String, name: String) -> DoesntExist
    ";

    let schema_definition = parse_schema(schema)?;
    let actual = self_validate_schema(&schema_definition);
    if actual.is_err() {
        match actual {
            Err(CastleError::UndefinedTypeOrEnumInSchema(_)) => { return Ok(()) }, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", actual),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn breaks_if_directive_has_argument_undefined() -> Result<(), CastleError> {
    let schema = "
        type Meow {
            is_admin: bool @authenticated(token: String) @is_admin(role: DoesntExist),
        }
    ";

    let schema_definition = parse_schema(schema)?;
    let actual = self_validate_schema(&schema_definition);
    if actual.is_err() {
        match actual {
            Err(CastleError::UndefinedTypeOrEnumInSchema(_)) => { return Ok(()) }, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", actual),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn test_vector_type_breaks_if_type_is_not_defined( ) -> Result<(), CastleError> {
    let schema = "
        type User {
            pets: Vec<DoesntExist>
        }
    ";

    let schema_definition = parse_schema(schema)?;
    let actual = self_validate_schema(&schema_definition);
    if actual.is_err() {
        match actual {
            Err(CastleError::UndefinedTypeOrEnumInSchema(_)) => { return Ok(()) }, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", actual),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn test_vector_type_with_inner_vec_breaks_if_type_is_not_defined() -> Result<(), CastleError> {
    let schema = "
        type User {
            pets: Vec<Vec<DoesntExist>>
        }
    ";

    let schema_definition = parse_schema(schema)?;
    let actual = self_validate_schema(&schema_definition);
    if actual.is_err() {
        match actual {
            Err(CastleError::UndefinedTypeOrEnumInSchema(_)) => { return Ok(()) }, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", actual),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn test_option_type_breaks_if_type_is_not_defined() -> Result<(), CastleError> {
    let schema = "
        type User {
            pets: Option<DoesntExist>
        }
    ";

    let schema_definition = parse_schema(schema)?;
    let actual = self_validate_schema(&schema_definition);
    if actual.is_err() {
        match actual {
            Err(CastleError::UndefinedTypeOrEnumInSchema(_)) => { return Ok(()) }, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", actual),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn test_vec_inside_option_type_breaks_if_type_is_not_defined() -> Result<(), CastleError> {
    let schema = "
        type User {
            pets: Option<Vec<DoesntExist>>
        }
    ";

    let schema_definition = parse_schema(schema)?;
    let actual = self_validate_schema(&schema_definition);
    if actual.is_err() {
        match actual {
            Err(CastleError::UndefinedTypeOrEnumInSchema(_)) => { return Ok(()) }, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", actual),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn test_option_type_inside_vec_breaks_if_type_is_not_defined( ) -> Result<(), CastleError> {
    let schema = "
        type User {
            pets: Vec<Option<Vec<Option<DoesntExist>>>>
        }
    ";

    let schema_definition = parse_schema(schema)?;
    let actual = self_validate_schema(&schema_definition);
    if actual.is_err() {
        match actual {
            Err(CastleError::UndefinedTypeOrEnumInSchema(_)) => { return Ok(()) }, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", actual),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn should_break_if_used_directive_is_not_defined( ) -> Result<(), CastleError> {
    let schema = "
        type User {
            pets: String @authenticated(token: String) @doesnt_exist
        }
        directive @authenticated(token: String) on FIELD
    ";

    let schema_definition = parse_schema(schema)?;
    let actual = self_validate_schema(&schema_definition);
    if actual.is_err() {
        match actual {
            Err(CastleError::UndefinedDirective(_)) => { return Ok(()) }, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", actual),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn should_break_if_used_directive_is_not_defined_enum( ) -> Result<(), CastleError> {
    let schema = "
        enum User {
            Red,
            Blue,
            Green @doesnt_exist
        }
        directive @authenticated(token: String) on ENUM_VARIANT
    ";

    let schema_definition = parse_schema(schema)?;
    let actual = self_validate_schema(&schema_definition);
    if actual.is_err() {
        match actual {
            Err(CastleError::UndefinedDirective(_)) => { return Ok(()) }, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", actual),
        }
    } else {
        panic!("No error thrown");
    }
}

#[test]
fn should_break_if_used_directive_definition_have_mismatched_arguments( ) -> Result<(), CastleError> {
    let schema = "
        enum User {
            Red,
            Blue,
            Green @authenticated (token: Int)
        }
        directive @authenticated(token: String) on ENUM_VARIANT
    ";

    let schema_definition = parse_schema(schema)?;
    let actual = self_validate_schema(&schema_definition);
    if actual.is_err() {
        match actual {
            Err(CastleError::DirectiveDoesNotMatchSchemaDirective(_)) => { return Ok(()) }, //passes
            _ => panic!("Expected error to be of type UndefinedTypeOrEnumInSchema, found: {:?}", actual),
        }
    } else {
        panic!("No error thrown");
    }
}