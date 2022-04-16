use std::{collections::{HashMap}, vec};
use shared::castle_error::CastleError;
use parser_and_schema::{parsers::schema_parser::{schema_tests_utils::{create_type_fields_for_tests, create_schema_types_for_test, create_enum_from_vec, insert_enums_into_enum_definitions}, types::{parse_type::Type, primitive_type::PrimitiveType, schema_type::SchemaType, schema_field::SchemaField, vec_type::VecType, option_type::OptionType}, parse_schema::parse_schema}, ast::syntax_definitions::{enum_definition::{EnumVariant, EnumDataType, EnumDefinition}, schema_definition::SchemaDefinition, argument::{ArgumentOrTuple}, field_definition::FnDefinition, directive_definition::{Directive, self, DirectiveDefinition, DirectiveOnValue}}};


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

    let expected = create_schema_types_for_test(vec![
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

    let expected = create_schema_types_for_test(vec![
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

    let expected: HashMap<Box<str>, SchemaType> = create_schema_types_for_test(vec![
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
            ProfilePic(url: String),
            User(user_type: User),
            Organization(org_type: Organization),
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

    let profile_pic_args = ("url".into(), Type::PrimitiveType(PrimitiveType::String));
    let user_args = ("user_type".into(), Type::SchemaTypeOrEnum("User".into())); // are these correct?
    let organization_args = ("org_type".into(), Type::SchemaTypeOrEnum("Organization".into())); // are these correct?

    let framework_types_enum = create_enum_from_vec("FrameworkTypes".into(), vec![
        ("ProfilePic".into(), EnumVariant::new("ProfilePic".into(), EnumDataType::EnumTuple(vec![
            ArgumentOrTuple::IdentifierAndType(profile_pic_args) 
        ]), Vec::new())),
        ("User".into(), EnumVariant::new("User".into(), EnumDataType::EnumTuple(vec![
            ArgumentOrTuple::IdentifierAndType(user_args) 
        ]), Vec::new())),
        ("Organization".into(), EnumVariant::new("Organization".into(), EnumDataType::EnumTuple(vec![
            ArgumentOrTuple::IdentifierAndType(organization_args) 
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
            ProfilePic(x: String, y: String, z: String),
        }
    ";

    let argument_1 = ("x".into(), Type::PrimitiveType(PrimitiveType::String));
    let argument_2 = ("y".into(), Type::PrimitiveType(PrimitiveType::String));
    let argument_3 = ("z".into(), Type::PrimitiveType(PrimitiveType::String));

    let framework_types_enum = create_enum_from_vec("FrameworkTypes".into(), vec![
        ("ProfilePic".into(), EnumVariant::new("ProfilePic".into(), EnumDataType::EnumTuple(vec![
            ArgumentOrTuple::IdentifierAndType(argument_1),
            ArgumentOrTuple::IdentifierAndType(argument_2),
            ArgumentOrTuple::IdentifierAndType(argument_3),
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
            SomeOtherType(a_string: String),
        }
    ";

    let argument_1 = ("a_string".into(), Type::PrimitiveType(PrimitiveType::String));

    let framework_types_enum = create_enum_from_vec("FrameworkTypes".into(), vec![
        ("User".into(), EnumVariant::new("User".into(), EnumDataType::new_enum_object(vec![
            ("id".into(), SchemaField::new("id".into(), Type::PrimitiveType(PrimitiveType::Uuid), Vec::new())),
            ("name".into(), SchemaField::new("name".into(), Type::PrimitiveType(PrimitiveType::String), Vec::new())),
            ("age".into(), SchemaField::new("age".into(), Type::PrimitiveType(PrimitiveType::Int), Vec::new())),
        ]), Vec::new())),
        ("SomeOtherType".into(), EnumVariant::new("SomeOtherType".into(), EnumDataType::EnumTuple(vec![
            ArgumentOrTuple::IdentifierAndType(argument_1),
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
    let id_arg = Type::PrimitiveType(PrimitiveType::Uuid);
    let name_arg = Type::PrimitiveType(PrimitiveType::String);
    let user_id_arg = Type::PrimitiveType(PrimitiveType::Uuid);

    let name = "do_nothing".into();
    let mut args = HashMap::new();
    args.insert("name".into(), name_arg);
    args.insert("id".into(), id_arg);

    let return_type = Type::PrimitiveType(PrimitiveType::String);
    let fn_do_nothing = FnDefinition::new(name, args, return_type, vec![]);

    let name = "get_user".into();
    let mut args = HashMap::new();
    args.insert("id".into(), user_id_arg);

    let return_type = Type::SchemaTypeOrEnum("User".into());
    let fn_get_user = FnDefinition::new(name, args, return_type, vec![]);


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
    
    let token_arg = Type::PrimitiveType(PrimitiveType::String);
    let mut authenicated_args = HashMap::new();
    authenicated_args.insert("token".into(), token_arg);
    let role_arg = Type::SchemaTypeOrEnum("DoesntExist".into());
    let mut is_admin_args = HashMap::new();
    is_admin_args.insert("role".into(), role_arg);

    let mut expected_fields = HashMap::new();
    expected_fields.insert("is_admin".into(), SchemaField::new("is_admin".into(), Type::PrimitiveType(PrimitiveType::Bool), vec![
        Directive::new("authenticated".into(), authenicated_args),
        Directive::new("is_admin".into(), is_admin_args),
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
        directive @authenticated(token: String) on ENUM_VARIANT
        directive @is_admin(role: DoesntExist) on ENUM_VARIANT
    ";

    let token_arg = Type::PrimitiveType(PrimitiveType::String);
    let role_arg = Type::SchemaTypeOrEnum("DoesntExist".into());

    let mut authenticated_arguments: HashMap<Box<str>, Type> = HashMap::new();
    authenticated_arguments.insert("token".into(), token_arg);

    let mut is_admin_arguments = HashMap::new();
    is_admin_arguments.insert("role".into(), role_arg);

    let mut directives = Vec::new();
    directives.push(Directive::new("authenticated".into(), authenticated_arguments));
    directives.push(Directive::new("is_admin".into(), is_admin_arguments));

    let enum_variant = EnumVariant::new("Red".into(), EnumDataType::EnumUnit, directives);

    let token_arg = Type::PrimitiveType(PrimitiveType::String);
    let mut authenticated_arguments: HashMap<Box<str>, Type> = HashMap::new();
    authenticated_arguments.insert("token".into(), token_arg);

    let mut is_admin_arguments = HashMap::new();
    let role_arg = Type::SchemaTypeOrEnum("DoesntExist".into());
    is_admin_arguments.insert("role".into(), role_arg);
    let authenticated_directive = DirectiveDefinition::new("authenticated".into(), authenticated_arguments, DirectiveOnValue::EnumVariant);
    let is_admin_directive = DirectiveDefinition::new("is_admin".into(), is_admin_arguments, DirectiveOnValue::EnumVariant);
    let mut enum_meow = EnumDefinition::new("Meow".into(), HashMap::new());
    enum_meow.variants.insert("Red".into(), enum_variant);

    let mut expected = SchemaDefinition::new();
    expected.enums.insert("Meow".into(), enum_meow);
    expected.directives.insert("authenticated".into(), authenticated_directive);
    expected.directives.insert("is_admin".into(), is_admin_directive);

    let actual = parse_schema(schema).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_directives_fields() {
    let schema = "
    directive @test(ar: String) on FIELD
    ";

    let ar_arg = Type::PrimitiveType(PrimitiveType::String);

    let mut args = HashMap::new();
    args.insert("ar".into(), ar_arg);

    let directive_definition = DirectiveDefinition::new("test".into(), args, directive_definition::DirectiveOnValue::Field);

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

    let ar_arg = Type::PrimitiveType(PrimitiveType::String);

    let mut args = HashMap::new();
    args.insert("ar".into(), ar_arg);
    let directive_definition = DirectiveDefinition::new("test".into(), args, directive_definition::DirectiveOnValue::EnumVariant);

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
    // This is a comment $#@!@\"\"@!E$#@!
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


#[test]
fn test_vec_inside_option_type_works() -> Result<(), CastleError> {
    let schema = "
        type User {
            pets: Option<Vec<String>>
        }
    ";

    let actual = parse_schema(schema)?;
    let mut fields: HashMap<Box<str>, SchemaField> = HashMap::new();
    fields.insert("pets".into(), SchemaField::new("pets".into(), 
    Type::OptionType(OptionType { inner_type: Type::VecType(VecType { inner_type: Type::PrimitiveType(PrimitiveType::String).into() }).into()}).into(), 
    Vec::new()));

    let mut expected_types: HashMap<Box<str>, SchemaType> = HashMap::new();
    expected_types.insert("User".into(), SchemaType::new("User".into(), fields));
    assert_eq!(expected_types, actual.schema_types);
    return Ok(())
}

#[test]
fn test_option_inside_vec_inside() -> Result<(), CastleError> {
    let schema = "
        type User {
            pets: Vec<Option<String>>
        }
    ";

    let actual = parse_schema(schema)?;
    let mut fields: HashMap<Box<str>, SchemaField> = HashMap::new();
    fields.insert("pets".into(), SchemaField::new("pets".into(), 
    Type::VecType(VecType { inner_type: Type::OptionType(OptionType { inner_type: Type::PrimitiveType(PrimitiveType::String).into() }).into()}).into(), 
    Vec::new()));

    let mut expected_types: HashMap<Box<str>, SchemaType> = HashMap::new();
    expected_types.insert("User".into(), SchemaType::new("User".into(), fields));
    assert_eq!(expected_types, actual.schema_types);
    return Ok(())
}

#[test]
fn test_option_inside_hashmap() -> Result<(), CastleError> {
    let schema = "
        type User {
            pets: HashMap<Option<String>>
        }
    ";

    let actual = parse_schema(schema)?;
    let mut fields: HashMap<Box<str>, SchemaField> = HashMap::new();
    fields.insert("pets".into(), SchemaField::new("pets".into(), 
    Type::HashMapType(Type::OptionType(OptionType { inner_type: Type::PrimitiveType(PrimitiveType::String).into() }).into()).into(), 
    Vec::new()));

    let mut expected_types: HashMap<Box<str>, SchemaType> = HashMap::new();
    expected_types.insert("User".into(), SchemaType::new("User".into(), fields));
    assert_eq!(expected_types, actual.schema_types);
    return Ok(())
}

/// You'll need to:
/// - Update FnDefinition to have a field with Vec<Directive>
/// - Need to update OnDirectiveValue to also include resolver value
#[test]
fn can_parse_directives_on_resolver_definitions() -> Result<(), CastleError> {
    
    let schema = "
        fn me() -> String @authenticated @uppercase(amount: Int)
        directive @uppercase on FIELD
    ";

    let name = "me".into();
    let args = HashMap::new();

    let mut upercase_argument = HashMap::new();
    upercase_argument.insert("amount".into(), Type::PrimitiveType(PrimitiveType::Int));

    let mut directives = Vec::new();
    directives.push(Directive::new("authenticated".into(), HashMap::new()));
    directives.push(Directive::new("uppercase".into(), upercase_argument));

    let return_type = Type::PrimitiveType(PrimitiveType::String);
    let fn_me = FnDefinition::new(name, args, return_type, directives);

    let mut expected_functions: HashMap<Box<str>, FnDefinition> = HashMap::new();
    expected_functions.insert("me".into(), fn_me);

    let on = DirectiveOnValue::Field;
    let expected_directives = DirectiveDefinition::new("uppercase".into(), HashMap::new(), on);

    let mut insert_directives = HashMap::new();
    insert_directives.insert("uppercase".into(), expected_directives);

    let expected = SchemaDefinition {
        schema_types: HashMap::new(),
        enums: HashMap::new(),
        directives: insert_directives,
        functions: expected_functions,
    };

    let actual = parse_schema(schema)?;
    assert_eq!(actual, expected);
    return Ok(())
}

/// This should pass the same as fn me() -> ()
#[test]
fn can_parse_resolver_with_default_return_type() -> Result<(), CastleError> {    
    let schema = "
        fn me() -> String
    ";
    let schema2 = "
        fn me -> String
    ";
    let actual = parse_schema(schema)?;
    let actual2 = parse_schema(schema2)?;
    assert_eq!(actual, actual2);
    return Ok(())
}