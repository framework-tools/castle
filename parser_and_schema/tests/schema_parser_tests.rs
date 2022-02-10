use std::{collections::HashMap, vec, string};


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
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), Vec::new()),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
        ("age".into(), Type::PrimitiveType(PrimitiveType::Int), Vec::new()),
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
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), Vec::new()),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
        ("age".into(), Type::PrimitiveType(PrimitiveType::Int), Vec::new()),
        ("is_admin".into(), Type::PrimitiveType(PrimitiveType::Bool), Vec::new()),
        ("location".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
        ("log_in_count".into(), Type::PrimitiveType(PrimitiveType::Int), Vec::new()),
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
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), Vec::new()),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
        ("age".into(), Type::PrimitiveType(PrimitiveType::Int), Vec::new()),
        ("is_admin".into(), Type::PrimitiveType(PrimitiveType::Bool), Vec::new()),
        ("location".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
        ("log_in_count".into(), Type::PrimitiveType(PrimitiveType::Int), Vec::new()),
    ]);

    let organization_fields: HashMap<Box<str>, SchemaField> = create_type_fields_for_tests(vec![
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), Vec::new()),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
        ("industry".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
    ]);

    let mut expected: HashMap<Box<str>, SchemaType> = create_schema_types_for_test(vec![
        ("User".into(), SchemaType::new("User".into(), user_fields)),
        ("Organization".into(), SchemaType::new("Organization".into(), organization_fields)),
    ]);
    
    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual.schema_types);
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

    let user_fields = create_type_fields_for_tests(vec![
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), Vec::new()),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
        ("age".into(), Type::PrimitiveType(PrimitiveType::Int), Vec::new()),
        ("is_admin".into(), Type::PrimitiveType(PrimitiveType::Bool), Vec::new()),
        ("location".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
        ("log_in_count".into(), Type::PrimitiveType(PrimitiveType::Int), Vec::new()),
    ]);
    
    let organization_fields: HashMap<Box<str>, SchemaField> = create_type_fields_for_tests(vec![
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), Vec::new()),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
        ("industries".into(), Type::VecType(VecType {inner_type: Type::PrimitiveType(PrimitiveType::String).into()}), Vec::new()),
        ("users".into(), Type::VecType(VecType {inner_type: Type::SchemaTypeOrEnum("User".into()).into()}), Vec::new())
    ]);
    let expected = create_schema_types_for_test(vec![
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
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), Vec::new()),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
        ("age".into(), Type::PrimitiveType(PrimitiveType::Int), Vec::new()),
        ("is_admin".into(), Type::PrimitiveType(PrimitiveType::Bool), Vec::new()),
        ("location".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
        ("log_in_count".into(), Type::PrimitiveType(PrimitiveType::Int), Vec::new()),
    ]);
    let expected_types = create_schema_types_for_test(vec![
        ("User".into(), SchemaType::new("User".into(), user_fields)),
    ]);

    let expected = SchemaDefinition {
        schema_types: expected_types,
        enums: expected_enums,

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
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), Vec::new()),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
        ("age".into(), Type::PrimitiveType(PrimitiveType::Int), Vec::new()),
        ("is_admin".into(), Type::PrimitiveType(PrimitiveType::Bool), Vec::new()),
        ("location".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
        ("log_in_count".into(), Type::PrimitiveType(PrimitiveType::Int), Vec::new()),
    ]);
    
    let organization_fields: HashMap<Box<str>, SchemaField> = create_type_fields_for_tests(vec![
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), Vec::new()),
        ("name".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
        ("industries".into(), Type::VecType(VecType {inner_type: Type::PrimitiveType(PrimitiveType::String).into()}), Vec::new()),
        ("users".into(), Type::VecType(VecType {inner_type: Type::SchemaTypeOrEnum("User".into()).into()}), Vec::new())
    ]);
    let expected_types = create_schema_types_for_test(vec![
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
            ("id".into(), SchemaField::new("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), Vec::new())),
            ("name".into(), SchemaField::new("name".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new())),
            ("age".into(), SchemaField::new("age".into(), Type::PrimitiveType(PrimitiveType::Int), Vec::new())),
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

        functions: HashMap::new(),
    };

    let actual = parse_schema(schema).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_function_with_args_and_return_type(){
    let schema = "
        fn do_nothing(id: uuid, name: String) -> String
    ";

    let mut fn_do_nothing = FnDefinition::new();
    fn_do_nothing.name = "do_nothing".into();
    fn_do_nothing.args = Some(vec![
        Argument::IdentifierAndType("id".into(), Type::PrimitiveType(PrimitiveType::Uuid)),
        Argument::IdentifierAndType("name".into(), Type::PrimitiveType(PrimitiveType::String))
    ]);

    fn_do_nothing.return_type = Some(Type::PrimitiveType(PrimitiveType::String));

    let mut expected_functions: HashMap<Box<str>, FnDefinition> = HashMap::new();   
    expected_functions.insert("do_nothing".into(), fn_do_nothing);

    let expected = SchemaDefinition {
        schema_types: HashMap::new(),
        enums: HashMap::new(),
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
        ("url".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
        ("width".into(), Type::PrimitiveType(PrimitiveType::Int), Vec::new()),
        ("height".into(), Type::PrimitiveType(PrimitiveType::Int), Vec::new()),
    ]);

    let user_fields = create_type_fields_for_tests(vec![
        ("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), Vec::new()),
        ("name".into(), Type::OptionType(OptionType { inner_type: Type::PrimitiveType(PrimitiveType::String).into()}), Vec::new()),
        ("profile_pic".into(), Type::OptionType(OptionType { inner_type: Type::SchemaTypeOrEnum("ProfilePic".into()).into()}), Vec::new()),
    ]);

    let expected_types = create_schema_types_for_test(vec![
        ("User".into(), SchemaType::new("User".into(), user_fields)),
        ("ProfilePic".into(), SchemaType::new("ProfilePic".into(), profile_pic_fields)),
    ]);

    let expected = SchemaDefinition {
        schema_types: expected_types,
        enums: HashMap::new(),
        functions: HashMap::new(),
    };
    assert_eq!(expected, parse_schema(schema).unwrap());
}

#[test]
fn can_parse_directives(){
    let schema = "

        type User {
            name: Option<String> @random_directive(arg1: Int, arg2: String),
            first_name: String,
            last_name: String,
            age: Int,
            password: String @is_authenticated @is_encrypted,
            is_admin: bool @is_admin(role: String),
        }
    ";

    let arg1 = Argument::IdentifierAndType("arg1".into(), Type::PrimitiveType(PrimitiveType::Int));
    let arg2 = Argument::IdentifierAndType("arg2".into(), Type::PrimitiveType(PrimitiveType::String));
    let name_directive_arguments = vec![arg1, arg2];
    let name_directives = vec![
        DirectiveDefinition::new("random_directive".into(), Some(name_directive_arguments)),
    ];

    let password_directives = vec![
        DirectiveDefinition::new("is_authenticated".into(), None),
        DirectiveDefinition::new("is_encrypted".into(), None),
    ];
    let is_admin_directive = DirectiveDefinition::new("is_admin".into(), Some(vec![
        Argument::IdentifierAndType("role".into(), Type::PrimitiveType(PrimitiveType::String))
    ]));

    let user_fields = create_type_fields_for_tests(vec![
        ("name".into(), Type::OptionType(OptionType { inner_type: Type::PrimitiveType(PrimitiveType::String).into()}), name_directives),
        ("first_name".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
        ("last_name".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new()),
        ("age".into(), Type::PrimitiveType(PrimitiveType::Int), Vec::new()),
        ("password".into(), Type::PrimitiveType(PrimitiveType::String), password_directives),
        ("is_admin".into(), Type::PrimitiveType(PrimitiveType::Bool), vec![is_admin_directive]),
    ]);
    
    let user_type = SchemaType::new("User".into(), user_fields);
    
    let mut expected = SchemaDefinition {
        schema_types: HashMap::new(),
        enums: HashMap::new(),
        functions: HashMap::new(),
    };
    expected.schema_types.insert("User".into(), user_type);
    let actual = parse_schema(schema).unwrap();
    assert_eq!(expected, actual);
}

// Need to write 1 more test for each piece of functionality
// to ensure working correctly