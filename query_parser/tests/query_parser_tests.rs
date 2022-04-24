
use std::collections::HashMap;

use castle_error::CastleError;
use query_parser::{Field, FieldKind, parse_query};

#[test]
fn can_parse_empty_query() {
    let query = "";
    let expected = HashMap::new();
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual);
}

type Query = HashMap<Box<str>, Field>;

#[test]
fn can_parse_single_field() -> Result<(), CastleError> {
    let query = "first_name";

    let expected = [
        ("first_name".into(), Field {
            name: "first_name".into(),
            inputs: HashMap::new(),
            rename: None,
            kind: FieldKind::Field,
        }),
    ].into_iter().collect::<Query>();

    let actual = parse_query(query)?;
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn can_parse_two_fields_with_empty_args() -> Result<(), CastleError> {
    let query = "
    first_name()
    last_name
    email()
    ";

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
    ].into_iter().collect::<Query>();

    let actual = parse_query(query)?;
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn can_parse_object_projection() -> Result<(), CastleError> {
    let query = "me {
        first_name
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
            ].into_iter().collect::<Query>()),
        }),
    ].into_iter().collect::<Query>();
    let actual = parse_query(query)?;
    assert_eq!(expected, actual);
    return Ok(())
}

#[test]
fn can_parse_object_projection_with_two_fields() -> Result<(), CastleError> {
    let query = "me() {
        first_name,
        last_name
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
            ].into_iter().collect::<Query>()),
        }),
    ].into_iter().collect::<Query>();

    let actual = parse_query(query)?;
    assert_eq!(expected, actual);

    Ok(())
}

fn query_with_two_separators_fails() -> Result<(), CastleError> {
    let query = "me() {
        first_name,,
        last_name
    }";

    let actual = parse_query(query).expect_err("Expected error");

    Ok(())
}

fn query_with_trailing_comma_succeeds() -> Result<(), CastleError> {
    let query = "me() {
        first_name,
        last_name,
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
            ].into_iter().collect::<Query>()),
        }),
    ].into_iter().collect::<Query>();

    let actual = parse_query(query)?;
    assert_eq!(expected, actual);

    Ok(())
}

fn query_without_trailing_comma_succeeds() -> Result<(), CastleError> {
    let query = "me() {
        first_name
        last_name
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
            ].into_iter().collect::<Query>()),
        }),
    ].into_iter().collect::<Query>();

    let actual = parse_query(query)?;
    assert_eq!(expected, actual);

    Ok(())
}

fn query_with_missing_closing_bracket_fails() -> Result<(), CastleError> {
    let query = "me() {
        first_name
    ";

    let actual = parse_query(query).expect_err("Expected error");

    Ok(())
}

#[test]
fn can_parse_complex_object_projection_with_three_fields() {
    let query = "me() {
        first_name
        last_name
        role
    }";

        let fields = insert_each_field_into_fields(vec![
            ("first_name".into(), Want::new_single_field(HashMap::new())),
            ("last_name".into(), Want::new_single_field(HashMap::new())),
            ("role".into(), Want::new_single_field(HashMap::new())),
        ]);

        let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
            ("me".into(), Want::new_object_projection(fields, HashMap::new())),
        ]);

        let actual = parse_query(query).unwrap();
        assert_eq!(expected, actual.wants);
}

#[test]
fn can_parse_object_and_single_field() {
    let query = "
        me() {
            lol
        }
        lets_gooo()
        ";

        let fields = insert_each_field_into_fields(vec![
            ("lol".into(), Want::new_single_field(HashMap::new())),
        ]);

        let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
            ("me".into(), Want::new_object_projection(fields, HashMap::new())),
            ("lets_gooo".into(), Want::new_single_field(HashMap::new())),
        ]);

        let actual = parse_query(query).unwrap();
        assert_eq!(expected, actual.wants);
}

#[test]
fn can_parse_two_objects_and_two_fields() {
    let query = "
        me() {
            first_name
        }
        user() {
            username
            log_in_count
        }
        location
        device";

        let me_fields = insert_each_field_into_fields(vec![
            ("first_name".into(), Want::new_single_field(HashMap::new())),
        ]);

        let user_fields = insert_each_field_into_fields(vec![
            ("username".into(), Want::new_single_field(HashMap::new())),
            ("log_in_count".into(), Want::new_single_field(HashMap::new())),
        ]);

        let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
            ("me".into(), Want::new_object_projection(me_fields, HashMap::new())),
            ("user".into(), Want::new_object_projection(user_fields, HashMap::new())),
            ("location".into(), Want::new_single_field(HashMap::new())),
            ("device".into(), Want::new_single_field(HashMap::new())),
        ]);

        let actual = parse_query(query).unwrap();
        assert_eq!(expected, actual.wants);
}

#[test]
fn can_parse_object_projection_with_argument() {
    let query = "
    me() {
        first_name
        last_name
        email
        profile_picture(size: 48)
    }
    ";

    let mut profile_picture_argument = HashMap::new();
    profile_picture_argument.insert("size".into(), PrimitiveValue::UInt(48));

    let fields = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field(HashMap::new())),
        ("last_name".into(), Want::new_single_field(HashMap::new())),
        ("email".into(), Want::new_single_field(HashMap::new())),
        ("profile_picture".into(), Want::new_single_field(profile_picture_argument)),
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection(fields, HashMap::new())),
    ]);
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual.wants);
}

#[test]
fn can_parse_object_projection_with_multiple_arguments() {
    let query = "
    me(size: 3) {
        first_name
        last_name
        email
        profile_pic(r: 4, g: 60, b: 32, opacity: 0.5)
        heading(color: \"#FF0000\", arg: true)
    }";

    let mut profile_pic_argument = HashMap::new();
    profile_pic_argument.insert("r".into(), PrimitiveValue::UInt(4));
    profile_pic_argument.insert("g".into(), PrimitiveValue::UInt(60));
    profile_pic_argument.insert("b".into(), PrimitiveValue::UInt(32));
    profile_pic_argument.insert("opacity".into(), PrimitiveValue::Float(0.5));

    let mut heading_argument = HashMap::new();
    heading_argument.insert("color".into(), PrimitiveValue::String("#FF0000".into()));
    heading_argument.insert("arg".into(), PrimitiveValue::Boolean(true));

    let mut me_argument = HashMap::new();
    me_argument.insert("size".into(), PrimitiveValue::UInt(3));

    let fields = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field(HashMap::new())),
        ("last_name".into(), Want::new_single_field(HashMap::new())),
        ("email".into(), Want::new_single_field(HashMap::new())),
        ("profile_pic".into(), Want::new_single_field(profile_pic_argument)),
        ("heading".into(), Want::new_single_field(heading_argument))
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection(fields, me_argument)),
    ]);
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual.wants);
}

#[test]
fn can_parse_object_projection_with_inner_object() {
    let query = "
    me() {
        name() {
            first_name
            last_name
        }
        last_name
        email(size: 48)
    }";

    let inner_fields = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field(HashMap::new())),
        ("last_name".into(), Want::new_single_field(HashMap::new())),
    ]);

        let mut email_argument = HashMap::new();
        email_argument.insert("size".into(), PrimitiveValue::UInt(48));
        let fields = insert_each_field_into_fields(vec![
            ("name".into(), Want::new_object_projection(inner_fields, HashMap::new())),
            ("last_name".into(), Want::new_single_field(HashMap::new())),
            ("email".into(), Want::new_single_field(email_argument)),
        ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection(fields, HashMap::new())),
    ]);
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual.wants);
}
#[test]
fn can_parse_object_projection_with_nested_object() {
    let query = "
    me() {
        profile_pic() {
            url
            size() {
                width
                height
            }
        }
    }";

    let size_fields = insert_each_field_into_fields(vec![
        ("width".into(), Want::new_single_field(HashMap::new())),
        ("height".into(), Want::new_single_field(HashMap::new())),
    ]);

    let inner_fields = insert_each_field_into_fields(vec![
        ("url".into(), Want::new_single_field(HashMap::new())),
        ("size".into(), Want::new_object_projection(size_fields, HashMap::new())),
    ]);

    let fields = insert_each_field_into_fields(vec![
        ("profile_pic".into(), Want::new_object_projection(inner_fields, HashMap::new())),
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection(fields, HashMap::new())),
    ]);
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual.wants);
}

#[test]
fn can_parse_object_projection_with_match() {
    let query = "
        me() {
            first_name
            last_name
            email
            profile_picture(size: 48)
            icon() match {
                Icon::Svg => icon() {
                    url,
                    size
                },
                Icon::Emoji => emoji() {
                    emoji,
                    size
                }
            }
        }
    ";

    let mut profile_picture_argument = HashMap::new();
    profile_picture_argument.insert("size".into(), PrimitiveValue::UInt(48));

    let svg_fields = insert_each_field_into_fields(vec![
        ("url".into(), Want::new_single_field(HashMap::new())),
        ("size".into(), Want::new_single_field(HashMap::new())),
    ]);

    let svg_match_arm = MatchArm::new(
        EnumValue { identifier: "Icon::Svg".into(), enum_parent: "Icon".into(), variant: "Svg".into(), data_type: EnumDataType::EnumUnit },
    "icon".into(),
        Want::new_object_projection(svg_fields, HashMap::new()));

    let emoji_fields = insert_each_field_into_fields(vec![
        ("emoji".into(), Want::new_single_field(HashMap::new())),
        ("size".into(), Want::new_single_field(HashMap::new())),
    ]);

    let emoji_match_arms = MatchArm::new(
            EnumValue { identifier: "Icon::Emoji".into(), enum_parent: "Icon".into(), variant: "Emoji".into(), data_type: EnumDataType::EnumUnit },
            "emoji".into(),
            Want::ObjectProjection(emoji_fields, HashMap::new()));

    let match_statement = vec![
        svg_match_arm,
        emoji_match_arms,
    ];

    let fields = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field(HashMap::new())),
        ("last_name".into(), Want::new_single_field(HashMap::new())),
        ("email".into(), Want::new_single_field(HashMap::new())),
        ("profile_picture".into(), Want::new_single_field(profile_picture_argument)),
        ("icon".into(), Want::new_match(match_statement)),
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection(fields, HashMap::new())),
    ]);
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual.wants);
}

#[test]
fn can_parse_object_projection_with_match_inside_match() {
    let query = "
        me() {
            icon() match {
                Icon::Svg => icon() {
                    url(size: 48)
                    size() match {
                        Size::Rectangle => rectangle() {
                            width
                            height
                        },
                        Size::Square => square() {
                            side
                        }
                    }
                },
                Icon::Emoji => emoji() {
                    emoji
                    lol
                }
            }
        }
    ";

    let mut url_argument = HashMap::new();
    url_argument.insert("size".into(), PrimitiveValue::UInt(48));

    let rectangle_fields = insert_each_field_into_fields(vec![
        ("width".into(), Want::new_single_field(HashMap::new())),
        ("height".into(), Want::new_single_field(HashMap::new())),
    ]);

    let square_fields = insert_each_field_into_fields(vec![
        ("side".into(), Want::new_single_field(HashMap::new())),
    ]);

    let size_match = vec![
        MatchArm::new(
            EnumValue { identifier: "Size::Rectangle".into(), enum_parent: "Size".into(), variant: "Rectangle".into(), data_type: EnumDataType::EnumUnit },
            "rectangle".into(),
            Want::new_object_projection(rectangle_fields, HashMap::new())
        ),
        MatchArm::new(
            EnumValue { identifier: "Size::Square".into(), enum_parent: "Size".into(), variant: "Square".into(), data_type: EnumDataType::EnumUnit },
            "square".into(),
            Want::new_object_projection(square_fields, HashMap::new()),
        ),
    ];

    let svg_fields = insert_each_field_into_fields(vec![
        ("url".into(), Want::new_single_field(url_argument)),
        ("size".into(), Want::new_match(size_match)),
    ]);

    let emoji_fields = insert_each_field_into_fields(vec![
        ("emoji".into(), Want::new_single_field(HashMap::new())),
        ("lol".into(), Want::new_single_field(HashMap::new())),
    ]);

    let icon_match = vec![
        MatchArm::new(
            EnumValue { identifier: "Icon::Svg".into(), enum_parent: "Icon".into(), variant: "Svg".into(), data_type: EnumDataType::EnumUnit },
            "icon".into(),
            Want::new_object_projection(svg_fields, HashMap::new()),
        ),
        MatchArm::new(
            EnumValue { identifier: "Icon::Emoji".into(), enum_parent: "Icon".into(), variant: "Emoji".into(), data_type: EnumDataType::EnumUnit },
            "emoji".into(),
            Want::new_object_projection(emoji_fields, HashMap::new()),
        ),
    ];

    let fields = insert_each_field_into_fields(vec![
        ("icon".into(), Want::new_match(icon_match)),
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection(fields, HashMap::new())),
    ]);
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual.wants);
}

#[test]
fn trying_to_break_test_v1() {
    let query = "
    me() {
        (
    }
    ";

    let answer = parse_query(query).is_err();
    assert!(answer);
}

#[test]
fn trying_to_break_test_v2() {
    let query = "
    me() {
        ( {
            )
        }
    }
    ";

    let answer = parse_query(query).is_err();
    assert!(answer);
}

#[test]
fn trying_to_break_test_v3() {
    let query = "
    breaking_test() {
        ( {
            )gerg
        }
        ( {
            )gergerge
        }
    }
    ";

    let answer = parse_query(query).is_err();
    assert!(answer);
}

#[test]
fn should_parse_object_with_assumed_args() -> Result<(), CastleError> {
    let query = "
    me {
        first_name,
        date_of_birth {
            year
            month
            day
        }
    }";
    let query2 = "
    me() {
        first_name,
        date_of_birth() {
            year
            month
            day
        }
    }";
    let result1 = parse_query(query)?;
    let result2 = parse_query(query2)?;
    assert_eq!(result1, result2);
    Ok(())
}
#[test]
fn should_parse_object_with_no_fields_and_no_match() -> Result<(), CastleError> {
    let query = "
    me() {

    }";

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection(HashMap::new(), HashMap::new())),
    ]);
    let actual = parse_query(query)?;
    assert_eq!(expected, actual.wants);
    Ok(())
}