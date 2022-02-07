use std::hash::Hash;
use std::collections::HashMap;

use shared::CastleError;


use crate::ast::syntax_definitions::expressions::PrimitiveValue;
use crate::ast::syntax_definitions::argument::Argument;
use crate::ast::syntax_definitions::want::SingleField;
use crate::parser::query_parser::parse_query::parse_query;
use crate::ast::syntax_definitions::want::Want;
use crate::ast::syntax_definitions::want::ObjectProjection;
use crate::parser::query_parser::query_tests_utils::insert_each_field_into_fields;

#[cfg(test)]
#[test]
fn can_parse_empty_query() {
    let query = "";
    let expected = HashMap::new();
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_single_field() -> Result<(), CastleError> {
    let query = "first_name";

    let mut expected: HashMap<Box<str>, Want> = HashMap::new();
    expected.insert("first_name".into(), Want::new_single_field("first_name".into(), None));

    let actual = parse_query(query)?;
    assert_eq!(expected, actual);
    return Ok(())
}

#[test]
fn can_parse_two_fields() -> Result<(), CastleError> {
    let query = "first_name last_name";

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), None)),
        ("last_name".into(), Want::new_single_field("last_name".into(), None)),
    ]);

    let actual = parse_query(query)?;
    assert_eq!(expected, actual);
    return Ok(())
}

#[test]
fn can_parse_two_fields_different_lines() -> Result<(), CastleError> {
    let query = "
    first_name 
    last_name
    ";

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), None)),
        ("last_name".into(), Want::new_single_field("last_name".into(), None)),
    ]);

    let actual = parse_query(query)?;
    assert_eq!(expected, actual);
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
        ("first_name".into(), Want::new_single_field("first_name".into(), None)),
        ("last_name".into(), Want::new_single_field("last_name".into(), None)),
        ("time".into(), Want::new_single_field("time".into(), None)),
        ("email".into(), Want::new_single_field("email".into(), None)),
    ]);

    let actual = parse_query(query)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn can_parse_object_projection_with_single_field() -> Result<(), CastleError> {
    let query = "me {
        first_name
    }";
    
    let fields = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), None)),
    ]);

    let expected = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection("me".into(), Some(fields), None)),
    ]);
    
    let actual = parse_query(query)?;

    assert_eq!(expected, actual);
    return Ok(())
}

#[test]
fn can_parse_complex_object_projection_with_two_fields() {
    let query = "me {
        first_name,
        last_name
    }";

        let fields = insert_each_field_into_fields(vec![
            ("first_name".into(), Want::new_single_field("first_name".into(), None)),
            ("last_name".into(), Want::new_single_field("last_name".into(), None)),
        ]);

        let expected = insert_each_field_into_fields(vec![
            ("me".into(), Want::new_object_projection("me".into(), Some(fields), None)),
        ]);

        let actual = parse_query(query).unwrap();
        assert_eq!(expected, actual);
}

#[test]
fn can_parse_complex_object_projection_with_three_fields() {
    let query = "me {
        first_name
        last_name
        role
    }";
    
        let fields = insert_each_field_into_fields(vec![
            ("first_name".into(), Want::new_single_field("first_name".into(), None)),
            ("last_name".into(), Want::new_single_field("last_name".into(), None)),
            ("role".into(), Want::new_single_field("role".into(), None)),
        ]);
    
        let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
            ("me".into(), Want::new_object_projection("me".into(), Some(fields), None)),
        ]);

        let actual = parse_query(query).unwrap();
        assert_eq!(expected, actual);
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
            ("lol".into(), Want::new_single_field("lol".into(), None)),
        ]);

        let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
            ("me".into(), Want::new_object_projection("me".into(), Some(fields), None)),
            ("lets_gooo".into(), Want::new_single_field("lets_gooo".into(), None)),
        ]);

        let actual = parse_query(query).unwrap();
        assert_eq!(expected, actual);
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
            ("first_name".into(), Want::new_single_field("first_name".into(), None)),
        ]);

        let user_fields = insert_each_field_into_fields(vec![
            ("username".into(), Want::new_single_field("username".into(), None)),
            ("log_in_count".into(), Want::new_single_field("log_in_count".into(), None)),
        ]);
        
        let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
            ("me".into(), Want::new_object_projection("me".into(), Some(me_fields), None)),
            ("user".into(), Want::new_object_projection("user".into(), Some(user_fields), None)),
            ("location".into(), Want::new_single_field("location".into(), None)),
            ("device".into(), Want::new_single_field("device".into(), None)),
        ]);

        let actual = parse_query(query).unwrap();
        assert_eq!(expected, actual);
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
        ("first_name".into(), Want::new_single_field("first_name".into(), None)),
        ("last_name".into(), Want::new_single_field("last_name".into(), None)),
        ("email".into(), Want::new_single_field("email".into(), None)),
        ("profile_picture".into(), Want::new_single_field("profile_picture".into(), Some(vec![
            Argument::PrimitiveValue(PrimitiveValue::UInt(48))
        ]))),
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection("me".into(), Some(fields), None)),
    ]);
    let actual = parse_query(query).unwrap();
    println!("actual = {:#?}", actual);
    assert_eq!(expected, actual);
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
        ("first_name".into(), Want::new_single_field("first_name".into(), None)),
        ("last_name".into(), Want::new_single_field("last_name".into(), None)),
        ("email".into(), Want::new_single_field("email".into(), None)),
        ("profile_pic".into(), Want::new_single_field("profile_pic".into(), Some(vec![
            Argument::PrimitiveValue(PrimitiveValue::UInt(4)),
            Argument::PrimitiveValue(PrimitiveValue::UInt(60)),
            Argument::PrimitiveValue(PrimitiveValue::UInt(32)),
            Argument::PrimitiveValue(PrimitiveValue::Float(0.5)),
        ]))),
        ("heading".into(), Want::new_single_field("heading".into(), Some(vec![
            Argument::PrimitiveValue(PrimitiveValue::String("#FF0000".into())),
            Argument::PrimitiveValue(PrimitiveValue::Boolean(true)),
        ]))),
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection("me".into(), Some(fields), None)),
    ]);

    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual);
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
        ("first_name".into(), Want::new_single_field("first_name".into(), None)),
        ("last_name".into(), Want::new_single_field("last_name".into(), None)),
    ]);

        let fields = insert_each_field_into_fields(vec![
            ("name".into(), Want::new_object_projection("name".into(), Some(inner_fields), None)),
            ("last_name".into(), Want::new_single_field("last_name".into(), None)),
            ("email".into(), Want::new_single_field("email".into(), Some(vec![
                Argument::PrimitiveValue(PrimitiveValue::UInt(48)),
            ]))),
        ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection("me".into(), Some(fields), None)),
    ]);
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual);
}
#[test]
fn can_parse_object_projection_with_nested_object() {
    let query = "
    me {
        profile_pic: {
            url
            size: {
                width
                height
            }
        }
    }";

    let size_fields = insert_each_field_into_fields(vec![
        ("width".into(), Want::new_single_field("width".into(), None)),
        ("height".into(), Want::new_single_field("height".into(), None)),
    ]);

    let inner_fields = insert_each_field_into_fields(vec![
        ("url".into(), Want::new_single_field("url".into(), None)),
        ("size".into(), Want::new_object_projection("size".into(), Some(size_fields), None)),
    ]);

    let fields = insert_each_field_into_fields(vec![
        ("profile_pic".into(), Want::new_object_projection("profile_pic".into(), Some(inner_fields), None)),
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection("me".into(), Some(fields), None)),
    ]);
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual);
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
                SVGIcon: {
                    url
                    size
                }
                Emoji: {
                    emoji
                    size
                }
            }
        }
    ";
    let svg_fields = insert_each_field_into_fields(vec![
        ("url".into(), Want::new_single_field("url".into(), None)),
        ("size".into(), Want::new_single_field("size".into(), None)),
    ]);

    let emoji_fields = insert_each_field_into_fields(vec![
        ("emoji".into(), Want::new_single_field("emoji".into(), None)),
        ("size".into(), Want::new_single_field("size".into(), None)),
    ]);

    let match_fields = insert_each_field_into_fields(vec![
        ("SVGIcon".into(), Want::new_object_projection("SVGIcon".into(), Some(svg_fields), None)),
        ("Emoji".into(), Want::new_object_projection("Emoji".into(), Some(emoji_fields), None)),
    ]);

    let fields = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), None)),
        ("last_name".into(), Want::new_single_field("last_name".into(), None)),
        ("email".into(), Want::new_single_field("email".into(), None)),
        ("profile_picture".into(), Want::new_single_field("profile_picture".into(), Some(vec![
            Argument::PrimitiveValue(PrimitiveValue::UInt(48)),
        ]))),
        ("icon".into(), Want::new_object_projection("icon".into(), None, Some(match_fields))),
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection("me".into(), Some(fields), None)),
    ]);
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_object_projection_with_complex_match() {
    let query = "
        me {
            first_name
            last_name
            email
            profile_picture(48)
            icon: match {
                SVGIcon: {
                    url(48)
                    size: match {
                        rectangle: {
                            width
                            height
                        }
                        square: {
                            side
                        }
                    }
                }
                Emoji: {
                    emoji
                    size
                }
            }
        }
    ";
    let rectangle_fields = insert_each_field_into_fields(vec![
        ("width".into(), Want::new_single_field("width".into(), None)),
        ("height".into(), Want::new_single_field("height".into(), None)),
    ]);

    let square_fields = insert_each_field_into_fields(vec![
        ("side".into(), Want::new_single_field("side".into(), None)),
    ]);

    let size_match = insert_each_field_into_fields(vec![
        ("rectangle".into(), Want::new_object_projection("rectangle".into(), Some(rectangle_fields), None)),
        ("square".into(), Want::new_object_projection("square".into(), Some(square_fields), None)),
    ]);

    let svg_fields = insert_each_field_into_fields(vec![
        ("url".into(), Want::new_single_field("url".into(), Some(vec![
            Argument::PrimitiveValue(PrimitiveValue::UInt(48)),
        ]))),
        ("size".into(), Want::new_object_projection("size".into(), None, Some(size_match))),
    ]);

    let emoji_fields = insert_each_field_into_fields(vec![
        ("emoji".into(), Want::new_single_field("emoji".into(), None)),
        ("size".into(), Want::new_single_field("size".into(), None)),
    ]);

    let match_fields = insert_each_field_into_fields(vec![
        ("SVGIcon".into(), Want::new_object_projection("SVGIcon".into(), Some(svg_fields), None)),
        ("Emoji".into(), Want::new_object_projection("Emoji".into(), Some(emoji_fields), None)),
    ]);

    let fields = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), None)),
        ("last_name".into(), Want::new_single_field("last_name".into(), None)),
        ("email".into(), Want::new_single_field("email".into(), None)),
        ("profile_picture".into(), Want::new_single_field("profile_picture".into(), Some(vec![
            Argument::PrimitiveValue(PrimitiveValue::UInt(48)),
        ]))),
        ("icon".into(), Want::new_object_projection("icon".into(), None, Some(match_fields))),
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection("me".into(), Some(fields), None)),
    ]);
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual);
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
            SVGIcon: {     
                url  
                size    
            }
            Emoji: 
            
            
            
            
            
            
            
            {
                
                emoji

                size     
            }
        }
    }
";

    let svg_fields = insert_each_field_into_fields(vec![
        ("url".into(), Want::new_single_field("url".into(), None)),
        ("size".into(), Want::new_single_field("size".into(), None)),
    ]);

    let emoji_fields = insert_each_field_into_fields(vec![
        ("emoji".into(), Want::new_single_field("emoji".into(), None)),
        ("size".into(), Want::new_single_field("size".into(), None)),
    ]);

    let match_fields = insert_each_field_into_fields(vec![
        ("SVGIcon".into(), Want::new_object_projection("SVGIcon".into(), Some(svg_fields), None)),
        ("Emoji".into(), Want::new_object_projection("Emoji".into(), Some(emoji_fields), None)),
    ]);

    let fields = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), None)),
        ("last_name".into(), Want::new_single_field("last_name".into(), None)),
        ("email".into(), Want::new_single_field("email".into(), None)),
        ("profile_picture".into(), Want::new_single_field("profile_picture".into(), Some(vec![
            Argument::PrimitiveValue(PrimitiveValue::UInt(48)),
        ]))),
        ("icon".into(), Want::new_object_projection("icon".into(), None, Some(match_fields))),
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection("me".into(), Some(fields), None)),
    ]);
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn should_not_parse_object_with_no_fields_and_no_match() {
    let query = "
    me {

    }";

    let answer = parse_query(query).is_err();
    assert!(answer);
}