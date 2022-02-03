use std::hash::Hash;
use std::collections::HashSet;

use shared::CastleError;

use crate::{parser::parse_query::parse_query, ast::syntax_definitions::want::{Want, ObjectProjection}};



#[cfg(test)]
#[test]
fn can_parse_empty_query() {

    let query = "";
    let expected = HashSet::new();
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual);

}

#[test]
fn can_parse_single_field() -> Result<(), CastleError> {
    let query = "first_name";
    
    let mut fields = Vec::new();
    fields.push(Box::new(Want::SingleField("first_name".into())));

    let mut expected: HashSet<Want> = HashSet::new();
    expected.insert(Want::SingleField(Box::<str>::from("first_name")));

    let actual = parse_query(query);
    let want = Box::new(Want::SingleField("first_name".into()));
    match actual {
        Ok(actual) => {
            assert_eq!(expected.get(&*want), actual.get(&*want));
            return Ok(());
        },
        Err(e) => return Err(e)
    };
}

#[test]
fn can_parse_two_fields() {
let query = "first_name, last_name";

    let mut fields = Vec::new();
    fields.push(Box::new(Want::SingleField("first_name".into())));
    fields.push(Box::new(Want::SingleField("last_name".into())));

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
    
        let mut fields = Vec::new();
        fields.push(Box::new(Want::SingleField("first_name".into())));
    
        let mut expected: HashSet<Want> = HashSet::new();
        expected.insert(Want::Projection(ObjectProjection {
            identifier: Some("me".into()),
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
    
        let mut fields = Vec::new();
        fields.push(Box::new(Want::SingleField("first_name".into())));
        fields.push(Box::new(Want::SingleField("last_name".into())));
    
        let mut expected: HashSet<Want> = HashSet::new();
        expected.insert(Want::Projection(ObjectProjection {
            identifier: Some("me".into()),
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

    let mut fields = Vec::new();
    fields.push(Box::new(Want::SingleField("first_name".into())));
    fields.push(Box::new(Want::SingleField("last_name".into())));
    fields.push(Box::new(Want::SingleField("email".into())));
    fields.push(Box::new(Want::SingleField("profile_picture".into())));
    fields.push(Box::new(Want::SingleField("icon".into())));

    let mut expected: HashSet<Want> = HashSet::new();
    expected.insert(Want::Projection(ObjectProjection {
        identifier: Some("me".into()),
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
    let query = "
        me {
            first_name
        }
        user {
            username
        }
        location
        device";
    
        let mut fields = Vec::new();
        fields.push(Box::new(Want::SingleField("first_name".into())));

        let mut expected: HashSet<Want> = HashSet::new();
        expected.insert(Want::Projection(ObjectProjection {
            identifier: Some("me".into()),
            fields
        }));

        let mut fields = Vec::new();
        fields.push(Box::new(Want::SingleField("username".into())));

        expected.insert(Want::Projection(ObjectProjection {
            identifier: Some("user".into()),
            fields
        }));
        expected.insert(Want::SingleField(Box::<str>::from("location")));
        expected.insert(Want::SingleField(Box::<str>::from("device")));

        let actual = parse_query(query).unwrap();
        assert_eq!(expected, actual);
}