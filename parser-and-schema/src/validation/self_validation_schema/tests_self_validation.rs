use crate::{parser::schema_parser::{schema_tests_utils::{create_type_fields_for_tests, create_schema_types_for_test, create_enum_from_vec}, types::{type_system::Type, primitive_type::PrimitiveType, schema_type::SchemaType, schema_field::SchemaField}, parse_schema::parse_schema}, ast::syntax_definitions::{enum_definition::{EnumDefinition, EnumVariant, EnumDataType}, schema_definition::SchemaDefinition}};

/// It needs to check every type, enum etc that’s used is defined in the schema.

#[test]
fn parser_breaks_if_unknown_schema_type() {

    use crate::parser::schema_parser::parse_schema::parse_schema;
    // In the User field organization,
    // Company is an undefined schema type or enum
    // Therefore, this should throw an error to notify the engineer
    let query = "
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
    
    let actual = parse_schema(query);
    assert!(actual.is_err());
}

#[test]
fn can_parse_defined_schema_type_as_type() {
    use std::collections::HashMap;

    use crate::parser::schema_parser::parse_schema::parse_schema;

    let query = "
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

    
    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual.schema_types);
}

#[test]
fn parser_breaks_if_unknown_enum_type() {

    use crate::parser::schema_parser::parse_schema::parse_schema;
    let query = "
        type User {
            organization: Company,
            id: uuid,
            name: String,
            age: Int,
            is_admin: bool,
            location: String,
            log_in_count: Int,
        }
        
        enum Color {
            Red,
            Green,
            Blue
        }";
    
    let actual = parse_schema(query);
    assert!(actual.is_err());
}

#[test]
fn can_parse_defined_schema_enum_as_type_for_field() {
    use std::collections::HashMap;

    use crate::parser::schema_parser::parse_schema::parse_schema;

    let query = "
        type User {
            organization_type: OrganizationType
        }
        
        enum OrganizationType {
            Company,
            School,
            University,
        }";

    let user_fields = create_type_fields_for_tests(vec![
        ("organization_type".into(), Type::SchemaTypeOrEnum("OrganizationType".into()), Vec::new()),
    ]);
    
    let organization_type_enums: EnumDefinition = create_enum_from_vec("OrganizationType".into(), 
    vec![
        ("Company".into(), EnumVariant::new("Company".into(), EnumDataType::EnumUnit, HashMap::new())),
        ("School".into(), EnumVariant::new("School".into(), EnumDataType::EnumUnit, HashMap::new())),
        ("University".into(), EnumVariant::new("University".into(), EnumDataType::EnumUnit, HashMap::new())),
    ]);

    let mut enums = HashMap::new();
    enums.insert("OrganizationType".into(), organization_type_enums);

    let schema_types = create_schema_types_for_test(vec![
        ("User".into(), SchemaType::new("User".into(), user_fields))
    ]);

    let mut expected_schema_definition = SchemaDefinition::new();
    expected_schema_definition.schema_types = schema_types;
    expected_schema_definition.enums = enums;
    
    let actual = parse_schema(query).unwrap();
    assert_eq!(expected_schema_definition, actual);
}

#[test]
fn parser_breaks_if_unknown_type_in_argument() {

    use crate::parser::schema_parser::parse_schema::parse_schema;
    // In the User field organization,
    // Company is an undefined schema type or enum
    // Therefore, this should throw an error to notify the engineer
    let query = "
        type User {
            dp(id: User): String,
        }
        
        ";
    
    let actual = parse_schema(query);
    assert!(actual.is_err());
}

#[test]
fn can_parse_enum_with_tuple(){
    let schema = "
        enum FrameworkTypes {
            SomeOtherType(DoesntExist),
        }
    ";
    let actual = parse_schema(schema);
    assert!(actual.is_err());
}

#[test]
fn can_parse_enum_with_tuple_and_object(){
    let schema = "
        enum FrameworkTypes {
            SomeOtherType(DoesntExist),
            User {
                id: uuid,
                name: DoesntExist,
                age: Int,
            },
        }
    ";
    let actual = parse_schema(schema);
    assert!(actual.is_err());
}

#[test]
fn breaks_if_function_has_argument_undefined(){
    let schema = "
        fn do_nothing(id: DoesntExist, name: String) -> User
    ";

    let actual = parse_schema(schema);
    assert!(actual.is_err());
}

#[test]
fn breaks_if_function_has_result_undefined(){
    let schema = "
        fn do_nothing(id: String, name: String) -> DoesntExist
    ";

    let actual = parse_schema(schema);
    assert!(actual.is_err());
}

fn breaks_if_directive_has_argument_undefined(){
    let schema = "
        type meow {
            is_admin: bool @is_admin(role: String),
        }
    ";

    let actual = parse_schema(schema);
    assert!(actual.is_err());
}