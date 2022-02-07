use std::collections::HashMap;

use crate::{parser::{schema_parser::{types::{schema_field::{SchemaField, PrimitiveType, Type}, schema_type::SchemaType}, schema_tests_utils::create_type_fields_for_tests}, self}, token::token::VecType, ast::syntax_definitions::{enum_definition::{EnumDefinition, EnumVariant, EnumDataType}, schema_definition::SchemaDefinition, want::Argument}};

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
    use std::collections::HashMap;

    use crate::parser::schema_parser::parse_schema::parse_schema;

    let query = "
        type User {
            id: uuid,
            name: String,
            age: Int,
        }";

    let user_fields = create_type_fields_for_tests(vec![
        ("id".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid)),
        ("name".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
        ("age".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int)),
    ]);

    let mut expected = HashMap::new();
    expected.insert("User".into(), SchemaType {
        identifier: "User".into(),
        fields: user_fields,
    }); 
    
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
        ("id".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid)),
        ("name".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
        ("age".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int)),
        ("is_admin".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Bool)),
        ("location".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
        ("log_in_count".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int)),
    ]);

    let mut expected = HashMap::new();
    expected.insert("User".into(), SchemaType {
        identifier: "User".into(),
        fields: user_fields,
    }); 
    
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
        ("id".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid)),
        ("name".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
        ("age".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int)),
        ("is_admin".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Bool)),
        ("location".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
        ("log_in_count".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int)),
    ]);
    let mut organization_fields: HashMap<Box<str>, SchemaField> = create_type_fields_for_tests(vec![
        ("id".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid)),
        ("name".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
        ("industry".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
    ]);

    let mut expected: HashMap<Box<str>, SchemaType> = HashMap::new();
    expected.insert("User".into(), SchemaType {
        identifier: "User".into(),
        fields: user_fields,
    }); 
    expected.insert("Organization".into(), SchemaType {
        identifier: "Organization".into(),
        fields: organization_fields,
    });
    
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
        ("id".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid)),
        ("name".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
        ("age".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int)),
        ("is_admin".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Bool)),
        ("location".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
        ("log_in_count".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int)),
        ("organization".into(), parser::schema_parser::types::schema_field::Type::SchemaTypeOrEnum("Organization".into())),
    ]);

    let mut expected: HashMap<Box<str>, SchemaType> = HashMap::new();
    expected.insert("User".into(), SchemaType {
        identifier: "User".into(),
        fields: user_fields,
    }); 
    
    let organization_fields: HashMap<Box<str>, SchemaField> = create_type_fields_for_tests(vec![
        ("id".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid)),
        ("name".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
        ("industry".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
    ]);

    expected.insert("Organization".into(), SchemaType {
        identifier: "Organization".into(),
        fields: organization_fields,
    });
    
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
        ("id".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid)),
        ("name".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
        ("age".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int)),
        ("is_admin".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Bool)),
        ("location".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
        ("log_in_count".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int)),
        ("organization".into(), parser::schema_parser::types::schema_field::Type::SchemaTypeOrEnum("Company".into())),
    ]);
    
    let organization_fields: HashMap<Box<str>, SchemaField> = create_type_fields_for_tests(vec![
        ("id".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid)),
        ("name".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
        ("industry".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
    ]);

    let mut expected: HashMap<Box<str>, SchemaType> = HashMap::new();
    expected.insert("User".into(), SchemaType {
        identifier: "User".into(),
        fields: user_fields,
    });
    expected.insert("Organization".into(), SchemaType {
        identifier: "Organization".into(),
        fields: organization_fields,
    });
    
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
        ("id".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid)),
        ("name".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
        ("age".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int)),
        ("is_admin".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Bool)),
        ("location".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
        ("log_in_count".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int)),
    ]);
    
    let mut organization_fields: HashMap<Box<str>, SchemaField> = create_type_fields_for_tests(vec![
        ("id".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid)),
        ("name".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
        ("industries".into(), parser::schema_parser::types::schema_field::Type::VecType(VecType {inner_type: Type::PrimitiveType(PrimitiveType::String).into()})),
        ("users".into(), parser::schema_parser::types::schema_field::Type::VecType(VecType {inner_type: Type::SchemaTypeOrEnum("User".into()).into()}))
    ]);
    let mut expected = HashMap::new();
    expected.insert("User".into(), SchemaType {
        identifier: "User".into(),
        fields: user_fields,
    }); 
    expected.insert("Organization".into(), SchemaType {
        identifier: "Organization".into(),
        fields: organization_fields,
    });
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

    let mut enum_variants = HashMap::new();
    enum_variants.insert("Red".into(), EnumVariant { name: "Red".into(), enum_data_type: EnumDataType::EnumUnit, directives: HashMap::new() });
    enum_variants.insert("Green".into(), EnumVariant { name: "Green".into(), enum_data_type: EnumDataType::EnumUnit, directives: HashMap::new() });
    enum_variants.insert("Blue".into(), EnumVariant { name: "Blue".into(), enum_data_type: EnumDataType::EnumUnit, directives: HashMap::new() });
    let enum_ = EnumDefinition {
        name: "Color".into(),
        variants: enum_variants,
        directives: HashMap::new()
    };

    let mut expected: HashMap<Box<str>, EnumDefinition> = HashMap::new();
    expected.insert("Color".into(), enum_);

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

    let mut enum_variants = HashMap::new();
    enum_variants.insert("Red".into(), EnumVariant { name: "Red".into(), enum_data_type: EnumDataType::EnumUnit, directives: HashMap::new() });
    enum_variants.insert("Green".into(), EnumVariant { name: "Green".into(), enum_data_type: EnumDataType::EnumUnit, directives: HashMap::new() });
    enum_variants.insert("Blue".into(), EnumVariant { name: "Blue".into(), enum_data_type: EnumDataType::EnumUnit, directives: HashMap::new() });
    let color_enum = EnumDefinition {
        name: "Color".into(),
        variants: enum_variants,
        directives: HashMap::new()
    };

    let mut enum_variants = HashMap::new();
    enum_variants.insert("Happy".into(), EnumVariant { name: "Happy".into(), enum_data_type: EnumDataType::EnumUnit, directives: HashMap::new() });
    enum_variants.insert("Sad".into(), EnumVariant { name: "Sad".into(), enum_data_type: EnumDataType::EnumUnit, directives: HashMap::new() });
    enum_variants.insert("Angry".into(), EnumVariant { name: "Angry".into(), enum_data_type: EnumDataType::EnumUnit, directives: HashMap::new() });
    enum_variants.insert("Fearful".into(), EnumVariant { name: "Fearful".into(), enum_data_type: EnumDataType::EnumUnit, directives: HashMap::new() });
    enum_variants.insert("Depressed".into(), EnumVariant { name: "Depressed".into(), enum_data_type: EnumDataType::EnumUnit, directives: HashMap::new() });
    let emotion_enum = EnumDefinition {
        name: "Emotion".into(),
        variants: enum_variants,
        directives: HashMap::new()
    };

    let mut expected_enums: HashMap<Box<str>, EnumDefinition> = HashMap::new();
    expected_enums.insert("Color".into(), color_enum);
    expected_enums.insert("Emotion".into(), emotion_enum);

    let user_fields = create_type_fields_for_tests(vec![
        ("id".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid)),
        ("name".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
        ("age".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int)),
        ("is_admin".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Bool)),
        ("location".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
        ("log_in_count".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int)),
    ]);
    let mut expected_types = HashMap::new();
    expected_types.insert("User".into(), SchemaType {
        identifier: "User".into(),
        fields: user_fields,
    }); 

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
        ("id".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid)),
        ("name".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
        ("age".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int)),
        ("is_admin".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Bool)),
        ("location".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
        ("log_in_count".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int)),
    ]);
    
    let organization_fields: HashMap<Box<str>, SchemaField> = create_type_fields_for_tests(vec![
        ("id".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid)),
        ("name".into(), parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String)),
        ("industries".into(), parser::schema_parser::types::schema_field::Type::VecType(VecType {inner_type: Type::PrimitiveType(PrimitiveType::String).into()})),
        ("users".into(), parser::schema_parser::types::schema_field::Type::VecType(VecType {inner_type: Type::SchemaTypeOrEnum("User".into()).into()}))
    ]);
    let mut expected_types = HashMap::new();
    expected_types.insert("User".into(), SchemaType {
        identifier: "User".into(),
        fields: user_fields,
    }); 

    expected_types.insert("Organization".into(), SchemaType {
        identifier: "Organization".into(),
        fields: organization_fields,
    });

    let mut enum_variants = HashMap::new();
    enum_variants.insert("ProfilePic".into(), EnumVariant { name: "ProfilePic".into(), enum_data_type: EnumDataType::EnumTuple(vec![
        Argument::Type(Type::PrimitiveType(PrimitiveType::String)) 
    ]), directives: HashMap::new() });
    enum_variants.insert("User".into(), EnumVariant { name: "User".into(), enum_data_type: EnumDataType::EnumTuple(vec![
        Argument::Type(Type::SchemaTypeOrEnum("User".into())) 
    ]), directives: HashMap::new() });
    enum_variants.insert("Organization".into(), EnumVariant { name: "Organization".into(), enum_data_type: EnumDataType::EnumTuple(vec![
        Argument::Type(Type::SchemaTypeOrEnum("Organization".into())) 
    ]), directives: HashMap::new() });
    let mut enums = HashMap::new();
    enums.insert(
        "FrameworkTypes".into(),
        EnumDefinition {
            name: "FrameworkTypes".into(),
            variants: enum_variants,
            directives: HashMap::new(),
        },
    );

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


// To Implement:
// - Parse Enums with primitive values
// - Parse Enums with type values - incl checking for undefined types
// - Parse functions
// - Parse directives
// - Parse implements
// - Parse traits ???

