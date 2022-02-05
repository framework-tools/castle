use std::collections::HashMap;

use super::schema_field::SchemaField;

#[derive(Debug, PartialEq)]
pub struct SchemaType {
    pub identifier: Box<str>,
    pub fields: HashMap<Box<str>, SchemaField>,
}