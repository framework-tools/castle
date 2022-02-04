use std::hash::Hash;
use std::collections::HashMap;

use shared::CastleError;

use crate::ast::syntax_definitions::expressions::F64;
use crate::ast::syntax_definitions::expressions::PrimitiveValue;
use crate::ast::syntax_definitions::want::SingleField;
use crate::parser::parse_query::parse_query; 
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

    let mut expected: HashMap<String, Want> = HashMap::new();
    expected.insert("first_name".into(), want1.clone());
    expected.insert(want2.clone());

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

    let mut expected: HashMap<String, Want> = HashMap::new();
    expected.insert(want1.clone());
    expected.insert(want2.clone());

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

    let mut expected: HashMap<String, Want> = HashMap::new();
    expected.insert(want1);
    expected.insert(want2);
    expected.insert(want3);
    expected.insert(want4);

    let actual = parse_query(query)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn can_parse_object_projection_with_single_field() -> Result<(), CastleError> {
    let query = "me {
        first_name
    }";
    
    let mut fields = Vec::new();
    fields.push(Want::new_single_field("first_name".into(), None).into());
    let mut expected: HashMap<String, Want> = HashMap::new();
    let want = Want::Projection(ObjectProjection {
        identifier: Some("me".into()),
        fields
    });
    expected.insert(want.clone());
    
    let actual = parse_query(query)?;

    assert_eq!(expected.get(&want), actual.get(&want));
    return Ok(())
}

#[test]
fn can_parse_complex_object_projection_with_two_fields() {
    let query = "me {
        first_name,
        last_name
    }";
    
        let mut fields = Vec::new();
        fields.push(Want::new_single_field("first_name".into(), None).into());
        fields.push(Want::new_single_field("last_name".into(), None).into());
    
        let mut expected: HashMap<String, Want> = HashMap::new();
        expected.insert(Want::Projection(ObjectProjection {
            identifier: Some("me".into()),
            fields
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
    
        let mut fields = Vec::new();
        fields.push(Want::new_single_field("first_name".into(), None).into());
        fields.push(Want::new_single_field("last_name".into(), None).into());
        fields.push(Want::new_single_field("role".into(), None).into());
    
        let mut expected: HashMap<String, Want> = HashMap::new();
        expected.insert(Want::Projection(ObjectProjection {
            identifier: Some("me".into()),
            fields
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
        lets_gooo";
    
        let mut fields = Vec::new();
        fields.push(Want::new_single_field("lol".into(), None).into());

        let mut expected: HashMap<String, Want> = HashMap::new();
        expected.insert(Want::Projection(ObjectProjection {
            identifier: Some("me".into()),
            fields
        }));
        expected.insert(Want::new_single_field("lets_gooo".into(), None));


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
    
        let mut fields = Vec::new();
        fields.push(Want::new_single_field("first_name".into(), None).into());

        let mut expected: HashMap<String, Want> = HashMap::new();
        expected.insert(Want::Projection(ObjectProjection {
            identifier: Some("me".into()),
            fields
        }));

        let mut fields = Vec::new();
        fields.push(Want::new_single_field("username".into(), None).into());
        fields.push(Want::new_single_field("log_in_count".into(), None).into());
        expected.insert(Want::Projection(ObjectProjection {
            identifier: Some("user".into()),
            fields
        }));
        expected.insert(Want::new_single_field("location".into(), None).into());
        expected.insert(Want::new_single_field("device".into(), None).into());

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
}";

    let mut fields = Vec::new();
    fields.push(Want::new_single_field("first_name".into(), None).into());
    fields.push(Want::new_single_field("last_name".into(), None).into());
    fields.push(Want::new_single_field("email".into(), None).into());
    fields.push(Want::new_single_field("profile_picture".into(), Some(vec![PrimitiveValue::UInt(48)])).into());

    let mut expected: HashMap<String, Want> = HashMap::new();
    expected.insert(Want::Projection(ObjectProjection {
        identifier: Some("me".into()),
        fields
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
    }
}";

    let mut fields = Vec::new();
    fields.push(Want::new_single_field("first_name".into(), None).into());
    fields.push(Want::new_single_field("last_name".into(), None).into());
    fields.push(Want::new_single_field("email".into(), None).into());
    fields.push(Want::new_single_field("color".into(), Some(vec![
        PrimitiveValue::UInt(0),
        PrimitiveValue::UInt(0),
        PrimitiveValue::Float( F64 { integer_part: 10, decimal_part: 4 } )
    ])).into());

    let mut expected: HashMap<String, Want> = HashMap::new();
    expected.insert(Want::Projection(ObjectProjection {
        identifier: Some("me".into()),
        fields
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
        email
    }
}";

    let mut fields = Vec::new();
    fields.push(Want::new_projection("name".into(), vec![
        Want::new_single_field("first_name".into(), None).into(),
        Want::new_single_field("last_name".into(), None).into()
    ]).into());
    fields.push(Want::new_single_field("email".into(), None).into());
    fields.push(Want::new_single_field("profile_picture".into(), Some(vec![PrimitiveValue::UInt(48)])).into());

    let mut expected: HashMap<String, Want> = HashMap::new();
    expected.insert(Want::Projection(ObjectProjection {
        identifier: "me".into(),
        fields
    }));
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_object_projection_with_match() {
    let query = "me {
    first_name
    last_name
    email
    profile_picture(48)
    icon match {
        SVGIcon {

        }
        Emoji {

        }
    }
}";

    let mut fields = Vec::new();
    fields.push(Want::new_single_field("first_name".into(), None).into());
    fields.push(Want::new_single_field("last_name".into(), None).into());
    fields.push(Want::new_single_field("email".into(), None).into());
    fields.push(Want::new_single_field("profile_picture".into(), Some(vec![PrimitiveValue::UInt(48)])).into());
    fields.push(Want::new_single_field("email".into(), None).into());
    // need to add match functionality in parser before we can write the last field
    // make sure you add this before starting testing
    let mut expected: HashMap<String, Want> = HashMap::new();
    expected.insert(Want::Projection(ObjectProjection {
        identifier: Some("me".into()),
        fields
    }));
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual);
}