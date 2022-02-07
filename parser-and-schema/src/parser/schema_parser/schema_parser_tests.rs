use crate::parser::{schema_parser::types::{schema_field::{SchemaField, PrimitiveType, Type}, schema_type::SchemaType}, self};




#[cfg(test)]
#[test]
fn can_parse_empty_query() {
    use std::collections::HashMap;

    use crate::parser::schema_parser::parse_schema::parse_schema;

    let query = "";
    let expected = HashMap::new();
    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual.schema_types);
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
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid),
            directives: None,
        },
    );
    user_fields.insert(
        "name".into(),
        SchemaField {
            name: "name".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String),
            directives: None,
        },
    );
    user_fields.insert(
        "age".into(),
        SchemaField {
            name: "age".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int),
            directives: None
        },
    );
    let mut expected = HashMap::new();
    expected.insert("User".into(), SchemaType {
        identifier: "User".into(),
        fields: user_fields,
    }); 
    
    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual.schema_types);
}

#[test]
fn can_parse_simple_type_more_fields_and_no_commas() {
    use std::collections::HashMap;

    use crate::parser::schema_parser::parse_schema::parse_schema;

    let query = "
        type User {
            id: uuid
            name: String
            age: Int
            is_admin: bool
            location: String
            log_in_count: Int
        }";

    let mut user_fields = HashMap::new();
    user_fields.insert(
        "id".into(),
        SchemaField {
            name: "id".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid),
            directives: None,
        },
    );
    user_fields.insert(
        "name".into(),
        SchemaField {
            name: "name".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String),
            directives: None,
        },
    );
    user_fields.insert(
        "age".into(),
        SchemaField {
            name: "age".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int),
            directives: None,
        },
    );
    user_fields.insert(
        "is_admin".into(),
        SchemaField {
            name: "is_admin".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Bool),
            directives: None,
        },
    );
    user_fields.insert(
        "location".into(),
        SchemaField {
            name: "location".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String),
            directives: None,
        },
    );
    user_fields.insert(
        "log_in_count".into(),
        SchemaField {
            name: "log_in_count".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int),
            directives: None,
        },
    );
    let mut expected = HashMap::new();
    expected.insert("User".into(), SchemaType {
        identifier: "User".into(),
        fields: user_fields,
    }); 
    
    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual.schema_types);
}

#[test]
fn can_parse_two_types() {
    use std::collections::HashMap;

    use crate::parser::schema_parser::parse_schema::parse_schema;

    let query = "
        type User {
            id: uuid,
            name: String,
            age: Int,
            is_admin: bool,
            location: String,
            log_in_count: Int
        }
        
        type Organization {
            id: uuid,
            name: String,
            industry: String,
        }";

    let mut user_fields = HashMap::new();
    user_fields.insert(
        "id".into(),
        SchemaField {
            name: "id".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid),
            directives: None,
        },
    );
    user_fields.insert(
        "name".into(),
        SchemaField {
            name: "name".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String),
            directives: None,
        },
    );
    user_fields.insert(
        "age".into(),
        SchemaField {
            name: "age".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int),
            directives: None,
        },
    );
    user_fields.insert(
        "is_admin".into(),
        SchemaField {
            name: "is_admin".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Bool),
            directives: None,
        },
    );
    user_fields.insert(
        "location".into(),
        SchemaField {
            name: "location".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String),
            directives: None,
        },
    );
    user_fields.insert(
        "log_in_count".into(),
        SchemaField {
            name: "log_in_count".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int),
            directives: None,
        },
    );
    let mut expected: HashMap<Box<str>, SchemaType> = HashMap::new();
    expected.insert("User".into(), SchemaType {
        identifier: "User".into(),
        fields: user_fields,
    }); 
    
    let mut organization_fields: HashMap<Box<str>, SchemaField> = HashMap::new();
    organization_fields.insert(
        "id".into(),
        SchemaField {
            name: "id".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid),
            directives: None,
        },
    );
    organization_fields.insert(
        "name".into(),
        SchemaField {
            name: "name".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String),
            directives: None,
        },
    );
    organization_fields.insert(
        "industry".into(),
        SchemaField {
            name: "industry".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String),
            directives: None,
        },
    );
    expected.insert("Organization".into(), SchemaType {
        identifier: "Organization".into(),
        fields: organization_fields,
    });
    
    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual.schema_types);
}

#[test]
fn can_parse_two_types_with_defined_value() {
    use std::collections::HashMap;

    use crate::parser::schema_parser::parse_schema::parse_schema;

    let query = "
        type User {
            id: uuid,
            name: String,
            age: Int,
            is_admin: bool,
            location: String,
            log_in_count: Int
            organization: Organization
        }
        
        type Organization {
            id: uuid,
            name: String,
            industry: String,
        }";

    let mut user_fields = HashMap::new();
    user_fields.insert(
        "id".into(),
        SchemaField {
            name: "id".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid),
            directives: None,
        },
    );
    user_fields.insert(
        "name".into(),
        SchemaField {
            name: "name".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String),
            directives: None,
        },
    );
    user_fields.insert(
        "age".into(),
        SchemaField {
            name: "age".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int),
            directives: None,
        },
    );
    user_fields.insert(
        "is_admin".into(),
        SchemaField {
            name: "is_admin".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Bool),
            directives: None,
        },
    );
    user_fields.insert(
        "location".into(),
        SchemaField {
            name: "location".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String),
            directives: None,
        },
    );
    user_fields.insert(
        "log_in_count".into(),
        SchemaField {
            name: "log_in_count".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int),
            directives: None,
        },
    );
    user_fields.insert(
        "organization".into(),
        SchemaField {
            name: "organization".into(),
            type_: parser::schema_parser::types::schema_field::Type::SchemaType("Organization".into()),
            directives: None,
        },
    );
    let mut expected: HashMap<Box<str>, SchemaType> = HashMap::new();
    expected.insert("User".into(), SchemaType {
        identifier: "User".into(),
        fields: user_fields,
    }); 
    
    let mut organization_fields: HashMap<Box<str>, SchemaField> = HashMap::new();
    organization_fields.insert(
        "id".into(),
        SchemaField {
            name: "id".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid),
            directives: None,
        },
    );
    organization_fields.insert(
        "name".into(),
        SchemaField {
            name: "name".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String),
            directives: None,
        },
    );
    organization_fields.insert(
        "industry".into(),
        SchemaField {
            name: "industry".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String),
            directives: None,
        },
    );
    expected.insert("Organization".into(), SchemaType {
        identifier: "Organization".into(),
        fields: organization_fields,
    });
    
    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual.schema_types);
}


#[test]
fn parser_breaks_if_unknown_schema_type() {
    use std::collections::HashMap;

    use crate::parser::schema_parser::parse_schema::parse_schema;
    // In the User field organization,
    // Company is an undefined schema type
    // Therefore, this should throw an error to notify the engineer
    let query = "
        type User {
            organization: Company,
            id: uuid,
            name: String,
            age: Int,
            is_admin: bool,
            location: String,
            log_in_count: Int,
        }
        
        type Organization {
            id: uuid,
            name: String,
            industry: String,
        }";

    let mut user_fields = HashMap::new();
    user_fields.insert(
        "id".into(),
        SchemaField {
            name: "id".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid),
            directives: None,
        },
    );
    user_fields.insert(
        "name".into(),
        SchemaField {
            name: "name".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String),
            directives: None,
        },
    );
    user_fields.insert(
        "age".into(),
        SchemaField {
            name: "age".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int),
            directives: None,
        },
    );
    user_fields.insert(
        "is_admin".into(),
        SchemaField {
            name: "is_admin".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Bool),
            directives: None,
        },
    );
    user_fields.insert(
        "location".into(),
        SchemaField {
            name: "location".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String),
            directives: None,
        },
    );
    user_fields.insert(
        "log_in_count".into(),
        SchemaField {
            name: "log_in_count".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int),
            directives: None,
        },
    );
    user_fields.insert(
        "organization".into(),
        SchemaField {
            name: "organization".into(),
            type_: parser::schema_parser::types::schema_field::Type::SchemaType("Organization".into()),
            directives: None,
        },
    );
    let mut expected: HashMap<Box<str>, SchemaType> = HashMap::new();
    expected.insert("User".into(), SchemaType {
        identifier: "User".into(),
        fields: user_fields,
    }); 
    
    let mut organization_fields: HashMap<Box<str>, SchemaField> = HashMap::new();
    organization_fields.insert(
        "id".into(),
        SchemaField {
            name: "id".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid),
            directives: None,
        },
    );
    organization_fields.insert(
        "name".into(),
        SchemaField {
            name: "name".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String),
            directives: None,
        },
    );
    organization_fields.insert(
        "industry".into(),
        SchemaField {
            name: "industry".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String),
            directives: None,
        },
    );
    expected.insert("Organization".into(), SchemaType {
        identifier: "Organization".into(),
        fields: organization_fields,
    });
    
    let actual = parse_schema(query);
    assert!(actual.is_err());
}


#[test]
fn can_parse_two_types_with_vec_type() {
    use std::collections::HashMap;

    use crate::parser::schema_parser::parse_schema::parse_schema;

    let query = "
        type User {
            id: uuid,
            name: String,
            age: Int,
            is_admin: bool,
            location: String,
            log_in_count: Int
        }
        
        type Organization {
            id: uuid,
            name: String,
            industries: Vec<String>,
            users: Vec<User>
        }";

    let mut user_fields = HashMap::new();
    user_fields.insert(
        "id".into(),
        SchemaField {
            name: "id".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid),
            directives: None,
        },
    );
    user_fields.insert(
        "name".into(),
        SchemaField {
            name: "name".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String),
            directives: None,
        },
    );
    user_fields.insert(
        "age".into(),
        SchemaField {
            name: "age".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int),
            directives: None,
        },
    );
    user_fields.insert(
        "is_admin".into(),
        SchemaField {
            name: "is_admin".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Bool),
            directives: None,
        },
    );
    user_fields.insert(
        "location".into(),
        SchemaField {
            name: "location".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String),
            directives: None,
        },
    );
    user_fields.insert(
        "log_in_count".into(),
        SchemaField {
            name: "log_in_count".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Int),
            directives: None,
        },
    );
    let mut expected = HashMap::new();
    expected.insert("User".into(), SchemaType {
        identifier: "User".into(),
        fields: user_fields,
    }); 
    
    let mut organization_fields: HashMap<Box<str>, SchemaField> = HashMap::new();
    organization_fields.insert(
        "id".into(),
        SchemaField {
            name: "id".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::Uuid),
            directives: None,
        },
    );
    organization_fields.insert(
        "name".into(),
        SchemaField {
            name: "name".into(),
            type_: parser::schema_parser::types::schema_field::Type::PrimitiveType(PrimitiveType::String),
            directives: None,
        },
    );
    organization_fields.insert(
        "users".into(),
        SchemaField {
            name: "users".into(),
            type_: parser::schema_parser::types::schema_field::Type::VecType(Type::SchemaType("User".into()).into()),
            directives: None,
        },
    );

    organization_fields.insert(
        "industries".into(),
        SchemaField {
            name: "industries".into(),
            type_: parser::schema_parser::types::schema_field::Type::VecType(Type::PrimitiveType(PrimitiveType::String).into()),
            directives: None,
        },
    );
    let actual = parse_schema(query).unwrap();
    assert_eq!(expected, actual.schema_types);
}


// To Implement:
// - Parse Enums (Unit types)
// - Parse Enums with primitive values
// - Parse Enums with type values - incl checking for undefined types
// - Parse functions
// - Parse directives
// - Parse implements
// - Parse traits ???