
use std::collections::HashMap;
use std::hash::Hash;

use parser_and_schema::ast::syntax_definitions::argument::{ArgumentOrTuple, IdentifierAndValueArgument};
use parser_and_schema::ast::syntax_definitions::enum_definition::{EnumValue, EnumDataType};
use parser_and_schema::ast::syntax_definitions::expressions::{PrimitiveValue, Expression};
use parser_and_schema::ast::syntax_definitions::match_statement::{MatchArm, MatchStatement};
use parser_and_schema::ast::syntax_definitions::want::{Want, FieldsType};
use parser_and_schema::parsers::query_parser::parse_query::parse_query;
use shared::CastleError;

pub fn insert_each_field_into_fields(vec_of_fields: Vec<(Box<str>, Want)>) -> HashMap<Box<str>, Want> {
    let mut fields = HashMap::new();
    for (field_name, field) in vec_of_fields {
        fields.insert(field_name.clone(), field);
    }
    return fields
}

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
    expected.insert("first_name".into(), Want::new_single_field("first_name".into(), HashMap::new()));

    let actual = parse_query(query)?;
assert_eq!(expected, actual.wants);
    return Ok(())
}

#[test]
fn can_parse_two_fields() -> Result<(), CastleError> {
    let query = "first_name() last_name()";

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), HashMap::new())),
        ("last_name".into(), Want::new_single_field("last_name".into(), HashMap::new())),
    ]);

    let actual = parse_query(query)?;
assert_eq!(expected, actual.wants);
    return Ok(())
}

#[test]
fn can_parse_two_fields_different_lines() -> Result<(), CastleError> {
    let query = "
    first_name()
    last_name()
    ";

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), HashMap::new())),
        ("last_name".into(), Want::new_single_field("last_name".into(), HashMap::new())),
    ]);

    let actual = parse_query(query)?;
    assert_eq!(expected, actual.wants);
    return Ok(())
}

#[test]
fn can_parse_four_fields_different_lines() -> Result<(), CastleError> {
    let query = "
    first_name()
    last_name()
    time()
    email()
    ";

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), HashMap::new())),
        ("last_name".into(), Want::new_single_field("last_name".into(), HashMap::new())),
        ("time".into(), Want::new_single_field("time".into(), HashMap::new())),
        ("email".into(), Want::new_single_field("email".into(), HashMap::new())),
    ]);

    let actual = parse_query(query)?;
    assert_eq!(expected, actual.wants);
    Ok(())
}

#[test]
fn can_parse_object_projection_with_single_field() -> Result<(), CastleError> {
    let query = "me() {
        first_name
    }";
    
    let fields = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), HashMap::new())),
    ]);

    let expected = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection("me".into(), FieldsType::Regular(fields), HashMap::new())),
    ]);
    
    let actual = parse_query(query)?;

    assert_eq!(expected, actual.wants);
    return Ok(())
}

#[test]
fn can_parse_complex_object_projection_with_two_fields() {
    let query = "me() {
        first_name,
        last_name
    }";

        let fields = insert_each_field_into_fields(vec![
            ("first_name".into(), Want::new_single_field("first_name".into(), HashMap::new())),
            ("last_name".into(), Want::new_single_field("last_name".into(), HashMap::new())),
        ]);

        let expected = insert_each_field_into_fields(vec![
            ("me".into(), Want::new_object_projection("me".into(), FieldsType::Regular(fields), HashMap::new())),
        ]);

        let actual = parse_query(query).unwrap();
        assert_eq!(expected, actual.wants);
}

#[test]
fn can_parse_complex_object_projection_with_three_fields() {
    let query = "me() {
        first_name
        last_name
        role
    }";
    
        let fields = insert_each_field_into_fields(vec![
            ("first_name".into(), Want::new_single_field("first_name".into(), HashMap::new())),
            ("last_name".into(), Want::new_single_field("last_name".into(), HashMap::new())),
            ("role".into(), Want::new_single_field("role".into(), HashMap::new())),
        ]);
    
        let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
            ("me".into(), Want::new_object_projection("me".into(), FieldsType::Regular(fields), HashMap::new())),
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
            ("lol".into(), Want::new_single_field("lol".into(), HashMap::new())),
        ]);

        let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
            ("me".into(), Want::new_object_projection("me".into(), FieldsType::Regular(fields), HashMap::new())),
            ("lets_gooo".into(), Want::new_single_field("lets_gooo".into(), HashMap::new())),
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
            ("first_name".into(), Want::new_single_field("first_name".into(), HashMap::new())),
        ]);

        let user_fields = insert_each_field_into_fields(vec![
            ("username".into(), Want::new_single_field("username".into(), HashMap::new())),
            ("log_in_count".into(), Want::new_single_field("log_in_count".into(), HashMap::new())),
        ]);
        
        let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
            ("me".into(), Want::new_object_projection("me".into(), FieldsType::Regular(me_fields), HashMap::new())),
            ("user".into(), Want::new_object_projection("user".into(), FieldsType::Regular(user_fields), HashMap::new())),
            ("location".into(), Want::new_single_field("location".into(), HashMap::new())),
            ("device".into(), Want::new_single_field("device".into(), HashMap::new())),
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
    profile_picture_argument.insert("size".into(), ("size".into(), PrimitiveValue::UInt(48)));

    let fields = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), HashMap::new())),
        ("last_name".into(), Want::new_single_field("last_name".into(), HashMap::new())),
        ("email".into(), Want::new_single_field("email".into(), HashMap::new())),
        ("profile_picture".into(), Want::new_single_field("profile_picture".into(), profile_picture_argument)),
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection("me".into(), FieldsType::Regular(fields), HashMap::new())),
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
    profile_pic_argument.insert("r".into(), ("r".into(), PrimitiveValue::UInt(4)));
    profile_pic_argument.insert("g".into(), ("g".into(), PrimitiveValue::UInt(60)));
    profile_pic_argument.insert("b".into(), ("b".into(), PrimitiveValue::UInt(32)));
    profile_pic_argument.insert("opacity".into(), ("opacity".into(), PrimitiveValue::Float(0.5)));

    let mut heading_argument = HashMap::new();
    heading_argument.insert("color".into(), ("color".into(), PrimitiveValue::String("#FF0000".into())));
    heading_argument.insert("arg".into(), ("arg".into(), PrimitiveValue::Boolean(true)));

    let mut me_argument: HashMap<Box<str>, IdentifierAndValueArgument> = HashMap::new();
    let me_arg_1 = ("size".into(), PrimitiveValue::UInt(3));
    me_argument.insert("size".into(), me_arg_1);

    let fields = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), HashMap::new())),
        ("last_name".into(), Want::new_single_field("last_name".into(), HashMap::new())),
        ("email".into(), Want::new_single_field("email".into(), HashMap::new())),
        ("profile_pic".into(), Want::new_single_field("profile_pic".into(), profile_pic_argument)),
        ("heading".into(), Want::new_single_field("heading".into(), heading_argument))
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection("me".into(), FieldsType::Regular(fields), me_argument)),
    ]);

    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual.wants);
}

#[test]
fn can_parse_object_projection_with_inner_object() {
    let query = "
    me() {
        name: {
            first_name
            last_name
        }
        last_name
        email(size: 48)
    }";

    let inner_fields = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), HashMap::new())),
        ("last_name".into(), Want::new_single_field("last_name".into(), HashMap::new())),
    ]);

        let mut email_argument = HashMap::new();
        email_argument.insert("size".into(), ("size".into(), PrimitiveValue::UInt(48)));
        let fields = insert_each_field_into_fields(vec![
            ("name".into(), Want::new_object_projection("name".into(), FieldsType::Regular(inner_fields), HashMap::new())),
            ("last_name".into(), Want::new_single_field("last_name".into(), HashMap::new())),
            ("email".into(), Want::new_single_field("email".into(), email_argument)),
        ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection("me".into(), FieldsType::Regular(fields), HashMap::new())),
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
        ("width".into(), Want::new_single_field("width".into(), HashMap::new())),
        ("height".into(), Want::new_single_field("height".into(), HashMap::new())),
    ]);

    let inner_fields = insert_each_field_into_fields(vec![
        ("url".into(), Want::new_single_field("url".into(), HashMap::new())),
        ("size".into(), Want::new_object_projection("size".into(), FieldsType::Regular(size_fields), HashMap::new())),
    ]);

    let fields = insert_each_field_into_fields(vec![
        ("profile_pic".into(), Want::new_object_projection("profile_pic".into(), FieldsType::Regular(inner_fields), HashMap::new())),
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection("me".into(), FieldsType::Regular(fields), HashMap::new())),
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
    
    let mut profile_picture_argument = HashMap::new();
    profile_picture_argument.insert("size".into(), ("size".into(), PrimitiveValue::UInt(48)));
    
    let svg_fields = insert_each_field_into_fields(vec![
        ("url".into(), Want::new_single_field("url".into(), HashMap::new())),
        ("size".into(), Want::new_single_field("size".into(), HashMap::new())),
    ]);

    let svg_match_arm = MatchArm::new(
        Expression::EnumValue( EnumValue { identifier: "Icon::Svg".into(), enum_parent: "Icon".into(), variant: "Svg".into(), data_type: EnumDataType::EnumUnit }),
        Want::new_object_projection("Icon::Svg".into(), FieldsType::Regular(svg_fields), HashMap::new()),
    );

    let emoji_fields = insert_each_field_into_fields(vec![
        ("emoji".into(), Want::new_single_field("emoji".into(), HashMap::new())),
        ("size".into(), Want::new_single_field("size".into(), HashMap::new())),
    ]);

    let emoji_match_arms = MatchArm::new(
            Expression::EnumValue( EnumValue { identifier: "Icon::Emoji".into(), enum_parent: "Icon".into(), variant: "Emoji".into(), data_type: EnumDataType::EnumUnit }),
            Want::new_object_projection("Icon::Emoji".into(), FieldsType::Regular(emoji_fields), HashMap::new()),
    );

    let match_statement = MatchStatement::new(vec![
        svg_match_arm,
        emoji_match_arms,
    ]);

    let fields = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), HashMap::new())),
        ("last_name".into(), Want::new_single_field("last_name".into(), HashMap::new())),
        ("email".into(), Want::new_single_field("email".into(), HashMap::new())),
        ("profile_picture".into(), Want::new_single_field("profile_picture".into(), profile_picture_argument)),
        ("icon".into(), Want::new_object_projection("icon".into(), FieldsType::Match(match_statement), HashMap::new())),
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection("me".into(), FieldsType::Regular(fields), HashMap::new())),
    ]);
    let actual = parse_query(query).unwrap();
    
    assert_eq!(expected, actual.wants);
}

#[test]
fn can_parse_object_projection_with_match_inside_match() {
    let query = "
        me() {
            icon: match {
                Icon::Svg => {
                    url(size: 48)
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

    let mut url_argument = HashMap::new();
    url_argument.insert("size".into(), ("size".into(), PrimitiveValue::UInt(48)));

    let rectangle_fields = insert_each_field_into_fields(vec![
        ("width".into(), Want::new_single_field("width".into(), HashMap::new())),
        ("height".into(), Want::new_single_field("height".into(), HashMap::new())),
    ]);

    let square_fields = insert_each_field_into_fields(vec![
        ("side".into(), Want::new_single_field("side".into(), HashMap::new())),
    ]);
    
    let size_match = MatchStatement::new(vec![
        MatchArm::new(
            Expression::EnumValue( EnumValue { identifier: "Size::Rectangle ".into(), enum_parent: "Size".into(), variant: "Rectangle".into(), data_type: EnumDataType::EnumUnit }),
            Want::new_object_projection("Size::Rectangle".into(), FieldsType::Regular(rectangle_fields), HashMap::new()),
        ),
        MatchArm::new(
            Expression::EnumValue( EnumValue { identifier: "Size::Square".into(), enum_parent: "Size".into(), variant: "Square".into(), data_type: EnumDataType::EnumUnit }),
            Want::new_object_projection("Size::Square".into(), FieldsType::Regular(square_fields), HashMap::new()),
        ),
    ]);

    let svg_fields = insert_each_field_into_fields(vec![
        ("url".into(), Want::new_single_field("url".into(), url_argument)),
        ("size".into(), Want::new_object_projection("size".into(), FieldsType::Match(size_match), HashMap::new())),
    ]);

    let emoji_fields = insert_each_field_into_fields(vec![
        ("emoji".into(), Want::new_single_field("emoji".into(), HashMap::new())),
        ("lol".into(), Want::new_single_field("lol".into(), HashMap::new())),
    ]);

    let icon_match = MatchStatement::new(vec![
        MatchArm::new(
            Expression::EnumValue( EnumValue { identifier: "Icon::Svg".into(), enum_parent: "Icon".into(), variant: "Svg".into(), data_type: EnumDataType::EnumUnit }),
            Want::new_object_projection("Icon::Svg".into(), FieldsType::Regular(svg_fields), HashMap::new()),
        ),
        MatchArm::new(
            Expression::EnumValue( EnumValue { identifier: "Icon::Emoji".into(), enum_parent: "Icon".into(), variant: "Emoji".into(), data_type: EnumDataType::EnumUnit }),
            Want::new_object_projection("Icon::Emoji".into(), FieldsType::Regular(emoji_fields), HashMap::new()),
        ),
    ]);

    let fields = insert_each_field_into_fields(vec![
        ("icon".into(), Want::new_object_projection("icon".into(), FieldsType::Match(icon_match), HashMap::new())),
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection("me".into(), FieldsType::Regular(fields), HashMap::new())),
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
fn trying_to_break_test_v4() {
    let query = "
    me() {
        first_name    
        last_name 

        email      
        profile_picture(
            size: 48
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

    let mut profile_picture_args = HashMap::new();
    profile_picture_args.insert("size".into(), ("size".into(), PrimitiveValue::UInt(48)));

    let svg_fields = insert_each_field_into_fields(vec![
        ("url".into(), Want::new_single_field("url".into(), HashMap::new())),
        ("size".into(), Want::new_single_field("size".into(), HashMap::new())),
    ]);

    let emoji_fields = insert_each_field_into_fields(vec![
        ("emoji".into(), Want::new_single_field("emoji".into(), HashMap::new())),
        ("size".into(), Want::new_single_field("size".into(), HashMap::new())),
    ]);

    let icon_match = MatchStatement::new(vec![
        MatchArm::new(
            Expression::EnumValue( EnumValue { identifier: "Icon::Svg".into(), enum_parent: "Icon".into(), variant: "Svg".into(), data_type: EnumDataType::EnumUnit }),
            Want::new_object_projection("Icon::Svg".into(), FieldsType::Regular(svg_fields), HashMap::new()),
        ),
        MatchArm::new(
            Expression::EnumValue( EnumValue { identifier: "Icon::Emoji".into(), enum_parent: "Icon".into(), variant: "Emoji".into(), data_type: EnumDataType::EnumUnit }),
            Want::new_object_projection("Icon::Emoji".into(), FieldsType::Regular(emoji_fields), HashMap::new()),
        ),
    ]);

    let fields = insert_each_field_into_fields(vec![
        ("first_name".into(), Want::new_single_field("first_name".into(), HashMap::new())),
        ("last_name".into(), Want::new_single_field("last_name".into(), HashMap::new())),
        ("email".into(), Want::new_single_field("email".into(), HashMap::new())),
        ("profile_picture".into(), Want::new_single_field("profile_picture".into(), profile_picture_args)),
        ("icon".into(), Want::new_object_projection("icon".into(), FieldsType::Match(icon_match), HashMap::new())),
    ]);

    let expected: HashMap<Box<str>, Want> = insert_each_field_into_fields(vec![
        ("me".into(), Want::new_object_projection(("me".into()), FieldsType::Regular(fields), HashMap::new())),
    ]);
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual.wants);
}

#[test]
fn should_not_parse_object_with_no_fields_and_no_match() {
    let query = "
    me() {

    }";

    let answer = parse_query(query).is_err();
    assert!(answer);
}