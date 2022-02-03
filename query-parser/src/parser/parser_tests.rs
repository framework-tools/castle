use std::hash::Hash;
use std::collections::HashSet;

use crate::{parser::parse_query::parse_query, ast::syntax_definitions::want::{Want, ObjectProjection}};



#[cfg(test)]
#[test]
fn can_parse_empty_query() {
    use std::collections::HashSet;

    let query = "";
    let expected = HashSet::new();
    let actual = parse_query(query).unwrap();
    loop {
        for key in expected {
            let key_is_present = actual.contains(key);
        }
    }
}

#[test]
fn can_parse_single_field() {
    let query = "first_name";
    
        let mut fields = HashSet::new();
        fields.insert("first_name".into());
    
        let mut expected: HashSet<Want> = HashSet::new();
        expected.insert(Want::SingleField(Box::<str>::from("first_name")));

        let actual = parse_query(query).unwrap();
        assert_eq!(expected, actual);
}

#[test]
fn can_parse_two_fields() {
    let query = "first_name, last_name";
    
        let mut fields = HashSet::new();
        fields.insert("first_name".into());
        fields.insert("last_name".into());

    
        let mut expected: HashSet<Want> = HashSet::new();
        expected.insert(Want::SingleField(Box::<str>::from("first_name")));
        expected.insert(Want::SingleField(Box::<str>::from("last_name")));


        let actual = parse_query(query).unwrap();
        assert_eq!(expected, actual);
}

#[test]
fn can_parse_complex_object_projection_with_single_field() {
    let query = "me {
        first_name
    }";
    
        let mut fields = HashSet::new();
        fields.insert("first_name".into());
    
        let mut expected: HashSet<Want> = HashSet::new();
        expected.insert(Want::ObjectProjection(ObjectProjection {
            identifier: "me".into(),
            fields
        }));
        let actual = parse_query(query).unwrap();
        assert_eq!(expected, actual);
}

#[test]
fn can_parse_complex_object_projection_with_two_fields() {
    let query = "me {
        first_name
        last_name
    }";
    
        let mut fields = HashSet::new();
        fields.insert("first_name".into());
        fields.insert("last_name".into());

    
        let mut expected: HashSet<Want> = HashSet::new();
        expected.insert(Want::ObjectProjection(ObjectProjection {
            identifier: "me".into(),
            fields
        }));

        let actual = parse_query(query).unwrap();
        assert_eq!(expected, actual);
}

#[test]
fn can_parse_complex_object_projection() {
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

    let mut fields = HashSet::new();
    fields.insert("first_name".into());
    fields.insert("last_name".into());
    fields.insert("email".into());
    fields.insert("profile_picture".into());
    fields.insert("icon".into());

    let mut expected: HashSet<Want> = HashSet::new();
    expected.insert(Want::ObjectProjection(ObjectProjection {
        identifier: "me".into(),
        fields
    }));
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual);
}
//working one
#[test]
fn can_parse_object_and_single_field() {
    let query = "me {
        first_name
        }
        username";
    
        let mut fields = Vec::new();
        fields.push(Box::new(Want::SingleField("first_name".into())));

        let mut expected: HashSet<Want> = HashSet::new();
        expected.insert(Want::Projection(ObjectProjection {
            identifier: Some("me".into()),
            fields
        }));
        expected.insert(Want::SingleField(Box::<str>::from("username")));


        let actual = parse_query(query).unwrap();
        assert_eq!(expected, actual);
}

#[test]
fn can_parse_two_objects_and_two_fields() {
    let query = "me {
        first_name
        }
        user {
            username
        }
        location
        device";
    
        let mut fields = Vec::new();
        fields.push("first_name".into());

        let mut expected: Vec<Want> = Vec::new();
        expected.push(Want::ObjectProjection(ObjectProjection {
            identifier: "me".into(),
            fields
        }));
        expected.insert(Want::ObjectProjection(ObjectProjection {
            identifier: "user".into(),
            fields
        }));
        expected.insert(Want::SingleField(Box::<str>::from("location")));
        expected.insert(Want::SingleField(Box::<str>::from("device")));

        let actual = parse_query(query).unwrap();
        assert_eq!(expected, actual);
}