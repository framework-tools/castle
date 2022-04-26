use std::collections::HashMap;

use schema_parser::{
    parsers::parse_schema::parse_schema,
    types::{
        Directive, DirectiveDefinition, DirectiveLocation, EnumDefinition, FieldDefinition,
        InputDefinition, Kind, SchemaDefinition, TypeDefinition, VariantDefinition,
        VariantKindDefinition,
    },
};
use shared_parser::Input;
use tokenizer::Primitive;

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
        types: [(
            "User".into(),
            TypeDefinition {
                ident: "User".into(),
                directives: vec![],
                fields: [(
                    "id".into(),
                    FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind {
                            name: "uuid".into(),
                            generics: vec![],
                        },
                        name: "id".into(),
                    },
                )]
                .into(),
            },
        )]
        .into(),
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
        types: [(
            "User".into(),
            TypeDefinition {
                ident: "User".into(),
                directives: vec![],
                fields: [
                    (
                        "id".into(),
                        FieldDefinition {
                            directives: vec![],
                            input_definitions: HashMap::new(),
                            return_kind: Kind {
                                name: "uuid".into(),
                                generics: vec![],
                            },
                            name: "id".into(),
                        },
                    ),
                    (
                        "name".into(),
                        FieldDefinition {
                            directives: vec![],
                            input_definitions: HashMap::new(),
                            return_kind: Kind {
                                name: "String".into(),
                                generics: vec![],
                            },
                            name: "name".into(),
                        },
                    ),
                    (
                        "age".into(),
                        FieldDefinition {
                            directives: vec![],
                            input_definitions: HashMap::new(),
                            return_kind: Kind {
                                name: "Int".into(),
                                generics: vec![],
                            },
                            name: "age".into(),
                        },
                    ),
                ]
                .into(),
            },
        )]
        .into(),
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
            (
                "User".into(),
                TypeDefinition {
                    ident: "User".into(),
                    directives: vec![],
                    fields: [
                        (
                            "id".into(),
                            FieldDefinition {
                                directives: vec![],
                                input_definitions: HashMap::new(),
                                return_kind: Kind {
                                    name: "uuid".into(),
                                    generics: vec![],
                                },
                                name: "id".into(),
                            },
                        ),
                        (
                            "name".into(),
                            FieldDefinition {
                                directives: vec![],
                                input_definitions: HashMap::new(),
                                return_kind: Kind {
                                    name: "String".into(),
                                    generics: vec![],
                                },
                                name: "name".into(),
                            },
                        ),
                        (
                            "age".into(),
                            FieldDefinition {
                                directives: vec![],
                                input_definitions: HashMap::new(),
                                return_kind: Kind {
                                    name: "Int".into(),
                                    generics: vec![],
                                },
                                name: "age".into(),
                            },
                        ),
                        (
                            "is_admin".into(),
                            FieldDefinition {
                                directives: vec![],
                                input_definitions: HashMap::new(),
                                return_kind: Kind {
                                    name: "bool".into(),
                                    generics: vec![],
                                },
                                name: "is_admin".into(),
                            },
                        ),
                        (
                            "location".into(),
                            FieldDefinition {
                                directives: vec![],
                                input_definitions: HashMap::new(),
                                return_kind: Kind {
                                    name: "String".into(),
                                    generics: vec![],
                                },
                                name: "location".into(),
                            },
                        ),
                        (
                            "log_in_count".into(),
                            FieldDefinition {
                                directives: vec![],
                                input_definitions: HashMap::new(),
                                return_kind: Kind {
                                    name: "Int".into(),
                                    generics: vec![],
                                },
                                name: "log_in_count".into(),
                            },
                        ),
                    ]
                    .into(),
                },
            ),
            (
                "Organization".into(),
                TypeDefinition {
                    ident: "Organization".into(),
                    directives: vec![],
                    fields: [
                        (
                            "id".into(),
                            FieldDefinition {
                                directives: vec![],
                                input_definitions: HashMap::new(),
                                return_kind: Kind {
                                    name: "uuid".into(),
                                    generics: vec![],
                                },
                                name: "id".into(),
                            },
                        ),
                        (
                            "name".into(),
                            FieldDefinition {
                                directives: vec![],
                                input_definitions: HashMap::new(),
                                return_kind: Kind {
                                    name: "String".into(),
                                    generics: vec![],
                                },
                                name: "name".into(),
                            },
                        ),
                        (
                            "industry".into(),
                            FieldDefinition {
                                directives: vec![],
                                input_definitions: HashMap::new(),
                                return_kind: Kind {
                                    name: "String".into(),
                                    generics: vec![],
                                },
                                name: "industry".into(),
                            },
                        ),
                    ]
                    .into(),
                },
            ),
        ]
        .into(),
    };

    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_types_with_two_vecs() {
    use std::collections::HashMap;

    let query = "
        type Organization {
            industries: Vec<String>,
            related_orgs: Vec<Organisation>
        }";

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: HashMap::new(),
        types: [
            (
                "Organization".into(),
                TypeDefinition {
                    ident: "Organization".into(),
                    directives: vec![],
                    fields: [
                        (
                            "industries".into(),
                            FieldDefinition {
                                directives: vec![],
                                input_definitions: HashMap::new(),
                                return_kind: Kind {
                                    name: "Vec".into(),
                                    generics: vec![
                                        Kind {
                                            name: "String".into(),
                                            generics: vec![],
                                        },
                                    ],
                                },
                                name: "industries".into(),
                            },
                        ),
                        (
                            "related_orgs".into(),
                            FieldDefinition {
                                directives: vec![],
                                input_definitions: HashMap::new(),
                                return_kind: Kind {
                                    name: "Vec".into(),
                                    generics: vec![
                                        Kind {
                                            name: "Organisation".into(),
                                            generics: vec![],
                                        },
                                    ],
                                },
                                name: "related_orgs".into(),
                            },
                        ),
                    ]
                    .into(),
                },
            ),
        ]
        .into(),
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
        enums: [(
            "Color".into(),
            EnumDefinition {
                ident: "Color".into(),
                variants: [
                    (
                        "Red".into(),
                        VariantDefinition {
                            ident: "Red".into(),
                            kind: VariantKindDefinition::Unit,
                            directives: vec![],
                        },
                    ),
                    (
                        "Green".into(),
                        VariantDefinition {
                            ident: "Green".into(),
                            kind: VariantKindDefinition::Unit,
                            directives: vec![],
                        },
                    ),
                    (
                        "Blue".into(),
                        VariantDefinition {
                            ident: "Blue".into(),
                            kind: VariantKindDefinition::Unit,
                            directives: vec![],
                        },
                    ),
                ]
                .into(),
                directives: vec![],
            },
        )]
        .into(),
        types: HashMap::new(),
    };

    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_enum_and_type() {
    let query = "
        enum Color {
            Red,
            Green,
            Blue
        }

        type User {
            first_name: String
        }
    ";

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: [(
            "Color".into(),
            EnumDefinition {
                ident: "Color".into(),
                variants: [
                    (
                        "Red".into(),
                        VariantDefinition {
                            ident: "Red".into(),
                            kind: VariantKindDefinition::Unit,
                            directives: vec![],
                        },
                    ),
                    (
                        "Green".into(),
                        VariantDefinition {
                            ident: "Green".into(),
                            kind: VariantKindDefinition::Unit,
                            directives: vec![],
                        },
                    ),
                    (
                        "Blue".into(),
                        VariantDefinition {
                            ident: "Blue".into(),
                            kind: VariantKindDefinition::Unit,
                            directives: vec![],
                        },
                    ),
                ]
                .into(),
                directives: vec![],
            },
        )]
        .into(),
        types: [(
            "User".into(),
            TypeDefinition {
                ident: "User".into(),
                directives: vec![],
                fields: [
                    (
                        "first_name".into(),
                        FieldDefinition {
                            directives: vec![],
                            input_definitions: HashMap::new(),
                            return_kind: Kind {
                                name: "String".into(),
                                generics: vec![],
                            },
                            name: "first_name".into(),
                        },
                    ),
                ].into(),
            },
        )]
        .into(),
    };

    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_enum_with_tuple_variant() {
    use std::collections::HashMap;

    let query = "
        enum Example {
            Unit1
            Tuple(String),
            Tuple2(String, String)
            Unit2
        }
        ";

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: [(
            "Example".into(),
            EnumDefinition {
                ident: "Example".into(),
                variants: [
                    (
                        "Unit1".into(),
                        VariantDefinition {
                            ident: "Unit1".into(),
                            kind: VariantKindDefinition::Unit,
                            directives: vec![],
                        },
                    ),
                    (
                        "Tuple".into(),
                        VariantDefinition {
                            ident: "Tuple".into(),
                            kind: VariantKindDefinition::Tuple(vec![Kind {
                                name: "String".into(),
                                generics: vec![],
                            }]),
                            directives: vec![],
                        },
                    ),
                    (
                        "Tuple2".into(),
                        VariantDefinition {
                            ident: "Tuple2".into(),
                            kind: VariantKindDefinition::Tuple(vec![
                                Kind {
                                    name: "String".into(),
                                    generics: vec![],
                                },
                                Kind {
                                    name: "String".into(),
                                    generics: vec![],
                                },
                            ]),
                            directives: vec![],
                        },
                    ),
                    (
                        "Unit2".into(),
                        VariantDefinition {
                            ident: "Unit2".into(),
                            kind: VariantKindDefinition::Unit,
                            directives: vec![],
                        },
                    ),
                ]
                .into(),
                directives: vec![],
            },
        )]
        .into(),
        types: HashMap::new(),
    };

    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_enum_with_fields() {
    let schema = "
        enum FrameworkTypes {
            User {
                id: uuid,
                name: String,
                age: Int,
            },
            Unit,
        }
    ";

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: [(
            "FrameworkTypes".into(),
            EnumDefinition {
                ident: "FrameworkTypes".into(),
                variants: [
                    (
                        "User".into(),
                        VariantDefinition {
                            ident: "User".into(),
                            kind: VariantKindDefinition::Map(
                                [
                                    (
                                        "id".into(),
                                        Kind {
                                            name: "uuid".into(),
                                            generics: vec![],
                                        },
                                    ),
                                    (
                                        "name".into(),
                                        Kind {
                                            name: "String".into(),
                                            generics: vec![],
                                        },
                                    ),
                                    (
                                        "age".into(),
                                        Kind {
                                            name: "Int".into(),
                                            generics: vec![],
                                        },
                                    ),
                                ]
                                .into(),
                            ),
                            directives: vec![],
                        },
                    ),
                    (
                        "Unit".into(),
                        VariantDefinition {
                            ident: "Unit".into(),
                            kind: VariantKindDefinition::Unit,
                            directives: vec![],
                        },
                    ),
                ]
                .into(),
                directives: vec![],
            },
        )]
        .into(),
        types: HashMap::new(),
    };

    let actual = parse_schema(schema).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_generic_field_type() {
    let schema = "
        type User {
            first_name: Option<String>,
            last_name: Option<String>,
        }
    ";

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: HashMap::new(),
        types: [(
            "User".into(),
            TypeDefinition {
                ident: "User".into(),
                directives: vec![],
                fields: [
                    (
                        "first_name".into(),
                        FieldDefinition {
                            directives: vec![],
                            input_definitions: HashMap::new(),
                            return_kind: Kind {
                                name: "Option".into(),
                                generics: vec![Kind {
                                    name: "String".into(),
                                    generics: vec![],
                                }],
                            },
                            name: "first_name".into(),
                        },
                    ),
                    (
                        "last_name".into(),
                        FieldDefinition {
                            directives: vec![],
                            input_definitions: HashMap::new(),
                            return_kind: Kind {
                                name: "Option".into(),
                                generics: vec![Kind {
                                    name: "String".into(),
                                    generics: vec![],
                                }],
                            },
                            name: "last_name".into(),
                        },
                    ),
                ]
                .into(),
            },
        )]
        .into(),
    };
    assert_eq!(expected, parse_schema(schema).unwrap());
}

#[test]
fn can_parse_directives_on_fields() {
    let schema = "
        type Test {
            is_admin: bool @bar(baz: 123) @foo
        }
    ";

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: HashMap::new(),
        types: [(
            "Test".into(),
            TypeDefinition {
                ident: "Test".into(),
                directives: vec![],
                fields: [(
                    "is_admin".into(),
                    FieldDefinition {
                        directives: vec![
                            Directive {
                                name: "bar".into(),
                                inputs: [("baz".into(), Input::Primitive(Primitive::UInt(123)))]
                                    .into(),
                            },
                            Directive {
                                name: "foo".into(),
                                inputs: HashMap::new(),
                            },
                        ],
                        input_definitions: HashMap::new(),
                        return_kind: Kind {
                            name: "bool".into(),
                            generics: vec![],
                        },
                        name: "is_admin".into(),
                    },
                )]
                .into(),
            },
        )]
        .into(),
    };

    assert_eq!(expected, parse_schema(schema).unwrap());
}

#[test]
fn can_parse_directive_definitions() {
    let schema = "
        directive @authenticated(token: String) on EnumDefinition
        directive @is_admin on InputDefinition
    ";

    let expected = SchemaDefinition {
        directives: [
            (
                "authenticated".into(),
                DirectiveDefinition {
                    name: "authenticated".into(),
                    input_definitions: [(
                        "token".into(),
                        InputDefinition {
                            default: None,
                            directives: vec![],
                            input_kind: Kind {
                                name: "String".into(),
                                generics: vec![],
                            },
                            name: "token".into(),
                        },
                    )]
                    .into(),
                    locations: [DirectiveLocation::EnumDefinition].into(),
                },
            ),
            (
                "is_admin".into(),
                DirectiveDefinition {
                    name: "is_admin".into(),
                    input_definitions: HashMap::new(),
                    locations: [DirectiveLocation::InputDefinition].into(),
                },
            ),
        ]
        .into(),
        enums: HashMap::new(),
        types: HashMap::new(),
    };

    let actual = parse_schema(schema).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_comments() {
    let schema = "
    # This is a comment
    type Foo {
        # This is a comment
        bar: String
    }
    ";

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: HashMap::new(),
        types: [(
            "Foo".into(),
            TypeDefinition {
                ident: "Foo".into(),
                directives: vec![],
                fields: [(
                    "bar".into(),
                    FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind {
                            name: "String".into(),
                            generics: vec![],
                        },
                        name: "bar".into(),
                    },
                )]
                .into(),
            },
        )]
        .into(),
    };

    let actual = parse_schema(schema).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn generic_inside_generic_works() {
    let schema = "
        type User {
            pets: Option<Vec<String>>
        }
    ";

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: HashMap::new(),
        types: [(
            "User".into(),
            TypeDefinition {
                ident: "User".into(),
                directives: vec![],
                fields: [(
                    "pets".into(),
                    FieldDefinition {
                        directives: vec![],
                        input_definitions: HashMap::new(),
                        return_kind: Kind {
                            name: "Option".into(),
                            generics: vec![Kind {
                                name: "Vec".into(),
                                generics: vec![Kind {
                                    name: "String".into(),
                                    generics: vec![],
                                }],
                            }],
                        },
                        name: "pets".into(),
                    },
                )]
                .into(),
            },
        )]
        .into(),
    };

    let actual = parse_schema(schema).unwrap();
    assert_eq!(expected, actual);
}
