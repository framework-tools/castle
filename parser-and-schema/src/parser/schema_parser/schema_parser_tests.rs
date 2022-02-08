use std::{collections::HashMap, vec, string};

use crate::{parser::{schema_parser::{types::{schema_field::{SchemaField}, schema_type::SchemaType, type_system::Type, primitive_type::PrimitiveType, vec_type::VecType, option_type::OptionType}, schema_tests_utils::{create_type_fields_for_tests, create_schema_types_for_test, create_enum_from_vec, insert_enums_into_enum_definitions}}, self, query_parser::query_tests_utils::insert_each_field_into_fields}, ast::syntax_definitions::{enum_definition::{EnumDefinition, EnumVariant, EnumDataType}, schema_definition::SchemaDefinition, argument::Argument, fn_definition::FnDefinition, directive_definition::DirectiveDefinition}};

use super::parse_schema::parse_schema;




#[cfg(test)]
#[test]
fn can_parse_empty_query() {
    use std::collections::HashMap;

    use crate::parser::schema_parser::parse_schema::parse_schema;

    let query = "";
    let expected = HashMap::new();
    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual.schema_types);
}

#[test]
fn can_parse_simple_type() {

    use crate::parser::schema_parser::parse_schema::parse_schema;

    let query = "
        type User {
            id: uuid,
            name: String,
            age: Int,
        }";

    let user_fields = create_type_fields_for_tests(vec![
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), None),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), None),
        ("age".into(), Type::PrimitiveType(PrimitiveType::Int), None),
    ]);

    let user_type = SchemaType::new("User".into(), user_fields);

    let mut expected = create_schema_types_for_test(vec![
        ("User".into(), user_type),
    ]);
    
    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual.schema_types);
}

#[test]
fn can_parse_simple_type_more_fields_and_no_commas() {
    use std::collections::HashMap;

    use crate::parser::schema_parser::parse_schema::parse_schema;

    let query = "
        type User {
            id: uuid
            name: String
            age: Int
            is_admin: bool
            location: String
            log_in_count: Int
        }";

    let user_fields = create_type_fields_for_tests(vec![
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), None),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), None),
        ("age".into(), Type::PrimitiveType(PrimitiveType::Int), None),
        ("is_admin".into(), Type::PrimitiveType(PrimitiveType::Bool), None),
        ("location".into(), Type::PrimitiveType(PrimitiveType::String), None),
        ("log_in_count".into(), Type::PrimitiveType(PrimitiveType::Int), None),
    ]);

    let mut expected = create_schema_types_for_test(vec![
        ("User".into(), SchemaType::new("User".into(), user_fields)),
    ]);
    
    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual.schema_types);
}

#[test]
fn can_parse_two_types() {
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
        }
        
        type Organization {
            id: uuid,
            name: String,
            industry: String,
        }";

    let user_fields = create_type_fields_for_tests(vec![
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), None),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), None),
        ("age".into(), Type::PrimitiveType(PrimitiveType::Int), None),
        ("is_admin".into(), Type::PrimitiveType(PrimitiveType::Bool), None),
        ("location".into(), Type::PrimitiveType(PrimitiveType::String), None),
        ("log_in_count".into(), Type::PrimitiveType(PrimitiveType::Int), None),
    ]);

    let organization_fields: HashMap<Box<str>, SchemaField> = create_type_fields_for_tests(vec![
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), None),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), None),
        ("industry".into(), Type::PrimitiveType(PrimitiveType::String), None),
    ]);

    let mut expected: HashMap<Box<str>, SchemaType> = create_schema_types_for_test(vec![
        ("User".into(), SchemaType::new("User".into(), user_fields)),
        ("Organization".into(), SchemaType::new("Organization".into(), organization_fields)),
    ]);
    
    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual.schema_types);
}

#[test]
fn can_parse_two_types_with_defined_value() {
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
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), None),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), None),
        ("age".into(), Type::PrimitiveType(PrimitiveType::Int), None),
        ("is_admin".into(), Type::PrimitiveType(PrimitiveType::Bool), None),
        ("location".into(), Type::PrimitiveType(PrimitiveType::String), None),
        ("log_in_count".into(), Type::PrimitiveType(PrimitiveType::Int), None),
        ("organization".into(), Type::SchemaTypeOrEnum("Organization".into()), None),
    ]);
    
    let organization_fields: HashMap<Box<str>, SchemaField> = create_type_fields_for_tests(vec![
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), None),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), None),
        ("industry".into(), Type::PrimitiveType(PrimitiveType::String), None),
    ]);

    let expected = create_schema_types_for_test(vec![
        ("User".into(), SchemaType::new("User".into(), user_fields)),
        ("Organization".into(), SchemaType::new("Organization".into(), organization_fields)),
    ]);
    
    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual.schema_types);
}


#[test]
fn parser_breaks_if_unknown_schema_type() {
    use std::collections::HashMap;

    use crate::parser::schema_parser::parse_schema::parse_schema;
    // In the User field organization,
    // Company is an undefined schema type
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

    let user_fields = create_type_fields_for_tests(vec![
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), None),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), None),
        ("age".into(), Type::PrimitiveType(PrimitiveType::Int), None),
        ("is_admin".into(), Type::PrimitiveType(PrimitiveType::Bool), None),
        ("location".into(), Type::PrimitiveType(PrimitiveType::String), None),
        ("log_in_count".into(), Type::PrimitiveType(PrimitiveType::Int), None),
        ("organization".into(), Type::SchemaTypeOrEnum("Company".into()), None),
    ]);
    
    let organization_fields = create_type_fields_for_tests(vec![
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), None),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), None),
        ("industry".into(), Type::PrimitiveType(PrimitiveType::String), None),
    ]);

    let mut expected = create_schema_types_for_test(vec![
        ("User".into(), SchemaType::new("User".into(), user_fields)),
        ("Organization".into(), SchemaType::new("Organization".into(), organization_fields)),
    ]);
    
    let actual = parse_schema(query);
    assert!(actual.is_err());
}


#[test]
fn can_parse_two_types_with_vec_type() {
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
        }
        
        type Organization {
            id: uuid,
            name: String,
            industries: Vec<String>,
            users: Vec<User>
        }";

    let mut user_fields = create_type_fields_for_tests(vec![
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), None),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), None),
        ("age".into(), Type::PrimitiveType(PrimitiveType::Int), None),
        ("is_admin".into(), Type::PrimitiveType(PrimitiveType::Bool), None),
        ("location".into(), Type::PrimitiveType(PrimitiveType::String), None),
        ("log_in_count".into(), Type::PrimitiveType(PrimitiveType::Int), None),
    ]);
    
    let mut organization_fields: HashMap<Box<str>, SchemaField> = create_type_fields_for_tests(vec![
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), None),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), None),
        ("industries".into(), Type::VecType(VecType {inner_type: Type::PrimitiveType(PrimitiveType::String).into()}), None),
        ("users".into(), Type::VecType(VecType {inner_type: Type::SchemaTypeOrEnum("User".into()).into()}), None)
    ]);
    let mut expected = create_schema_types_for_test(vec![
        ("User".into(), SchemaType::new("User".into(), user_fields)),
        ("Organization".into(), SchemaType::new("Organization".into(), organization_fields)),
    ]);

    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual.schema_types);
}

#[test]
fn can_parse_enum_schema() {
    let query = "
        enum Color {
            Red,
            Green,
            Blue
        }
    ";

    let color_enum = create_enum_from_vec("Color".into(), vec![
        ("Red".into(), EnumVariant::new("Red".into(), EnumDataType::EnumUnit, HashMap::new())),
        ("Green".into(), EnumVariant::new("Green".into(), EnumDataType::EnumUnit, HashMap::new())),
        ("Blue".into(), EnumVariant::new("Blue".into(), EnumDataType::EnumUnit, HashMap::new())),
    ]);

    let expected: HashMap<Box<str>, EnumDefinition> = insert_enums_into_enum_definitions(vec![
        ("Color".into(), color_enum),
    ]);

    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual.enums);
}

#[test]
fn can_parse_two_enums_and_type_schema() {
    let query = "
        enum Color {
            Red,
            Green,
            Blue
        }

        enum Emotion {
            Happy,
            Sad,
            Angry,
            Fearful,
            Depressed,
        }

        type User {
            id: uuid,
            name: String,
            age: Int,
            is_admin: bool,
            location: String,
            log_in_count: Int
        }
    ";

    let color_enum = create_enum_from_vec("Color".into(), vec![
        ("Red".into(), EnumVariant::new("Red".into(), EnumDataType::EnumUnit, HashMap::new())),
        ("Green".into(), EnumVariant::new("Green".into(), EnumDataType::EnumUnit, HashMap::new())),
        ("Blue".into(), EnumVariant::new("Blue".into(), EnumDataType::EnumUnit, HashMap::new())),
    ]);

    let emotion_enum = create_enum_from_vec("Emotion".into(), vec![
        ("Happy".into(), EnumVariant::new("Happy".into(), EnumDataType::EnumUnit, HashMap::new())),
        ("Sad".into(), EnumVariant::new("Sad".into(), EnumDataType::EnumUnit, HashMap::new())),
        ("Angry".into(), EnumVariant::new("Angry".into(), EnumDataType::EnumUnit, HashMap::new())),
        ("Fearful".into(), EnumVariant::new("Fearful".into(), EnumDataType::EnumUnit, HashMap::new())),
        ("Depressed".into(), EnumVariant::new("Depressed".into(), EnumDataType::EnumUnit, HashMap::new())),
    ]);

    let expected_enums: HashMap<Box<str>, EnumDefinition> = insert_enums_into_enum_definitions(vec![
        ("Color".into(), color_enum),
        ("Emotion".into(), emotion_enum),
    ]);

    let user_fields = create_type_fields_for_tests(vec![
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), None),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), None),
        ("age".into(), Type::PrimitiveType(PrimitiveType::Int), None),
        ("is_admin".into(), Type::PrimitiveType(PrimitiveType::Bool), None),
        ("location".into(), Type::PrimitiveType(PrimitiveType::String), None),
        ("log_in_count".into(), Type::PrimitiveType(PrimitiveType::Int), None),
    ]);
    let expected_types = create_schema_types_for_test(vec![
        ("User".into(), SchemaType::new("User".into(), user_fields)),
    ]);

    let expected = SchemaDefinition {
        schema_types: expected_types,
        enums: expected_enums,
        traits: HashMap::new(),
        impls: HashMap::new(),
        functions: HashMap::new(),
    };

    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_enum_schema_with_values() {
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
        }
        
        type Organization {
            id: uuid,
            name: String,
            industries: Vec<String>,
            users: Vec<User>
        }
        
        enum FrameworkTypes {
            ProfilePic(String),
            User(User),
            Organization(Organization),
        }
        ";

    let user_fields = create_type_fields_for_tests(vec![
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), None),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), None),
        ("age".into(), Type::PrimitiveType(PrimitiveType::Int), None),
        ("is_admin".into(), Type::PrimitiveType(PrimitiveType::Bool), None),
        ("location".into(), Type::PrimitiveType(PrimitiveType::String), None),
        ("log_in_count".into(), Type::PrimitiveType(PrimitiveType::Int), None),
    ]);
    
    let organization_fields: HashMap<Box<str>, SchemaField> = create_type_fields_for_tests(vec![
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), None),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), None),
        ("industries".into(), Type::VecType(VecType {inner_type: Type::PrimitiveType(PrimitiveType::String).into()}), None),
        ("users".into(), Type::VecType(VecType {inner_type: Type::SchemaTypeOrEnum("User".into()).into()}), None)
    ]);
    let mut expected_types = create_schema_types_for_test(vec![
        ("User".into(), SchemaType::new("User".into(), user_fields)),
        ("Organization".into(), SchemaType::new("Organization".into(), organization_fields)),
    ]);

    let framework_types_enum = create_enum_from_vec("FrameworkTypes".into(), vec![
        ("ProfilePic".into(), EnumVariant::new("ProfilePic".into(), EnumDataType::EnumTuple(vec![
            Argument::Type(Type::PrimitiveType(PrimitiveType::String)) 
        ]), HashMap::new())),
        ("User".into(), EnumVariant::new("User".into(), EnumDataType::EnumTuple(vec![
            Argument::Type(Type::SchemaTypeOrEnum("User".into())) 
        ]), HashMap::new())),
        ("Organization".into(), EnumVariant::new("Organization".into(), EnumDataType::EnumTuple(vec![
            Argument::Type(Type::SchemaTypeOrEnum("Organization".into())) 
        ]), HashMap::new())),
    ]);

    let enums = insert_enums_into_enum_definitions(vec![
        ("FrameworkTypes".into(), framework_types_enum),
    ]);

    let expected = SchemaDefinition {
        schema_types: expected_types,
        enums,
        traits: HashMap::new(),
        impls: HashMap::new(),
        functions: HashMap::new(),
        
    };

    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_enum_multiple_arguments(){
    let schema = "
        enum FrameworkTypes {
            ProfilePic(String, String, String),
        }
    ";

    let framework_types_enum = create_enum_from_vec("FrameworkTypes".into(), vec![
        ("ProfilePic".into(), EnumVariant::new("ProfilePic".into(), EnumDataType::EnumTuple(vec![
            Argument::Type(Type::PrimitiveType(PrimitiveType::String)),
            Argument::Type(Type::PrimitiveType(PrimitiveType::String)),
            Argument::Type(Type::PrimitiveType(PrimitiveType::String)),
        ]), HashMap::new())),
    ]);

    let enums = insert_enums_into_enum_definitions(vec![
        ("FrameworkTypes".into(), framework_types_enum),
    ]);

    let expected = SchemaDefinition {
        schema_types: HashMap::new(),
        enums,
        traits: HashMap::new(),
        impls: HashMap::new(),
        functions: HashMap::new(),
    };

    let actual = parse_schema(schema).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_enum_with_fields(){
    let schema = "
        enum FrameworkTypes {
            User {
                id: uuid,
                name: String,
                age: Int,
            },
            SomeOtherType(String),
        }
    ";

    let framework_types_enum = create_enum_from_vec("FrameworkTypes".into(), vec![
        ("User".into(), EnumVariant::new("User".into(), EnumDataType::new_enum_object(vec![
            ("id".into(), SchemaField::new("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), None)),
            ("name".into(), SchemaField::new("name".into(), Type::PrimitiveType(PrimitiveType::String), None)),
            ("age".into(), SchemaField::new("age".into(), Type::PrimitiveType(PrimitiveType::Int), None)),
        ]), HashMap::new())),
        ("SomeOtherType".into(), EnumVariant::new("SomeOtherType".into(), EnumDataType::EnumTuple(vec![
            Argument::Type(Type::PrimitiveType(PrimitiveType::String)),
        ]), HashMap::new()))
    ]);

    let enums = insert_enums_into_enum_definitions(vec![
        ("FrameworkTypes".into(), framework_types_enum),
    ]);

    let expected = SchemaDefinition {
        schema_types: HashMap::new(),
        enums,
        traits: HashMap::new(),
        impls: HashMap::new(),
        functions: HashMap::new(),
    };

    let actual = parse_schema(schema).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_function_with_args_and_return_type(){
    let schema = "
        fn do_nothing(id: uuid, name: String) -> User {

        }
    ";

    let mut fn_do_nothing = FnDefinition::new();
    fn_do_nothing.name = "do_nothing".into();
    fn_do_nothing.args = Some(vec![
        Argument::IdentifierAndType("id".into(), Type::PrimitiveType(PrimitiveType::Uuid)),
        Argument::IdentifierAndType("name".into(), Type::PrimitiveType(PrimitiveType::String))
    ]);

    fn_do_nothing.return_type = Some(Type::SchemaTypeOrEnum("User".into()));

    let mut expected_functions: HashMap<Box<str>, FnDefinition> = HashMap::new();   
    expected_functions.insert("do_nothing".into(), fn_do_nothing);

    let expected = SchemaDefinition {
        schema_types: HashMap::new(),
        enums: HashMap::new(),
        traits: HashMap::new(),
        impls: HashMap::new(),
        functions: expected_functions,
    };
    let actual = parse_schema(schema).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_option_type(){
    let schema = "
        type User {
            id: uuid,
            name: Option<String>,
            profile_pic: Option<ProfilePic>,
        }

        type ProfilePic {
            url: String,
            width: Int,
            height: Int,
        }
    ";

    let profile_pic_fields = create_type_fields_for_tests(vec![
        ("url".into(), Type::PrimitiveType(PrimitiveType::String), None),
        ("width".into(), Type::PrimitiveType(PrimitiveType::Int), None),
        ("height".into(), Type::PrimitiveType(PrimitiveType::Int), None),
    ]);

    let user_fields = create_type_fields_for_tests(vec![
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), None),
        ("name".into(), Type::OptionType(OptionType { inner_type: Type::PrimitiveType(PrimitiveType::String).into()}), None),
        ("profile_pic".into(), Type::OptionType(OptionType { inner_type: Type::SchemaTypeOrEnum("ProfilePic".into()).into()}), None),
    ]);

    let expected_types = create_schema_types_for_test(vec![
        ("User".into(), SchemaType::new("User".into(), user_fields)),
        ("ProfilePic".into(), SchemaType::new("ProfilePic".into(), profile_pic_fields)),
    ]);

    let expected = SchemaDefinition {
        schema_types: expected_types,
        enums: HashMap::new(),
        traits: HashMap::new(),
        impls: HashMap::new(),
        functions: HashMap::new(),
    };
    assert_eq!(expected, parse_schema(schema).unwrap());
}

#[test]
fn can_parse_directives(){
    let schema = "
        type User {
            name: Option<String> into String,
            first_name: String into Option<String>,
            last_name: String into Option<String>,
            age: Int
        }
    ";

    let user_fields = create_type_fields_for_tests(vec![
        ("name".into(), Type::OptionType(OptionType { inner_type: Type::PrimitiveType(PrimitiveType::String).into()}), Some(DirectiveDefinition::new(Type::PrimitiveType(PrimitiveType::String))
        )),
        ("first_name".into(), Type::PrimitiveType(PrimitiveType::String).into(), Some(DirectiveDefinition::new(Type::OptionType(OptionType { inner_type: Type::PrimitiveType(PrimitiveType::String).into()})))),
        ("last_name".into(), Type::PrimitiveType(PrimitiveType::String).into(), Some(DirectiveDefinition::new(Type::OptionType(OptionType { inner_type: Type::PrimitiveType(PrimitiveType::String).into()})))),
        ("age".into(), Type::PrimitiveType(PrimitiveType::Int).into(), None)
    ]);
    let user_type = SchemaType::new("User".into(), user_fields);
    
    let mut expected = SchemaDefinition {
        schema_types: HashMap::new(),
        traits: HashMap::new(),
        enums: HashMap::new(),
        impls: HashMap::new(),
        functions: HashMap::new(),
    };
    expected.schema_types.insert("User".into(), user_type);
    let actual = parse_schema(schema).unwrap();
    assert_eq!(expected, actual);

}

#[test]
fn should_fail_directives_that_are_not_compatible(){
    let schema = "
        type User {
            last_name: String into Option<Float>,
        }
    ";

    let actual = parse_schema(schema);
    assert!(actual.is_err());
}

// To Implement:
// - Parse implements

// Need to write 1 more test for each piece of functionality
// to ensure working correctly