use std::collections::HashMap;

use super::types::schema_field::{SchemaField, Type};

pub fn create_type_fields_for_tests(fields: Vec<(Box<str>, Type)>) -> HashMap<Box<str>, SchemaField>{
    let mut type_fields = HashMap::new();
    for (identifier, type_) in fields {
        type_fields.insert(identifier, SchemaField { 
            name: identifier, 
            type_,
            directives: None
        });
    }
    return type_fields
}