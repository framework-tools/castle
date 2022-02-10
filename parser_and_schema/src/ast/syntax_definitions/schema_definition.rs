use std::collections::HashMap;

use crate::parsers::schema_parser::types::schema_type::SchemaType;

use super::enum_definition::EnumDefinition;
use super::fn_definition::FnDefinition;

#[derive(Debug, PartialEq)]
pub struct SchemaDefinition {
    pub schema_types: HashMap<Box<str>, SchemaType>,
    pub enums: HashMap<Box<str>, EnumDefinition>,
    pub functions: HashMap<Box<str>, FnDefinition>,
}

impl SchemaDefinition {
    pub fn new() -> SchemaDefinition {
        SchemaDefinition {
            schema_types: HashMap::new(),
            enums: HashMap::new(),
            functions: HashMap::new(),
        }
    }
}