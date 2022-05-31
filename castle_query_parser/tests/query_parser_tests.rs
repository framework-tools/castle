
use std::collections::HashMap;

use castle_query_parser::parse_message;
use castle_types::{Projection, Field, FieldKind, Input, Primitive};


#[test]
fn can_parse_empty_message() {
    let query = "";
    let expected: Projection = HashMap::new();
    let actual = &parse_message(query).expect("Failed to parse query").projection;
    assert_eq!(&expected, actual);
}

type Root = HashMap<Box<str>, Field>;

#[test]
fn can_parse_single_field() {
    let query = "message { first_name }";

    let expected = [
        ("first_name".into(), Field {
            name: "first_name".into(),
            inputs: HashMap::new(),
            rename: None,
            kind: FieldKind::Field,
        }),
    ].into_iter().collect::<Root>();

    let actual = &parse_message(query).expect("Failed to parse query").projection;
    assert_eq!(&expected, actual);
}

#[test]
fn can_parse_two_fields_with_empty_args() {
    let query = "message {
        first_name()
        last_name
        email()
    }";

    let expected = [
        ("first_name".into(), Field {
            name: "first_name".into(),
            inputs: HashMap::new(),
            rename: None,
            kind: FieldKind::Field,
        }),
        ("last_name".into(), Field {
            name: "last_name".into(),
            inputs: HashMap::new(),
            rename: None,
            kind: FieldKind::Field,
        }),
        ("email".into(), Field {
            name: "email".into(),
            inputs: HashMap::new(),
            rename: None,
            kind: FieldKind::Field,
        }),
    ].into_iter().collect::<Root>();

    let actual = &parse_message(query).expect("Failed to parse query").projection;
    assert_eq!(&expected, actual);
}

#[test]
fn can_parse_object_projection() {
    let query = "message {
        me {
            first_name
        }
    }";

    let expected: Root = [
        ("me".into(), Field {
            name: "me".into(),
            inputs: HashMap::new(),
            rename: None,
            kind: FieldKind::Object([
                ("first_name".into(), Field {
                    name: "first_name".into(),
                    inputs: HashMap::new(),
                    rename: None,
                    kind: FieldKind::Field,
                }),
            ].into()),
        }),
    ].into();
    let actual = &parse_message(query).expect("Failed to parse query").projection;
    assert_eq!(&expected, actual);
}

#[test]
fn can_parse_object_projection_with_two_fields() {
    let query = "message {
        me() {
            first_name,
            last_name
        }
    }";

    let expected = [
        ("me".into(), Field {
            name: "me".into(),
            inputs: HashMap::new(),
            rename: None,
            kind: FieldKind::Object([
                ("first_name".into(), Field {
                    name: "first_name".into(),
                    inputs: HashMap::new(),
                    rename: None,
                    kind: FieldKind::Field,
                }),
                ("last_name".into(), Field {
                    name: "last_name".into(),
                    inputs: HashMap::new(),
                    rename: None,
                    kind: FieldKind::Field,
                }),
            ].into_iter().collect::<Root>()),
        }),
    ].into_iter().collect::<Root>();

    let actual = &parse_message(query).expect("Failed to parse query").projection;
    assert_eq!(&expected, actual);
}

#[test]
fn query_with_two_separators_fails() {
    let query = "message {
        me() {
            first_name,,
            last_name
        }
    }";

    parse_message(query).unwrap_err();
}

#[test]
fn query_with_trailing_comma_succeeds() {
    let query = "message {
        me() {
            first_name,
            last_name,
        }
    }";

    let expected: Root = [
        ("me".into(), Field {
            name: "me".into(),
            inputs: HashMap::new(),
            rename: None,
            kind: FieldKind::Object([
                ("first_name".into(), Field {
                    name: "first_name".into(),
                    inputs: HashMap::new(),
                    rename: None,
                    kind: FieldKind::Field,
                }),
                ("last_name".into(), Field {
                    name: "last_name".into(),
                    inputs: HashMap::new(),
                    rename: None,
                    kind: FieldKind::Field,
                }),
            ].into()),
        }),
    ].into();

    let actual = &parse_message(query).expect("Expected success").projection;
    assert_eq!(&expected, actual);
}

#[test]
fn query_without_trailing_comma_succeeds() {
    let query = "message {
        me() {
            first_name
            last_name
        }
    }";

    let expected: Root = [
        ("me".into(), Field {
            name: "me".into(),
            inputs: HashMap::new(),
            rename: None,
            kind: FieldKind::Object([
                ("first_name".into(), Field {
                    name: "first_name".into(),
                    inputs: HashMap::new(),
                    rename: None,
                    kind: FieldKind::Field,
                }),
                ("last_name".into(), Field {
                    name: "last_name".into(),
                    inputs: HashMap::new(),
                    rename: None,
                    kind: FieldKind::Field,
                }),
            ].into()),
        }),
    ].into();

    let actual = &parse_message(query).expect("Expected success").projection;
    assert_eq!(&expected, actual);
}

#[test]
fn query_with_missing_closing_bracket_fails() {
    let query = "message {
        me() {
            first_name
        }";

    parse_message(query).unwrap_err();
}

#[test]
fn can_parse_object_and_single_field() {
    let query = "message {
        foo {
            bar as sdsd
            baz
        }
        xyz
    }";

    let expected: Root = [
        ("foo".into(), Field {
            name: "foo".into(),
            inputs: HashMap::new(),
            rename: None,
            kind: FieldKind::Object([
                ("bar".into(), Field {
                    name: "bar".into(),
                    inputs: HashMap::new(),
                    rename: Some("sdsd".into()),
                    kind: FieldKind::Field,
                }),
                ("baz".into(), Field {
                    name: "baz".into(),
                    inputs: HashMap::new(),
                    rename: None,
                    kind: FieldKind::Field,
                }),
            ].into()),
        }),
        ("xyz".into(), Field {
            name: "xyz".into(),
            inputs: HashMap::new(),
            rename: None,
            kind: FieldKind::Field,
        }),
    ].into();

    let actual = &parse_message(query).expect("Failed to parse query").projection;
    assert_eq!(&expected, actual);
}

#[test]
fn can_parse_numeric_argument() {
    let query = "message {
        profile_picture(size: 48)
    }";

    let expected: Root = [
        ("profile_picture".into(), Field {
            name: "profile_picture".into(),
            inputs: [("size".into(), Input::Primitive(Primitive::Number(48.into())))].into(),
            rename: None,
            kind: FieldKind::Field,
        }),
    ].into();

    let actual = &parse_message(query).expect("Expected query to parse").projection;
    assert_eq!(&expected, actual);
}

#[test]
fn can_parse_multiple_numeric_arguments() {
    let query = "message {
        profile_picture(size: 48, width: 100)
    }";

    let expected: Root = [
        ("profile_picture".into(), Field {
            name: "profile_picture".into(),
            inputs: [
                ("size".into(), Input::Primitive(Primitive::Number(48.into()))),
                ("width".into(), Input::Primitive(Primitive::Number(100.into()))),
            ].into(),
            rename: None,
            kind: FieldKind::Field,
        }),
    ].into();

    let actual = &parse_message(query).expect("Expected query to parse").projection;
    assert_eq!(&expected, actual);
}

#[test]
fn can_parse_string_arguments() {
    let query = "message {
        profile_picture(size: \"48\")
    }";

    let expected: Root = [
        ("profile_picture".into(), Field {
            name: "profile_picture".into(),
            inputs: [("size".into(), Input::Primitive(Primitive::String("48".into())))].into(),
            rename: None,
            kind: FieldKind::Field,
        }),
    ].into();

    let actual = &parse_message(query).expect("Expected query to parse").projection;
    assert_eq!(&expected, actual);
}

#[test]
fn can_parse_boolean_arguments() {
    let query = "message {
        foo(a: true, b: false)
    }";

    let expected: Root = [
        ("foo".into(), Field {
            name: "foo".into(),
            inputs: [
                ("a".into(), Input::Primitive(Primitive::Boolean(true))),
                ("b".into(), Input::Primitive(Primitive::Boolean(false))),
            ].into(),
            rename: None,
            kind: FieldKind::Field,
        }),
    ].into();

    let actual = &parse_message(query).expect("Expected query to parse").projection;
    assert_eq!(&expected, actual);
}

#[test]
fn can_parse_deeply_nested_message() {
    let query = "message {
        me() {
            first_name
            last_name
            profile_picture(size: 48) {
                url
            }
        }
    }";

    let expected: Root = [
        ("me".into(), Field {
            name: "me".into(),
            inputs: HashMap::new(),
            rename: None,
            kind: FieldKind::Object([
                ("first_name".into(), Field {
                    name: "first_name".into(),
                    inputs: HashMap::new(),
                    rename: None,
                    kind: FieldKind::Field,
                }),
                ("last_name".into(), Field {
                    name: "last_name".into(),
                    inputs: HashMap::new(),
                    rename: None,
                    kind: FieldKind::Field,
                }),
                ("profile_picture".into(), Field {
                    name: "profile_picture".into(),
                    inputs: [("size".into(), Input::Primitive(Primitive::Number(48.into())))].into(),
                    rename: None,
                    kind: FieldKind::Object([
                        ("url".into(), Field {
                            name: "url".into(),
                            inputs: HashMap::new(),
                            rename: None,
                            kind: FieldKind::Field,
                        }),
                    ].into()),
                }),
            ].into()),
        }),
    ].into();

    let actual = &parse_message(query).expect("Expected query to parse").projection;
    assert_eq!(&expected, actual);
}

#[test]
fn can_parse_object_argument() {
    let query = "message {
        create_user(user: {
            first_name: \"John\"
            last_name: \"Doe\"
        })
    }
    ";

    let expected: Root = [
        ("create_user".into(), Field {
            name: "create_user".into(),
            inputs: [("user".into(), Input::Map(
                [
                    ("first_name".into(), Input::Primitive(Primitive::String("John".into()))),
                    ("last_name".into(), Input::Primitive(Primitive::String("Doe".into()))),
                ].into()
            ))].into(),
            rename: None,
            kind: FieldKind::Field,
        }),
    ].into();

    let actual = &parse_message(query).expect("Expected query to parse").projection;
    assert_eq!(&expected, actual);
}

#[test]
fn can_parse_array_arguments() {
    let query = "message {
        create_user(user: [
            {
                first_name: \"John\"
                last_name: \"Doe\"
            },
            {
                first_name: \"Jane\"
                last_name: \"Doe\"
            }
        ])
    }
    ";

    let expected: Root = [
        ("create_user".into(), Field {
            name: "create_user".into(),
            inputs: [("user".into(), Input::List(
                [
                    Input::Map(
                        [
                            ("first_name".into(), Input::Primitive(Primitive::String("John".into()))),
                            ("last_name".into(), Input::Primitive(Primitive::String("Doe".into()))),
                        ].into()
                    ),
                    Input::Map(
                        [
                            ("first_name".into(), Input::Primitive(Primitive::String("Jane".into()))),
                            ("last_name".into(), Input::Primitive(Primitive::String("Doe".into()))),
                        ].into()
                    ),
                ].into()
            ))].into(),
            rename: None,
            kind: FieldKind::Field,
        }),
    ].into();

    let actual = &parse_message(query).expect("Expected query to parse").projection;
    assert_eq!(&expected, actual);
}


#[test]
fn arg_with_no_ident_fails() {
    let query = "message {
        ()
    }
    ";

    parse_message(query).unwrap_err();
}
#[test]
fn can_parse_resolver_with_no_fields () {
    let query = "message {
        me {

        }
    }";

    let expected: Root = [
        ("me".into(), Field {
            name: "me".into(),
            inputs: HashMap::new(),
            rename: None,
            kind: FieldKind::Object(HashMap::new()),
        }),
    ].into();

    let actual = &parse_message(query).expect("Expected query to parse").projection;
    assert_eq!(&expected, actual);
}

// #[test]
// fn can_parse_object_projection_with_match() {
//     let query = "
//         me() {
//             first_name
//             last_name
//             email
//             profile_picture(size: 48)
//             icon() match {
//                 Icon::Svg => icon() {
//                     url,
//                     size
//                 },
//                 Icon::Emoji => emoji() {
//                     emoji,
//                     size
//                 }
//             }
//         }
//     ";

//     let mut profile_picture_argument = HashMap::new();
//     profile_picture_argument.insert("size".into(), PrimitiveValue::UInt(48));

//     let svg_fields = insert_each_field_into_fields(vec![
//         ("url".into(), Want::new_single_field(HashMap::new())),
//         ("size".into(), Want::new_single_field(HashMap::new())),
//     ]);

//     let svg_match_arm = MatchArm::new(
//         EnumValue { identifier: "Icon::Svg".into(), enum_parent: "Icon".into(), variant: "Svg".into(), data_type: EnumDataType::EnumUnit },
//     "icon".into(),
//         Want::new_object_projection(svg_fields, HashMap::new()));

//     let emoji_fields = insert_each_field_into_fields(vec![
//         ("emoji".into(), Want::new_single_field(HashMap::new())),
//         ("size".into(), Want::new_single_field(HashMap::new())),
//     ]);

//     let emoji_match_arms = MatchArm::new(
//             EnumValue { identifier: "Icon::Emoji".into(), enum_parent: "Icon".into(), variant: "Emoji".into(), data_type: EnumDataType::EnumUnit },
//             "emoji".into(),
//             Want::ObjectProjection(emoji_fields, HashMap::new()));

//     let match_statement = vec![
//         svg_match_arm,
//         emoji_match_arms,
//     ];

//     let fields = insert_each_field_into_fields(vec![
//         ("first_name".into(), Want::new_single_field(HashMap::new())),
//         ("last_name".into(), Want::new_single_field(HashMap::new())),
//         ("email".into(), Want::new_single_field(HashMap::new())),
//         ("profile_picture".into(), Want::new_single_field(profile_picture_argument)),
//         ("icon".into(), Want::new_match(match_statement)),
//     ]);

//     let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
//         ("me".into(), Want::new_object_projection(fields, HashMap::new())),
//     ]);
//     let actual = &parse_message(query).unwrap();
//     assert_eq!(&expected, actual.wants);
// }

// #[test]
// fn can_parse_object_projection_with_match_inside_match() {
//     let query = "
//         me() {
//             icon() match {
//                 Icon::Svg => icon() {
//                     url(size: 48)
//                     size() match {
//                         Size::Rectangle => rectangle() {
//                             width
//                             height
//                         },
//                         Size::Square => square() {
//                             side
//                         }
//                     }
//                 },
//                 Icon::Emoji => emoji() {
//                     emoji
//                     lol
//                 }
//             }
//         }
//     ";

//     let mut url_argument = HashMap::new();
//     url_argument.insert("size".into(), PrimitiveValue::UInt(48));

//     let rectangle_fields = insert_each_field_into_fields(vec![
//         ("width".into(), Want::new_single_field(HashMap::new())),
//         ("height".into(), Want::new_single_field(HashMap::new())),
//     ]);

//     let square_fields = insert_each_field_into_fields(vec![
//         ("side".into(), Want::new_single_field(HashMap::new())),
//     ]);

//     let size_match = vec![
//         MatchArm::new(
//             EnumValue { identifier: "Size::Rectangle".into(), enum_parent: "Size".into(), variant: "Rectangle".into(), data_type: EnumDataType::EnumUnit },
//             "rectangle".into(),
//             Want::new_object_projection(rectangle_fields, HashMap::new())
//         ),
//         MatchArm::new(
//             EnumValue { identifier: "Size::Square".into(), enum_parent: "Size".into(), variant: "Square".into(), data_type: EnumDataType::EnumUnit },
//             "square".into(),
//             Want::new_object_projection(square_fields, HashMap::new()),
//         ),
//     ];

//     let svg_fields = insert_each_field_into_fields(vec![
//         ("url".into(), Want::new_single_field(url_argument)),
//         ("size".into(), Want::new_match(size_match)),
//     ]);

//     let emoji_fields = insert_each_field_into_fields(vec![
//         ("emoji".into(), Want::new_single_field(HashMap::new())),
//         ("lol".into(), Want::new_single_field(HashMap::new())),
//     ]);

//     let icon_match = vec![
//         MatchArm::new(
//             EnumValue { identifier: "Icon::Svg".into(), enum_parent: "Icon".into(), variant: "Svg".into(), data_type: EnumDataType::EnumUnit },
//             "icon".into(),
//             Want::new_object_projection(svg_fields, HashMap::new()),
//         ),
//         MatchArm::new(
//             EnumValue { identifier: "Icon::Emoji".into(), enum_parent: "Icon".into(), variant: "Emoji".into(), data_type: EnumDataType::EnumUnit },
//             "emoji".into(),
//             Want::new_object_projection(emoji_fields, HashMap::new()),
//         ),
//     ];

//     let fields = insert_each_field_into_fields(vec![
//         ("icon".into(), Want::new_match(icon_match)),
//     ]);

//     let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
//         ("me".into(), Want::new_object_projection(fields, HashMap::new())),
//     ]);
//     let actual = &parse_message(query).unwrap();
//     assert_eq!(&expected, actual.wants);
// }
