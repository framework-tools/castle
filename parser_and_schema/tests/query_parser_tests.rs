use std::hash::Hash;
use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::argument::Argument;
use parser_and_schema::ast::syntax_definitions::enum_definition::{EnumValue, EnumDataType};
use parser_and_schema::ast::syntax_definitions::expressions::{PrimitiveValue, Expression};
use parser_and_schema::ast::syntax_definitions::match_statement::{MatchArm, MatchStatement};
use parser_and_schema::ast::syntax_definitions::want::Want;
use parser_and_schema::parsers::query_parser::parse_query::parse_query;
use shared::CastleError;

pub fn insert_each_field_into_fields(vec_of_fields: Vec<(Box<str>, Want)>) -> HashMap<Box<str>, Want> {
    let mut fields = HashMap::new();
    for (field_name, field) in vec_of_fields {
        fields.insert(field_name.clone(), field);
    }
    return fields
}


#[cfg(test)]
#[test]
fn can_parse_empty_query() {
    use parser_and_schema::parsers::query_parser::parse_query::parse_query;

    let query = "";
    let expected = HashMap::new();
    let actual = parse_query(query).unwrap();
assert_eq!(expected, actual.wants);
}

#[test]
fn can_parse_single_field() -> Result<(), CastleError> {
    let query = "first_name";

    let mut expected: HashMap<Box<str>, Want> = HashMap::new();
    expected.insert("first_name".into(), Want::new_single_field("first_name".into(), None, None));

    let actual = parse_query(query)?;
assert_eq!(expected, actual.wants);
    return Ok(())
}

#[test]
fn can_parse_two_fields() -> Result<(), CastleError> {
    let query = "first_name last_name";

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), None, None)),
        ("last_name".into(), Want::new_single_field("last_name".into(), None, None)),
    ]);

    let actual = parse_query(query)?;
assert_eq!(expected, actual.wants);
    return Ok(())
}

#[test]
fn can_parse_two_fields_different_lines() -> Result<(), CastleError> {
    let query = "
    first_name 
    last_name
    ";

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), None, None)),
        ("last_name".into(), Want::new_single_field("last_name".into(), None, None)),
    ]);

    let actual = parse_query(query)?;
    assert_eq!(expected, actual.wants);
    return Ok(())
}

#[test]
fn can_parse_four_fields_different_lines() -> Result<(), CastleError> {
    let query = "
    first_name 
    last_name
    time
    email
    ";

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), None, None)),
        ("last_name".into(), Want::new_single_field("last_name".into(), None, None)),
        ("time".into(), Want::new_single_field("time".into(), None, None)),
        ("email".into(), Want::new_single_field("email".into(), None, None)),
    ]);

    let actual = parse_query(query)?;
    assert_eq!(expected, actual.wants);
    Ok(())
}

#[test]
fn can_parse_object_projection_with_single_field() -> Result<(), CastleError> {
    let query = "me {
        first_name
    }";
    
    let fields = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), None, None)),
    ]);

    let expected = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection(Some("me".into()), Some(fields), None)),
    ]);
    
    let actual = parse_query(query)?;

    assert_eq!(expected, actual.wants);
    return Ok(())
}

#[test]
fn can_parse_complex_object_projection_with_two_fields() {
    let query = "me {
        first_name,
        last_name
    }";

        let fields = insert_each_field_into_fields(vec![
            ("first_name".into(), Want::new_single_field("first_name".into(), None, None)),
            ("last_name".into(), Want::new_single_field("last_name".into(), None, None)),
        ]);

        let expected = insert_each_field_into_fields(vec![
            ("me".into(), Want::new_object_projection(Some("me".into()), Some(fields), None)),
        ]);

        let actual = parse_query(query).unwrap();
        assert_eq!(expected, actual.wants);
}

#[test]
fn can_parse_complex_object_projection_with_three_fields() {
    let query = "me {
        first_name
        last_name
        role
    }";
    
        let fields = insert_each_field_into_fields(vec![
            ("first_name".into(), Want::new_single_field("first_name".into(), None, None)),
            ("last_name".into(), Want::new_single_field("last_name".into(), None, None)),
            ("role".into(), Want::new_single_field("role".into(), None, None)),
        ]);
    
        let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
            ("me".into(), Want::new_object_projection(Some("me".into()), Some(fields), None)),
        ]);

        let actual = parse_query(query).unwrap();
        assert_eq!(expected, actual.wants);
}

#[test]
fn can_parse_object_and_single_field() {
    let query = "
        me {
            lol
        }
        lets_gooo
        ";
    
        let fields = insert_each_field_into_fields(vec![
            ("lol".into(), Want::new_single_field("lol".into(), None, None)),
        ]);

        let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
            ("me".into(), Want::new_object_projection(Some("me".into()), Some(fields), None)),
            ("lets_gooo".into(), Want::new_single_field("lets_gooo".into(), None, None)),
        ]);

        let actual = parse_query(query).unwrap();
        assert_eq!(expected, actual.wants);
}

#[test]
fn can_parse_two_objects_and_two_fields() {
    let query = "
        me {
            first_name
        }
        user {
            username
            log_in_count
        }
        location
        device";
    
        let me_fields = insert_each_field_into_fields(vec![
            ("first_name".into(), Want::new_single_field("first_name".into(), None, None)),
        ]);

        let user_fields = insert_each_field_into_fields(vec![
            ("username".into(), Want::new_single_field("username".into(), None, None)),
            ("log_in_count".into(), Want::new_single_field("log_in_count".into(), None, None)),
        ]);
        
        let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
            ("me".into(), Want::new_object_projection(Some("me".into()), Some(me_fields), None)),
            ("user".into(), Want::new_object_projection(Some("user".into()), Some(user_fields), None)),
            ("location".into(), Want::new_single_field("location".into(), None, None)),
            ("device".into(), Want::new_single_field("device".into(), None, None)),
        ]);

        let actual = parse_query(query).unwrap();
        assert_eq!(expected, actual.wants);
}

#[test]
fn can_parse_object_projection_with_argument() {
    let query = "
    me {
        first_name
        last_name
        email
        profile_picture(48)
    }
    ";

    let fields = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), None, None)),
        ("last_name".into(), Want::new_single_field("last_name".into(), None, None)),
        ("email".into(), Want::new_single_field("email".into(), None, None)),
        ("profile_picture".into(), Want::new_single_field("profile_picture".into(), Some(vec![
            Argument::PrimitiveValue(PrimitiveValue::UInt(48))
        ]), None
    )),
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection(Some("me".into()), Some(fields), None)),
    ]);
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual.wants);
}

#[test]
fn can_parse_object_projection_with_multiple_arguments() {
    let query = "
    me {
        first_name
        last_name
        email
        profile_pic(4, 60, 32, 0.5)
        heading(\"#FF0000\", true)
    }";

    let fields = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), None, None)),
        ("last_name".into(), Want::new_single_field("last_name".into(), None, None)),
        ("email".into(), Want::new_single_field("email".into(), None, None)),
        ("profile_pic".into(), Want::new_single_field("profile_pic".into(), Some(vec![
            Argument::PrimitiveValue(PrimitiveValue::UInt(4)),
            Argument::PrimitiveValue(PrimitiveValue::UInt(60)),
            Argument::PrimitiveValue(PrimitiveValue::UInt(32)),
            Argument::PrimitiveValue(PrimitiveValue::Float(0.5)),
        ]), None)),
        ("heading".into(), Want::new_single_field("heading".into(), Some(vec![
            Argument::PrimitiveValue(PrimitiveValue::String("#FF0000".into())),
            Argument::PrimitiveValue(PrimitiveValue::Boolean(true)),
        ]), None)),
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection(Some("me".into()), Some(fields), None)),
    ]);

    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual.wants);
}

#[test]
fn can_parse_object_projection_with_inner_object() {
    let query = "
    me {
        name: {
            first_name
            last_name
        }
        last_name
        email(48)
    }";

    let inner_fields = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), None, None)),
        ("last_name".into(), Want::new_single_field("last_name".into(), None, None)),
    ]);

        let fields = insert_each_field_into_fields(vec![
            ("name".into(), Want::new_object_projection(Some("name".into()), Some(inner_fields), None)),
            ("last_name".into(), Want::new_single_field("last_name".into(), None, None)),
            ("email".into(), Want::new_single_field("email".into(), Some(vec![
                Argument::PrimitiveValue(PrimitiveValue::UInt(48)),
            ]), None)),
        ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection(Some("me".into()), Some(fields), None)),
    ]);
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual.wants);
}
#[test]
fn can_parse_object_projection_with_nested_object() {
    let query = "
    me() {
        profile_pic: {
            url
            size: {
                width
                height
            }
        }
    }";

    let size_fields = insert_each_field_into_fields(vec![
        ("width".into(), Want::new_single_field("width".into(), None, None)),
        ("height".into(), Want::new_single_field("height".into(), None, None)),
    ]);

    let inner_fields = insert_each_field_into_fields(vec![
        ("url".into(), Want::new_single_field("url".into(), None, None)),
        ("size".into(), Want::new_object_projection(Some("size".into()), Some(size_fields), None)),
    ]);

    let fields = insert_each_field_into_fields(vec![
        ("profile_pic".into(), Want::new_object_projection(Some("profile_pic".into()), Some(inner_fields), None)),
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection(Some("me".into()), Some(fields), None)),
    ]);
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual.wants);
}

#[test]
fn can_parse_object_projection_with_match() {
    let query = "
        me {
            first_name
            last_name
            email
            profile_picture(48)
            icon: match {
                Icon::Svg => {
                    url
                    size
                },
                Icon::Emoji => {
                    emoji
                    size
                }
            }
        }
    ";

    let svg_fields = insert_each_field_into_fields(vec![
        ("url".into(), Want::new_single_field("url".into(), None, None)),
        ("size".into(), Want::new_single_field("size".into(), None, None)),
    ]);

    let svg_match_arm = MatchArm::new(
        Expression::EnumValue( EnumValue { identifier: "Icon::Svg".into(), enum_parent: "Icon".into(), variant: "Svg".into(), data_type: EnumDataType::EnumUnit }),
        Want::new_object_projection(Some("Icon::Svg".into()), Some(svg_fields), None),
    );

    let emoji_fields = insert_each_field_into_fields(vec![
        ("emoji".into(), Want::new_single_field("emoji".into(), None, None)),
        ("size".into(), Want::new_single_field("size".into(), None, None)),
    ]);

    let emoji_match_arms = MatchArm::new(
            Expression::EnumValue( EnumValue { identifier: "Icon::Emoji".into(), enum_parent: "Icon".into(), variant: "Emoji".into(), data_type: EnumDataType::EnumUnit }),
            Want::new_object_projection(Some("Icon::Emoji".into()), Some(emoji_fields), None),
    );

    let match_statement = MatchStatement::new(vec![
        svg_match_arm,
        emoji_match_arms,
    ]);

    let fields = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), None, None)),
        ("last_name".into(), Want::new_single_field("last_name".into(), None, None)),
        ("email".into(), Want::new_single_field("email".into(), None, None)),
        ("profile_picture".into(), Want::new_single_field("profile_picture".into(), Some(vec![
            Argument::PrimitiveValue(PrimitiveValue::UInt(48)),
        ]), None)),
        ("icon".into(), Want::new_object_projection(Some("icon".into()), None, Some(match_statement))),
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection(Some("me".into()), Some(fields), None)),
    ]);
    let actual = parse_query(query).unwrap();
    
    assert_eq!(expected, actual.wants);
}

#[test]
fn can_parse_object_projection_with_match_inside_match() {
    let query = "
        me {
            icon: match {
                Icon::Svg => {
                    url(48)
                    size: match {
                        Size::Rectangle => {
                            width
                            height
                        }
                        Size::Square => {
                            side
                        }
                    }
                },
                Icon::Emoji => {
                    emoji
                    lol
                }
            }
        }
    ";

    let rectangle_fields = insert_each_field_into_fields(vec![
        ("width".into(), Want::new_single_field("width".into(), None, None)),
        ("height".into(), Want::new_single_field("height".into(), None, None)),
    ]);

    let square_fields = insert_each_field_into_fields(vec![
        ("side".into(), Want::new_single_field("side".into(), None, None)),
    ]);
    
    let size_match = MatchStatement::new(vec![
        MatchArm::new(
            Expression::EnumValue( EnumValue { identifier: "Size::Rectangle ".into(), enum_parent: "Size".into(), variant: "Rectangle".into(), data_type: EnumDataType::EnumUnit }),
            Want::new_object_projection(Some("Size::Rectangle".into()), Some(rectangle_fields), None),
        ),
        MatchArm::new(
            Expression::EnumValue( EnumValue { identifier: "Size::Square".into(), enum_parent: "Size".into(), variant: "Square".into(), data_type: EnumDataType::EnumUnit }),
            Want::new_object_projection(Some("Size::Square".into()), Some(square_fields), None),
        ),
    ]);

    let svg_fields = insert_each_field_into_fields(vec![
        ("url".into(), Want::new_single_field("url".into(), Some(vec![
            Argument::PrimitiveValue(PrimitiveValue::UInt(48)),
        ]), None)),
        ("size".into(), Want::new_object_projection(Some("size".into()), None, Some(size_match))),
    ]);

    let emoji_fields = insert_each_field_into_fields(vec![
        ("emoji".into(), Want::new_single_field("emoji".into(), None, None)),
        ("lol".into(), Want::new_single_field("lol".into(), None, None)),
    ]);

    let icon_match = MatchStatement::new(vec![
        MatchArm::new(
            Expression::EnumValue( EnumValue { identifier: "Icon::Svg".into(), enum_parent: "Icon".into(), variant: "Svg".into(), data_type: EnumDataType::EnumUnit }),
            Want::new_object_projection(Some("Icon::Svg".into()), Some(svg_fields), None),
        ),
        MatchArm::new(
            Expression::EnumValue( EnumValue { identifier: "Icon::Emoji".into(), enum_parent: "Icon".into(), variant: "Emoji".into(), data_type: EnumDataType::EnumUnit }),
            Want::new_object_projection(Some("Icon::Emoji".into()), Some(emoji_fields), None),
        ),
    ]);

    let fields = insert_each_field_into_fields(vec![
        ("icon".into(), Want::new_object_projection(Some("icon".into()), None, Some(icon_match))),
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection(Some("me".into()), Some(fields), None)),
    ]);
    let actual = parse_query(query).unwrap();
    println!("actual {:#?}", actual);
    assert_eq!(expected, actual.wants);
}

#[test]
fn trying_to_break_test_v1() {
    let query = "
    me {
        (
    }
    ";
    
    let answer = parse_query(query).is_err();
    assert!(answer);
}

#[test]
fn trying_to_break_test_v2() {
    let query = "
    me {
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
    breaking_test {
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
fn trying_to_break_test_v4() {
    let query = "
    me {
        first_name    
        last_name 

        email      
        profile_picture(
            48
        )
        icon: match {
            Icon::Svg => {     
                url  
                size    
            },
            Icon::Emoji =>
            
            
            
            
            
            
            
            {
                
                emoji

                size     
            }
        }
    }
";

    let svg_fields = insert_each_field_into_fields(vec![
        ("url".into(), Want::new_single_field("url".into(), None, None)),
        ("size".into(), Want::new_single_field("size".into(), None, None)),
    ]);

    let emoji_fields = insert_each_field_into_fields(vec![
        ("emoji".into(), Want::new_single_field("emoji".into(), None, None)),
        ("size".into(), Want::new_single_field("size".into(), None, None)),
    ]);

    let icon_match = MatchStatement::new(vec![
        MatchArm::new(
            Expression::EnumValue( EnumValue { identifier: "Icon::Svg".into(), enum_parent: "Icon".into(), variant: "Svg".into(), data_type: EnumDataType::EnumUnit }),
            Want::new_object_projection(Some("Icon::Svg".into()), Some(svg_fields), None),
        ),
        MatchArm::new(
            Expression::EnumValue( EnumValue { identifier: "Icon::Emoji".into(), enum_parent: "Icon".into(), variant: "Emoji".into(), data_type: EnumDataType::EnumUnit }),
            Want::new_object_projection(Some("Icon::Emoji".into()), Some(emoji_fields), None),
        ),
    ]);

    let fields = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), None, None)),
        ("last_name".into(), Want::new_single_field("last_name".into(), None, None)),
        ("email".into(), Want::new_single_field("email".into(), None, None)),
        ("profile_picture".into(), Want::new_single_field("profile_picture".into(), Some(vec![
            Argument::PrimitiveValue(PrimitiveValue::UInt(48)),
        ]), None)),
        ("icon".into(), Want::new_object_projection(Some("icon".into()), None, Some(icon_match))),
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection(Some("me".into()), Some(fields), None)),
    ]);
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual.wants);
}

#[test]
fn should_not_parse_object_with_no_fields_and_no_match() {
    let query = "
    me {

    }";

    let answer = parse_query(query).is_err();
    assert!(answer);
}