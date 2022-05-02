
use std::collections::HashMap;

use schema_parser::{
    parsers::parse_schema::parse_schema,
    types::{
        Directive, DirectiveDefinition, DirectiveLocation, EnumDefinition, FieldDefinition,
        InputDefinition, Kind, SchemaDefinition, TypeDefinition, VariantDefinition,
        VariantKindDefinition, InputTypeDefinition,
    },
};
use shared_parser::Input;
use tokenizer::{Primitive, Number};

#[test]
fn can_parse_empty_message() {
    let query = "";
    let expected = SchemaDefinition::new();
    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_simple_type() {
    let query = "
        type User {
            id: Uuid,
        }";

    let expected = SchemaDefinition {
        input_types: HashMap::new(),
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
                            ident: "Uuid".into(),
                            generics: vec![],
                        },
                        ident: "id".into(),
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
            id: Uuid
            name: String
            age: number
        }";

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        input_types: HashMap::new(),
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
                                ident: "Uuid".into(),
                                generics: vec![],
                            },
                            ident: "id".into(),
                        },
                    ),
                    (
                        "name".into(),
                        FieldDefinition {
                            directives: vec![],
                            input_definitions: HashMap::new(),
                            return_kind: Kind {
                                ident: "String".into(),
                                generics: vec![],
                            },
                            ident: "name".into(),
                        },
                    ),
                    (
                        "age".into(),
                        FieldDefinition {
                            directives: vec![],
                            input_definitions: HashMap::new(),
                            return_kind: Kind {
                                ident: "number".into(),
                                generics: vec![],
                            },
                            ident: "age".into(),
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
            id: Uuid,
            name: String,
            age: number
        }

        type Organization {
            id: Uuid,
            name: String,
            industry: String,
        }";

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        input_types: HashMap::new(),
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
                                    ident: "Uuid".into(),
                                    generics: vec![],
                                },
                                ident: "id".into(),
                            },
                        ),
                        (
                            "name".into(),
                            FieldDefinition {
                                directives: vec![],
                                input_definitions: HashMap::new(),
                                return_kind: Kind {
                                    ident: "String".into(),
                                    generics: vec![],
                                },
                                ident: "name".into(),
                            },
                        ),
                        (
                            "age".into(),
                            FieldDefinition {
                                directives: vec![],
                                input_definitions: HashMap::new(),
                                return_kind: Kind {
                                    ident: "number".into(),
                                    generics: vec![],
                                },
                                ident: "age".into(),
                            },
                        ),
                    ].into(),
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
                                    ident: "Uuid".into(),
                                    generics: vec![],
                                },
                                ident: "id".into(),
                            },
                        ),
                        (
                            "name".into(),
                            FieldDefinition {
                                directives: vec![],
                                input_definitions: HashMap::new(),
                                return_kind: Kind {
                                    ident: "String".into(),
                                    generics: vec![],
                                },
                                ident: "name".into(),
                            },
                        ),
                        (
                            "industry".into(),
                            FieldDefinition {
                                directives: vec![],
                                input_definitions: HashMap::new(),
                                return_kind: Kind {
                                    ident: "String".into(),
                                    generics: vec![],
                                },
                                ident: "industry".into(),
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
        input_types: HashMap::new(),
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
                                    ident: "Vec".into(),
                                    generics: vec![
                                        Kind {
                                            ident: "String".into(),
                                            generics: vec![],
                                        },
                                    ],
                                },
                                ident: "industries".into(),
                            },
                        ),
                        (
                            "related_orgs".into(),
                            FieldDefinition {
                                directives: vec![],
                                input_definitions: HashMap::new(),
                                return_kind: Kind {
                                    ident: "Vec".into(),
                                    generics: vec![
                                        Kind {
                                            ident: "Organisation".into(),
                                            generics: vec![],
                                        },
                                    ],
                                },
                                ident: "related_orgs".into(),
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
        input_types: HashMap::new(),
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
        input_types: HashMap::new(),
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
                                ident: "String".into(),
                                generics: vec![],
                            },
                            ident: "first_name".into(),
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
        input_types: HashMap::new(),
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
                                ident: "String".into(),
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
                                    ident: "String".into(),
                                    generics: vec![],
                                },
                                Kind {
                                    ident: "String".into(),
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
                id: Uuid,
                name: String,
                age: number,
            },
            Unit,
        }
    ";

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        input_types: HashMap::new(),
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
                                            ident: "Uuid".into(),
                                            generics: vec![],
                                        },
                                    ),
                                    (
                                        "name".into(),
                                        Kind {
                                            ident: "String".into(),
                                            generics: vec![],
                                        },
                                    ),
                                    (
                                        "age".into(),
                                        Kind {
                                            ident: "number".into(),
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
        input_types: HashMap::new(),
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
                                ident: "Option".into(),
                                generics: vec![Kind {
                                    ident: "String".into(),
                                    generics: vec![],
                                }],
                            },
                            ident: "first_name".into(),
                        },
                    ),
                    (
                        "last_name".into(),
                        FieldDefinition {
                            directives: vec![],
                            input_definitions: HashMap::new(),
                            return_kind: Kind {
                                ident: "Option".into(),
                                generics: vec![Kind {
                                    ident: "String".into(),
                                    generics: vec![],
                                }],
                            },
                            ident: "last_name".into(),
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
        input_types: HashMap::new(),
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
                                ident: "bar".into(),
                                inputs: [("baz".into(), Input::Primitive(Primitive::Number(Number::from(123))))]
                                    .into(),
                            },
                            Directive {
                                ident: "foo".into(),
                                inputs: HashMap::new(),
                            },
                        ],
                        input_definitions: HashMap::new(),
                        return_kind: Kind {
                            ident: "bool".into(),
                            generics: vec![],
                        },
                        ident: "is_admin".into(),
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
        input_types: HashMap::new(),
        directives: [
            (
                "authenticated".into(),
                DirectiveDefinition {
                    ident: "authenticated".into(),
                    input_definitions: [(
                        "token".into(),
                        InputDefinition {
                            default: None,
                            directives: vec![],
                            input_kind: Kind {
                                ident: "String".into(),
                                generics: vec![],
                            },
                            ident: "token".into(),
                        },
                    )]
                    .into(),
                    locations: [DirectiveLocation::EnumDefinition].into(),
                },
            ),
            (
                "is_admin".into(),
                DirectiveDefinition {
                    ident: "is_admin".into(),
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
        input_types: HashMap::new(),
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
                            ident: "String".into(),
                            generics: vec![],
                        },
                        ident: "bar".into(),
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
        input_types: HashMap::new(),
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
                            ident: "Option".into(),
                            generics: vec![Kind {
                                ident: "Vec".into(),
                                generics: vec![Kind {
                                    ident: "String".into(),
                                    generics: vec![],
                                }],
                            }],
                        },
                        ident: "pets".into(),
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
fn can_parse_input_type() {
    let schema = "
        input Test {
            foo: String
        }
    ";

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: HashMap::new(),
        types: HashMap::new(),
        input_types: [
            ("Test".into(), InputTypeDefinition {
                ident: "Test".into(),
                directives: vec![],
                input_definitions: [
                    ("foo".into(), InputDefinition {
                        ident: "foo".into(),
                        default: None,
                        directives: vec![],
                        input_kind: Kind {
                            ident: "String".into(),
                            generics: vec![],
                        },
                    }),
                ].into(),
            }),
        ].into(),
    };

    let actual = parse_schema(schema).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_type_with_field_args() {
    let schema = "
        type Test {
            foo(bar: String): String
        }
    ";

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: HashMap::new(),
        input_types: HashMap::new(),
        types: [(
            "Test".into(),
            TypeDefinition {
                ident: "Test".into(),
                directives: vec![],
                fields: [(
                    "foo".into(),
                    FieldDefinition {
                        directives: vec![],
                        input_definitions: [("bar".into(), InputDefinition {
                            ident: "bar".into(),
                            default: None,
                            directives: vec![],
                            input_kind: Kind {
                                ident: "String".into(),
                                generics: vec![],
                            },
                        })]
                        .into(),
                        return_kind: Kind {
                            ident: "String".into(),
                            generics: vec![],
                        },
                        ident: "foo".into(),
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
fn can_parse_type_with_custom_input_type() {
    let schema = "
        type Test {
            foo(bar: Xyz): String
        }
    ";

    let expected = SchemaDefinition {
        directives: HashMap::new(),
        enums: HashMap::new(),
        input_types: HashMap::new(),
        types: [(
            "Test".into(),
            TypeDefinition {
                ident: "Test".into(),
                directives: vec![],
                fields: [(
                    "foo".into(),
                    FieldDefinition {
                        directives: vec![],
                        input_definitions: [("bar".into(), InputDefinition {
                            ident: "bar".into(),
                            default: None,
                            directives: vec![],
                            input_kind: Kind {
                                ident: "Xyz".into(),
                                generics: vec![],
                            },
                        })]
                        .into(),
                        return_kind: Kind {
                            ident: "String".into(),
                            generics: vec![],
                        },
                        ident: "foo".into(),
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