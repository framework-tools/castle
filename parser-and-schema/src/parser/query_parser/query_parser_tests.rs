use std::hash::Hash;
use std::collections::HashMap;

use shared::CastleError;


use crate::ast::syntax_definitions::expressions::PrimitiveValue;
use crate::ast::syntax_definitions::want::SingleField;
use crate::parser::query_parser::parse_query::parse_query;
use crate::ast::syntax_definitions::want::Want;
use crate::ast::syntax_definitions::want::ObjectProjection;

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

    let want1 = Want::new_single_field("first_name".into(), None);
    let want2 = Want::new_single_field("last_name".into(), None);

    let mut expected: HashMap<Box<str>, Want> = HashMap::new();
    expected.insert("first_name".into(), want1.clone());
    expected.insert("last_name".into(),want2.clone());

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

    let want1 = Want::new_single_field("first_name".into(), None);
    let want2 = Want::new_single_field("last_name".into(), None);

    let mut expected: HashMap<Box<str>, Want> = HashMap::new();
    expected.insert("first_name".into(), want1.clone());
    expected.insert("last_name".into(), want2.clone());

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

    let want1 = Want::new_single_field("first_name".into(), None);
    let want2 = Want::new_single_field("last_name".into(), None);
    let want3 = Want::new_single_field("time".into(), None);
    let want4 = Want::new_single_field("email".into(), None);

    let mut expected: HashMap<Box<str>, Want> = HashMap::new();
    expected.insert("first_name".into(), want1);
    expected.insert("last_name".into(), want2);
    expected.insert("time".into(),want3);
    expected.insert("email".into(),want4);

    let actual = parse_query(query)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn can_parse_object_projection_with_single_field() -> Result<(), CastleError> {
    let query = "me {
        first_name
    }";
    
    let mut fields = HashMap::new();
    fields.insert("first_name".into(), Want::new_single_field("first_name".into(), None).into());
    let mut expected: HashMap<Box<str>, Want> = HashMap::new();
    let want = Want::Projection(ObjectProjection {
        identifier: "me".into(),
        fields: Some(fields),
        match_statements: None
    });
    expected.insert("me".into(), want.clone());
    
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
    
        let mut fields = HashMap::new();
        fields.insert("first_name".into(), Want::new_single_field("first_name".into(), None).into());
        fields.insert("last_name".into(), Want::new_single_field("last_name".into(), None).into());
    
        let mut expected: HashMap<Box<str>, Want> = HashMap::new();

        expected.insert("me".into(), Want::Projection(ObjectProjection { // unsure here
            identifier: "me".into(),
            fields: Some(fields),
            match_statements: None
        }));

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
    
        let mut fields = HashMap::new();
        fields.insert("first_name".into(), Want::new_single_field("first_name".into(), None).into());
        fields.insert("last_name".into(), Want::new_single_field("last_name".into(), None).into());
        fields.insert("role".into(), Want::new_single_field("role".into(), None).into());
    
        let mut expected: HashMap<Box<str>, Want> = HashMap::new();
        expected.insert("me".into(),Want::Projection(ObjectProjection {
            identifier: "me".into(),
            fields: Some(fields),
            match_statements: None
        }));

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
    
        let mut fields = HashMap::new();
        fields.insert("lol".into(), Want::new_single_field("lol".into(), None).into());

        let mut expected: HashMap<Box<str>, Want> = HashMap::new();
        expected.insert("me".into(), Want::Projection(ObjectProjection {
            identifier:"me".into(),
            fields: Some(fields),
            match_statements: None
        }));
        expected.insert("lets_gooo".into(), Want::new_single_field("lets_gooo".into(), None));

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
    
        let mut fields = HashMap::new();
        fields.insert("first_name".into(), Want::new_single_field("first_name".into(), None).into());

        let mut expected: HashMap<Box<str>, Want> = HashMap::new();
        expected.insert("me".into(),Want::Projection(ObjectProjection {
            identifier: "me".into(),
            fields: Some(fields),
            match_statements: None
        }));

        let mut fields = HashMap::new();
        fields.insert("username".into(), Want::new_single_field("username".into(), None).into());
        fields.insert("log_in_count".into(), Want::new_single_field("log_in_count".into(), None).into());
        expected.insert("user".into(), Want::Projection(ObjectProjection {
            identifier: "user".into(),
            fields: Some(fields),
            match_statements: None
        }));
        expected.insert("location".into(), Want::new_single_field("location".into(), None).into());
        expected.insert("device".into(), Want::new_single_field("device".into(), None).into());

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

    let mut fields = HashMap::new();
    fields.insert("first_name".into(), Want::new_single_field("first_name".into(), None).into());
    fields.insert("last_name".into(), Want::new_single_field("last_name".into(), None).into());
    fields.insert("email".into(), Want::new_single_field("email".into(), None).into());
    fields.insert("profile_picture".into(), Want::new_single_field("profile_picture".into(), Some(vec![PrimitiveValue::UInt(48)])).into());

    let mut expected: HashMap<Box<str>, Want> = HashMap::new();
    expected.insert("me".into(),Want::Projection(ObjectProjection {
        identifier: "me".into(),
        fields: Some(fields),
        match_statements: None
    }));
    let actual = parse_query(query).unwrap();
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

    let mut fields = HashMap::new();
    fields.insert("first_name".into(), Want::new_single_field("first_name".into(), None).into());
    fields.insert("last_name".into(), Want::new_single_field("last_name".into(), None).into());
    fields.insert("email".into(), Want::new_single_field("email".into(), None).into());
    fields.insert("profile_pic".into(), Want::new_single_field("profile_pic".into(), Some(vec![
        PrimitiveValue::UInt(4),
        PrimitiveValue::UInt(60),
        PrimitiveValue::UInt(32),
        PrimitiveValue::Float(0.5)
    ])).into());
    fields.insert("heading".into(), Want::new_single_field("heading".into(), Some(vec![
        PrimitiveValue::String("\"#FF0000\"".into()),
        PrimitiveValue::Boolean(true)
    ])).into());

    let mut expected: HashMap<Box<str>, Want> = HashMap::new();
    expected.insert("me".into(), Want::Projection(ObjectProjection {
        identifier:"me".into(),
        fields: Some(fields),
        match_statements: None
    }));
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

    let mut fields = HashMap::new();
    let mut inner_fields = HashMap::new();
    inner_fields.insert("first_name".into(), Want::new_single_field("first_name".into(), None).into());
    inner_fields.insert("last_name".into(), Want::new_single_field("last_name".into(), None).into());

    fields.insert("name".into(), Want::new_projection("name".into(), Some(inner_fields), None).into());
    fields.insert("last_name".into(), Want::new_single_field("last_name".into(), None).into());
    fields.insert("email".into(), Want::new_single_field("email".into(), Some(vec![PrimitiveValue::UInt(48)])).into());

    let mut expected: HashMap<Box<str>, Want> = HashMap::new();
    expected.insert("me".into(),Want::Projection(ObjectProjection {
        identifier: "me".into(),
        fields: Some(fields),
        match_statements: None
    }));
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

    let mut size_fields = HashMap::new();
    size_fields.insert("width".into(),Want::new_single_field("width".into(), None).into());
    size_fields.insert("height".into(),Want::new_single_field("height".into(), None).into());

    let mut fields = HashMap::new();
    let mut inner_fields = HashMap::new();
    inner_fields.insert("url".into(), Want::new_single_field("url".into(), None).into());
    inner_fields.insert("size".into(), Want::new_projection("size".into(), Some(size_fields), None).into());

    fields.insert("profile_pic".into(), Want::new_projection("profile_pic".into(), Some(inner_fields), None).into());

    let mut expected: HashMap<Box<str>, Want> = HashMap::new();
    expected.insert("me".into(),Want::Projection(ObjectProjection {
        identifier: "me".into(),
        fields: Some(fields),
        match_statements: None
    }));
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
    let mut svg_fields = HashMap::new();
    svg_fields.insert("url".into(), Want::new_single_field("url".into(), None).into());
    svg_fields.insert("size".into(), Want::new_single_field("size".into(), None).into());

    let mut emoji_fields = HashMap::new();
    emoji_fields.insert("emoji".into(), Want::new_single_field("emoji".into(), None).into());
    emoji_fields.insert("size".into(), Want::new_single_field("size".into(), None).into());

    let mut match_fields = HashMap::new();
    match_fields.insert("SVGIcon".into(), Want::new_projection("SVGIcon".into(), Some(svg_fields), None).into());
    match_fields.insert("Emoji".into(), Want::new_projection("Emoji".into(), Some(emoji_fields), None).into());

    let mut fields = HashMap::new();
    fields.insert("first_name".into(), Want::new_single_field("first_name".into(), None).into());
    fields.insert("last_name".into(), Want::new_single_field("last_name".into(), None).into());
    fields.insert("email".into(), Want::new_single_field("email".into(), None).into());
    fields.insert("profile_picture".into(), Want::new_single_field("profile_picture".into(), Some(vec![PrimitiveValue::UInt(48)])).into());
    fields.insert("icon".into(), Want::new_projection("icon".into(), None, Some(match_fields)).into());

    let mut expected: HashMap<Box<str>, Want> = HashMap::new();
    expected.insert("me".into(),Want::Projection(ObjectProjection {
        identifier: "me".into(),
        fields: Some(fields),
        match_statements: None
    }));
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
    let mut rectangle_fields = HashMap::new();
    rectangle_fields.insert("width".into(), Want::new_single_field("width".into(), None).into());
    rectangle_fields.insert("height".into(), Want::new_single_field("height".into(), None).into());

    let mut square_fields = HashMap::new();
    square_fields.insert("side".into(), Want::new_single_field("side".into(), None).into());

    let mut size_match = HashMap::new();
    size_match.insert("rectangle".into(), Want::new_projection("rectangle".into(), Some(rectangle_fields), None).into());
    size_match.insert("square".into(), Want::new_projection("square".into(), Some(square_fields), None).into());

    let mut svg_fields = HashMap::new();
    svg_fields.insert("url".into(), Want::new_single_field("url".into(), Some(vec![PrimitiveValue::UInt(48)])).into());
    svg_fields.insert("size".into(), Want::new_projection("size".into(), None, Some(size_match)).into());

    let mut emoji_fields = HashMap::new();
    emoji_fields.insert("emoji".into(), Want::new_single_field("emoji".into(), None).into());
    emoji_fields.insert("size".into(), Want::new_single_field("size".into(), None).into());

    let mut match_fields = HashMap::new();
    match_fields.insert("SVGIcon".into(), Want::new_projection("SVGIcon".into(), Some(svg_fields), None).into());
    match_fields.insert("Emoji".into(), Want::new_projection("Emoji".into(), Some(emoji_fields), None).into());

    let mut fields = HashMap::new();
    fields.insert("first_name".into(), Want::new_single_field("first_name".into(), None).into());
    fields.insert("last_name".into(), Want::new_single_field("last_name".into(), None).into());
    fields.insert("email".into(), Want::new_single_field("email".into(), None).into());
    fields.insert("profile_picture".into(), Want::new_single_field("profile_picture".into(), Some(vec![PrimitiveValue::UInt(48)])).into());
    fields.insert("icon".into(), Want::new_projection("icon".into(), None, Some(match_fields)).into());

    let mut expected: HashMap<Box<str>, Want> = HashMap::new();
    expected.insert("me".into(),Want::Projection(ObjectProjection {
        identifier: "me".into(),
        fields: Some(fields),
        match_statements: None
    }));
    let actual = parse_query(query).unwrap();
    println!("actual: {:#?}", actual);
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

    let mut svg_fields = HashMap::new();
    svg_fields.insert("url".into(), Want::new_single_field("url".into(), None));
    svg_fields.insert("size".into(), Want::new_single_field("size".into(), None));

    let mut emoji_fields = HashMap::new();
    emoji_fields.insert("emoji".into(), Want::new_single_field("emoji".into(), None));
    emoji_fields.insert("size".into(), Want::new_single_field("size".into(), None));

    let mut match_fields = HashMap::new();
    match_fields.insert("SVGIcon".into(), Want::new_projection("SVGIcon".into(), Some(svg_fields), None));
    match_fields.insert("Emoji".into(), Want::new_projection("Emoji".into(), Some(emoji_fields), None));

    let mut fields = HashMap::new();
    fields.insert("first_name".into(), Want::new_single_field("first_name".into(), None));
    fields.insert("last_name".into(), Want::new_single_field("last_name".into(), None));
    fields.insert("email".into(), Want::new_single_field("email".into(), None));
    fields.insert("profile_picture".into(), Want::new_single_field("profile_picture".into(), Some(vec![PrimitiveValue::UInt(48)])).into());
    fields.insert("icon".into(), Want::new_projection("icon".into(), None, Some(match_fields)).into());

    let mut expected: HashMap<Box<str>, Want> = HashMap::new();
    expected.insert("me".into(),Want::Projection(ObjectProjection {
        identifier: "me".into(),
        fields: Some(fields),
        match_statements: None
    }));
    let actual = parse_query(query).unwrap();
    println!("actual: {:#?}", actual);
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