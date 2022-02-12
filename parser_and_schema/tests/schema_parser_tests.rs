use std::{collections::{HashMap, hash_map}, vec, string};

use parser_and_schema::{parsers::schema_parser::{schema_tests_utils::{create_type_fields_for_tests, create_schema_types_for_test, create_enum_from_vec, insert_enums_into_enum_definitions}, types::{type_system::Type, primitive_type::PrimitiveType, schema_type::SchemaType, schema_field::SchemaField, vec_type::VecType, option_type::OptionType}, parse_schema::parse_schema}, ast::syntax_definitions::{enum_definition::{EnumVariant, EnumDataType, EnumDefinition}, schema_definition::SchemaDefinition, argument::Argument, fn_definition::FnDefinition, directive_definition::{Directive, self, DirectiveDefinition}}};

#[test]
fn can_parse_empty_query() {
    use std::collections::HashMap;

    use parser_and_schema::parsers::schema_parser::parse_schema::parse_schema;

    let query = "";
    let expected = HashMap::new();
    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual.schema_types);
}

#[test]
fn can_parse_simple_type() {

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
        ("Red".into(), EnumVariant::new("Red".into(), EnumDataType::EnumUnit, Vec::new())),
        ("Green".into(), EnumVariant::new("Green".into(), EnumDataType::EnumUnit, Vec::new())),
        ("Blue".into(), EnumVariant::new("Blue".into(), EnumDataType::EnumUnit, Vec::new())),
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
        ("Red".into(), EnumVariant::new("Red".into(), EnumDataType::EnumUnit, Vec::new())),
        ("Green".into(), EnumVariant::new("Green".into(), EnumDataType::EnumUnit, Vec::new())),
        ("Blue".into(), EnumVariant::new("Blue".into(), EnumDataType::EnumUnit, Vec::new())),
    ]);

    let emotion_enum = create_enum_from_vec("Emotion".into(), vec![
        ("Happy".into(), EnumVariant::new("Happy".into(), EnumDataType::EnumUnit, Vec::new())),
        ("Sad".into(), EnumVariant::new("Sad".into(), EnumDataType::EnumUnit, Vec::new())),
        ("Angry".into(), EnumVariant::new("Angry".into(), EnumDataType::EnumUnit, Vec::new())),
        ("Fearful".into(), EnumVariant::new("Fearful".into(), EnumDataType::EnumUnit, Vec::new())),
        ("Depressed".into(), EnumVariant::new("Depressed".into(), EnumDataType::EnumUnit, Vec::new())),
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
        directives: HashMap::new(),

        functions: HashMap::new(),
    };

    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_enum_schema_with_values() {
    use std::collections::HashMap;

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
        ]), Vec::new())),
        ("User".into(), EnumVariant::new("User".into(), EnumDataType::EnumTuple(vec![
            Argument::Type(Type::SchemaTypeOrEnum("User".into())) 
        ]), Vec::new())),
        ("Organization".into(), EnumVariant::new("Organization".into(), EnumDataType::EnumTuple(vec![
            Argument::Type(Type::SchemaTypeOrEnum("Organization".into())) 
        ]), Vec::new())),
    ]);

    let enums = insert_enums_into_enum_definitions(vec![
        ("FrameworkTypes".into(), framework_types_enum),
    ]);

    let expected = SchemaDefinition {
        schema_types: expected_types,
        enums,
        directives: HashMap::new(),
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
        ]), Vec::new())),
    ]);

    let enums = insert_enums_into_enum_definitions(vec![
        ("FrameworkTypes".into(), framework_types_enum),
    ]);

    let expected = SchemaDefinition {
        schema_types: HashMap::new(),
        enums,
        directives: HashMap::new(),
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
        ]), Vec::new())),
        ("SomeOtherType".into(), EnumVariant::new("SomeOtherType".into(), EnumDataType::EnumTuple(vec![
            Argument::Type(Type::PrimitiveType(PrimitiveType::String)),
        ]), Vec::new()))
    ]);

    let enums = insert_enums_into_enum_definitions(vec![
        ("FrameworkTypes".into(), framework_types_enum),
    ]);

    let expected = SchemaDefinition {
        schema_types: HashMap::new(),
        enums,
        directives: HashMap::new(),
        functions: HashMap::new(),
    };

    let actual = parse_schema(schema).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_function_with_args_and_return_type(){
    let schema = "
        fn do_nothing (id: uuid, name: String) -> String
        fn get_user(id: uuid) -> User 
    ";

    let mut fn_do_nothing = FnDefinition::initalise();
    fn_do_nothing.name = "do_nothing".into();
    fn_do_nothing.args = Some(vec![
        Argument::IdentifierAndType("id".into(), Type::PrimitiveType(PrimitiveType::Uuid)),
        Argument::IdentifierAndType("name".into(), Type::PrimitiveType(PrimitiveType::String))
    ]);

    fn_do_nothing.return_type = Some(Type::PrimitiveType(PrimitiveType::String));

    let mut fn_get_user = FnDefinition::initalise();
    fn_get_user.name = "get_user".into();
    fn_get_user.args = Some(vec![
        Argument::IdentifierAndType("id".into(), Type::PrimitiveType(PrimitiveType::Uuid)),
    ]);
    fn_get_user.return_type = Some(Type::SchemaTypeOrEnum("User".into()));

    let mut expected_functions: HashMap<Box<str>, FnDefinition> = HashMap::new();   
    expected_functions.insert("do_nothing".into(), fn_do_nothing);
    expected_functions.insert("get_user".into(), fn_get_user);

    let expected = SchemaDefinition {
        schema_types: HashMap::new(),
        enums: HashMap::new(),
        directives: HashMap::new(),
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
        directives: HashMap::new(),
    };
    assert_eq!(expected, parse_schema(schema).unwrap());
}

#[test]
fn can_parse_directives_on_fields(){
    let schema = "
        type Meow {
            is_admin: bool @authenticated(token: String) @is_admin(role: DoesntExist),
        }
    ";

    let mut expected_fields = HashMap::new();
    expected_fields.insert("is_admin".into(), SchemaField::new("is_admin".into(), Type::PrimitiveType(PrimitiveType::Bool), vec![
        Directive::new("authenticated".into(), Some(vec![
            Argument::IdentifierAndType("token".into(), Type::PrimitiveType(PrimitiveType::String)),
        ])),
        Directive::new("is_admin".into(), Some(vec![
            Argument::IdentifierAndType("role".into(), Type::SchemaTypeOrEnum("DoesntExist".into())),
        ])),
    ]));
    let mut expected_types = HashMap::new();
    expected_types.insert("Meow".into(), SchemaType::new("Meow".into(), expected_fields));
    let expected = SchemaDefinition {
        schema_types: expected_types,
        enums: HashMap::new(),
        directives: HashMap::new(),
        functions: HashMap::new(),
    };
    let actual = parse_schema(schema).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_directives_on_enums(){
    let schema = "
        enum Meow {
            Red @authenticated(token: String) @is_admin(role: DoesntExist),
        }
    ";

    let mut directives = Vec::new();
    directives.push(Directive::new("authenticated".into(), Some(vec![
        Argument::IdentifierAndType("token".into(), Type::PrimitiveType(PrimitiveType::String)),
    ])));
    directives.push(Directive::new("is_admin".into(), Some(vec![
        Argument::IdentifierAndType("role".into(), Type::SchemaTypeOrEnum("DoesntExist".into())),
    ])));

    let enum_variant = EnumVariant::new("Red".into(), EnumDataType::EnumUnit, directives);

    let mut enum_meow = EnumDefinition::new("Meow".into(), HashMap::new(), HashMap::new());
    enum_meow.variants.insert("Red".into(), enum_variant);

    let mut expected = SchemaDefinition::new();
    expected.enums.insert("Meow".into(), enum_meow);
    let actual = parse_schema(schema).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_directives_fields(){
    let schema = "
    directive @test(ar: String) on FIELD
    ";
    let arguments = vec![
        Argument::IdentifierAndType("ar".into(), Type::PrimitiveType(PrimitiveType::String)),
    ];
    let function = FnDefinition::new("test".into(), Some(arguments), None);
    let directive_definition = DirectiveDefinition::new(function, directive_definition::DirectiveOnValue::Field);

    let mut expected = SchemaDefinition {
        schema_types: HashMap::new(),
        enums: HashMap::new(),
        directives: HashMap::new(),
        functions: HashMap::new(),
    };
    expected.directives.insert("test".into(), directive_definition);
    let actual = parse_schema(schema).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_directives_enums(){
    let schema = "
    directive @test(ar: String) on ENUM_VARIANT
    ";
    let arguments = vec![
        Argument::IdentifierAndType("ar".into(), Type::PrimitiveType(PrimitiveType::String)),
    ];
    let function = FnDefinition::new("test".into(), Some(arguments), None);
    let directive_definition = DirectiveDefinition::new(function, directive_definition::DirectiveOnValue::EnumVariant);

    let mut expected = SchemaDefinition {
        schema_types: HashMap::new(),
        enums: HashMap::new(),
        directives: HashMap::new(),
        functions: HashMap::new(),
    };
    expected.directives.insert("test".into(), directive_definition);
    let actual = parse_schema(schema).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_comments(){
    let schema = "
    // This is a comment
    type Meow {
        // This is a comment
        is_admin: bool //This is also a comment
    }
   //lol ";
    let mut expected_fields = HashMap::new();
    expected_fields.insert("is_admin".into(), SchemaField::new("is_admin".into(), Type::PrimitiveType(PrimitiveType::Bool), Vec::new()));
    let mut expected_types = HashMap::new();
    expected_types.insert("Meow".into(), SchemaType::new("Meow".into(), expected_fields));
    let expected = SchemaDefinition {
        schema_types: expected_types,
        enums: HashMap::new(),
        directives: HashMap::new(),
        functions: HashMap::new(),
    };
    let actual = parse_schema(schema).unwrap();
    assert_eq!(expected, actual);
}