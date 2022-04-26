use std::{collections::HashMap};

use schema_parser::{types::{TypeDefinition, FieldDefinition, Kind, SchemaDefinition, EnumDefinition, VariantDefinition, VariantKindDefinition, DirectiveDefinition, Directive}, parsers::parse_schema::parse_schema};
use shared_parser::{Input, Variant, VariantType};


#[test]
fn can_parse_empty_query() {
    let query = "";
    let expected = SchemaDefinition::new();
    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_simple_type() {

    let query = "
        type User {
            id: uuid,
        }";

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: HashMap::new(),
        types: [
            ("User".into(), TypeDefinition {
                identifier: "User".into(),
                directives: vec![],
                fields: [
                    ("id".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "uuid".into(), generics: vec![] },
                        name: "id".into(),
                    }),
                ].into(),
            }),
        ].into(),
    };

    let actual = parse_schema(query).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn can_parse_simple_type_more_fields_and_no_commas() {

    let query = "
        type User {
            id: uuid
            name: String
            age: Int
        }";

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: HashMap::new(),
        types: [
            ("User".into(), TypeDefinition {
                identifier: "User".into(),
                directives: vec![],
                fields: [
                    ("id".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "uuid".into(), generics: vec![] },
                        name: "id".into(),
                    }),
                    ("name".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "String".into(), generics: vec![] },
                        name: "name".into(),
                    }),
                    ("age".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "Int".into(), generics: vec![] },
                        name: "age".into(),
                    }),
                ].into(),
            }),
        ].into(),
    };

    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual);
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

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: HashMap::new(),
        types: [
            ("User".into(), TypeDefinition {
                identifier: "User".into(),
                directives: vec![],
                fields: [
                    ("id".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "uuid".into(), generics: vec![] },
                        name: "id".into(),
                    }),
                    ("name".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "String".into(), generics: vec![] },
                        name: "name".into(),
                    }),
                    ("age".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "Int".into(), generics: vec![] },
                        name: "age".into(),
                    }),
                    ("is_admin".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "bool".into(), generics: vec![] },
                        name: "is_admin".into(),
                    }),
                    ("location".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "String".into(), generics: vec![] },
                        name: "location".into(),
                    }),
                    ("log_in_count".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "Int".into(), generics: vec![] },
                        name: "log_in_count".into(),
                    }),
                ].into(),
            }),
            ("Organization".into(), TypeDefinition {
                identifier: "Organization".into(),
                directives: vec![],
                fields: [
                    ("id".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "uuid".into(), generics: vec![] },
                        name: "id".into(),
                    }),
                    ("name".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "String".into(), generics: vec![] },
                        name: "name".into(),
                    }),
                    ("industry".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "String".into(), generics: vec![] },
                        name: "industry".into(),
                    }),
                ].into(),
            }),
        ].into(),
    };

    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual);
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

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: HashMap::new(),
        types: [
            ("User".into(), TypeDefinition {
                identifier: "User".into(),
                directives: vec![],
                fields: [
                    ("id".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "uuid".into(), generics: vec![] },
                        name: "id".into(),
                    }),
                    ("name".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "String".into(), generics: vec![] },
                        name: "name".into(),
                    }),
                    ("age".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "Int".into(), generics: vec![] },
                        name: "age".into(),
                    }),
                    ("is_admin".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "bool".into(), generics: vec![] },
                        name: "is_admin".into(),
                    }),
                    ("location".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "String".into(), generics: vec![] },
                        name: "location".into(),
                    }),
                    ("log_in_count".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "Int".into(), generics: vec![] },
                        name: "log_in_count".into(),
                    }),
                ].into(),
            }),
            ("Organization".into(), TypeDefinition {
                identifier: "Organization".into(),
                directives: vec![],
                fields
                    : [
                        ("id".into(), FieldDefinition {
                            directives: vec![],
                            input_definitions: HashMap::new(),
                            return_kind: Kind { name: "uuid".into(), generics: vec![] },
                            name: "id".into(),
                        }),
                        ("name".into(), FieldDefinition {
                            directives: vec![],
                            input_definitions: HashMap::new(),
                            return_kind: Kind { name: "String".into(), generics: vec![] },
                            name: "name".into(),
                        }),
                        ("industries".into(), FieldDefinition {
                            directives: vec![],
                            input_definitions: HashMap::new(),
                            return_kind: Kind { name: "Vec<String>".into(), generics: vec![] },
                            name: "industries".into(),
                        }),
                        ("users".into(), FieldDefinition {
                            directives: vec![],
                            input_definitions: HashMap::new(),
                            return_kind: Kind { name: "Vec<User>".into(), generics: vec![] },
                            name: "users".into(),
                        }),
                    ].into(),
            }),
        ].into(),
    };

    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual);
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

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: [
            ("Color".into(), EnumDefinition {
                name: "Color".into(),
                variants: vec![
                    VariantDefinition{
                        name: "Red".into(),
                        kind: VariantKindDefinition::Unit,
                        directives: vec![],
                    },
                    VariantDefinition{
                        name: "Green".into(),
                        kind: VariantKindDefinition::Unit,
                        directives: vec![],
                    },
                    VariantDefinition{
                        name: "Blue".into(),
                        kind: VariantKindDefinition::Unit,
                        directives: vec![],
                    },
                ],
                directives: vec![],
            }),
        ].into(),
        types: HashMap::new(),
    };

    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual);
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

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: [
            ("Color".into(), EnumDefinition {
                name: "Color".into(),
                variants: vec![
                    VariantDefinition{
                        name: "Red".into(),
                        kind: VariantKindDefinition::Unit,
                        directives: vec![],
                    },
                    VariantDefinition{
                        name: "Green".into(),
                        kind: VariantKindDefinition::Unit,
                        directives: vec![],
                    },
                    VariantDefinition{
                        name: "Blue".into(),
                        kind: VariantKindDefinition::Unit,
                        directives: vec![],
                    },
                ],
                directives: vec![],
            }),
            ("Emotion".into(), EnumDefinition {
                name: "Emotion".into(),
                variants: vec![
                    VariantDefinition{
                        name: "Happy".into(),
                        kind: VariantKindDefinition::Unit,
                        directives: vec![],
                    },
                    VariantDefinition{
                        name: "Sad".into(),
                        kind: VariantKindDefinition::Unit,
                        directives: vec![],
                    },
                    VariantDefinition{
                        name: "Angry".into(),
                        kind: VariantKindDefinition::Unit,
                        directives: vec![],
                    },
                    VariantDefinition{
                        name: "Fearful".into(),
                        kind: VariantKindDefinition::Unit,
                        directives: vec![],
                    },
                    VariantDefinition{
                        name: "Depressed".into(),
                        kind: VariantKindDefinition::Unit,
                        directives: vec![],
                    },
                ],
                directives: vec![],
            }),
        ].into(),
        types: [
            ("User".into(), TypeDefinition {
                identifier: "User".into(),
                directives: vec![],
                fields: [
                    ("id".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "uuid".into(), generics: vec![] },
                        name: "id".into(),
                    }),
                    ("name".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "String".into(), generics: vec![] },
                        name: "name".into(),
                    }),
                    ("age".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "Int".into(), generics: vec![] },
                        name: "age".into(),
                    }),
                    ("is_admin".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "bool".into(), generics: vec![] },
                        name: "is_admin".into(),
                    }),
                    ("location".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "String".into(), generics: vec![] },
                        name: "location".into(),
                    }),
                    ("log_in_count".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "Int".into(), generics: vec![] },
                        name: "log_in_count".into(),
                    }),
                ].into(),
            }),
        ].into(),
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

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: [
            ("FrameworkTypes".into(), EnumDefinition {
                name: "FrameworkTypes".into(),
                variants: vec![
                    VariantDefinition{
                        name: "ProfilePic".into(),
                        kind: VariantKindDefinition::Tuple(vec![Kind { name: "String".into(), generics: vec![] }]),
                        directives: vec![],
                    },
                    VariantDefinition{
                        name: "User".into(),
                        kind: VariantKindDefinition::Tuple(vec![Kind { name: "User".into(), generics: vec![] }]),
                        directives: vec![],
                    },
                    VariantDefinition{
                        name: "Organization".into(),
                        kind: VariantKindDefinition::Tuple(vec![Kind { name: "Organization".into(), generics: vec![] }]),
                        directives: vec![],
                    },
                ],
                directives: vec![],
            }),
        ].into(),
        types: [
            ("User".into(), TypeDefinition {
                identifier: "User".into(),
                directives: vec![],
                fields: [
                    ("id".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "uuid".into(), generics: vec![] },
                        name: "id".into(),
                    }),
                    ("name".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "String".into(), generics: vec![] },
                        name: "name".into(),
                    }),
                    ("age".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "Int".into(), generics: vec![] },
                        name: "age".into(),
                    }),
                    ("is_admin".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "bool".into(), generics: vec![] },
                        name: "is_admin".into(),
                    }),
                    ("location".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "String".into(), generics: vec![] },
                        name: "location".into(),
                    }),
                    ("log_in_count".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "Int".into(), generics: vec![] },
                        name: "log_in_count".into(),
                    }),
                ].into(),
            }),
            ("Organization".into(), TypeDefinition {
                identifier: "Organization".into(),
                directives: vec![],
                fields: [
                    ("id".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "uuid".into(), generics: vec![] },
                        name: "id".into(),
                    }),
                    ("name".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "String".into(), generics: vec![] },
                        name: "name".into(),
                    }),
                    ("industries".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "Vec<String>".into(), generics: vec![] },
                        name: "industries".into(),
                    }),
                    ("users".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "Vec<User>".into(), generics: vec![] },
                        name: "users".into(),
                    }),
                ].into(),
            }),
        ].into(),
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

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: [
            ("FrameworkTypes".into(), EnumDefinition {
                name: "FrameworkTypes".into(),
                variants: vec![
                    VariantDefinition{
                        name: "ProfilePic".into(),
                        kind: VariantKindDefinition::Tuple(vec![
                            Kind { name: "String".into(), generics: vec![] },
                            Kind { name: "String".into(), generics: vec![] },
                            Kind { name: "String".into(), generics: vec![] },
                        ]),
                        directives: vec![],
                    },
                ],
                directives: vec![],
            }),
        ].into(),
        types: HashMap::new(),
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

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: [
            ("FrameworkTypes".into(), EnumDefinition {
                name: "FrameworkTypes".into(),
                variants: vec![
                    VariantDefinition{
                        name: "User".into(),
                        kind: VariantKindDefinition::Map([
                            ("id".into(), Kind { name: "uuid".into(), generics: vec![] }),
                            ("name".into(), Kind { name: "String".into(), generics: vec![] }),
                            ("age".into(), Kind { name: "Int".into(), generics: vec![] }),
                        ].into()),
                        directives: vec![],
                    },
                    VariantDefinition{
                        name: "SomeOtherType".into(),
                        kind: VariantKindDefinition::Tuple(vec![
                            Kind { name: "String".into(), generics: vec![] },
                        ]),
                        directives: vec![],
                    },
                ],
                directives: vec![],
            }),
        ].into(),
        types: HashMap::new(),
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

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: HashMap::new(),
        types: [
            ("User".into(), TypeDefinition {
                identifier: "User".into(),
                directives: vec![],
                fields: [
                    ("id".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "uuid".into(), generics: vec![] },
                        name: "id".into(),
                    }),
                    ("name".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "Option".into(), generics: vec![Kind { name: "String".into(), generics: vec![]} ] },
                        name: "name".into(),
                    }),
                    ("profile_pic".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "Option".into(), generics: vec![Kind { name: "ProfilePic".into(), generics: vec![]}] },
                        name: "profile_pic".into(),
                    }),
                ].into(),
            }),
            ("ProfilePic".into(), TypeDefinition {
                identifier: "ProfilePic".into(),
                directives: vec![],
                fields: [
                    ("url".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "String".into(), generics: vec![] },
                        name: "url".into(),
                    }),
                    ("width".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "Int".into(), generics: vec![] },
                        name: "width".into(),
                    }),
                    ("height".into(), FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "Int".into(), generics: vec![] },
                        name: "height".into(),
                    }),
                ].into(),
            }),
        ].into(),
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

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: HashMap::new(),
        types: [
            ("Meow".into(), TypeDefinition {
                identifier: "Meow".into(),
                directives: vec![],
                fields: [
                    ("is_admin".into(), FieldDefinition {
                        directives: vec![
                            Directive {
                                name: "authenticated".into(),
                                inputs: [
                                    ("token".into(), Input::Variant(Variant{ident: "String".into(), value: VariantType::Unit})),
                                ].into(),
                            },
                            Directive {
                                name: "is_admin".into(),
                                inputs: [
                                    ("role".into(), Input::Variant(Variant{ident: "DoesntExist".into(), value: VariantType::Unit})),
                                ].into(),
                            },
                        ],
                        input_definitions: HashMap::new(),
                        return_kind: Kind { name: "bool".into(), generics: vec![] },
                        name: "is_admin".into(),
                    }),
                ].into(),
            }),
        ].into(),
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

    let expected = SchemaDefinition {
        directives: [
            ("authenticated".into(), DirectiveDefinition {
                name: "authenticated".into(),
                input_definitions: [
                    ("token".into(), Input::Variant(Variant{ident: "String".into(), value: VariantType::Unit})),
                ].into(),
            }),
            ("is_admin".into(), DirectiveDefinition {
                name: "is_admin".into(),
                input_definitions: [
                    ("role".into(), Input::Variant(Variant{ident: "DoesntExist".into(), value: VariantType::Unit})),
                ].into(),
            }),
        ].into(),
        enums: [
            ("Meow".into(), EnumDefinition {
                name: "Meow".into(),
                variants: vec![
                    VariantDefinition{
                        name: "Red".into(),
                        kind: VariantKindDefinition::Unit,
                        directives: vec![
                            Directive {
                                name: "authenticated".into(),
                                inputs: [
                                    ("token".into(), Input::Variant(Variant{ident: "String".into(), value: VariantType::Unit})),
                                ].into(),
                            },
                            Directive {
                                name: "is_admin".into(),
                                inputs: [
                                    ("role".into(), Input::Variant(Variant{ident: "DoesntExist".into(), value: VariantType::Unit})),
                                ].into(),
                            },
                        ],
                    },
                ],
                directives: vec![],
            }),
        ].into(),
        types: HashMap::new(),
    };

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
    directives.insert(Directive::new("authenticated".into(), HashMap::new()));
    directives.insert(Directive::new("uppercase".into(), upercase_argument));

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