use crate::parser::{schema_parser::types::{schema_field::{SchemaField, PrimitiveType, Type}, schema_type::SchemaType}, self};




#[cfg(test)]
#[test]
fn can_parse_empty_query() {
    use std::collections::HashMap;

    use crate::parser::schema_parser::parse_schema::parse_schema;

    let query = "";
    let expected = HashMap::new();
    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn can_parse_simple_type() {
    use std::collections::HashMap;

    use crate::parser::schema_parser::parse_schema::parse_schema;

    let query = "
        type User {
            id: uuid,
            name: String,
            age: Int,
        }";

    let mut user_fields = HashMap::new();
    user_fields.insert(
        "id".into(),
        SchemaField {
            name: "id".into(),
            schema_type: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid),
        },
    );
    user_fields.insert(
        "name".into(),
        SchemaField {
            name: "name".into(),
            schema_type: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String),
        },
    );
    user_fields.insert(
        "age".into(),
        SchemaField {
            name: "age".into(),
            schema_type: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int),
        },
    );
    let mut expected = HashMap::new();
    expected.insert("id".into(), Type {
        identifier: "User".into(),
        fields: user_fields,
    }); 
    
    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual);
}

