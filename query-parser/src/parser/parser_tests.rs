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