use std::collections::HashMap;


use crate::parsers::schema_parser::types::schema_type::SchemaType;

use super::directive_definition::{ DirectiveDefinition};
use super::enum_definition::EnumDefinition;
use super::field_definition::FieldDefinition;

#[derive(Debug, PartialEq)]
pub struct SchemaDefinition {
    pub schema_types: HashMap<Box<str>, SchemaType>,
    pub enums: HashMap<Box<str>, EnumDefinition>,
    pub functions: HashMap<Box<str>, FieldDefinition>,
    pub directives: HashMap<Box<str>, DirectiveDefinition>
}

impl SchemaDefinition {
    pub fn new() -> SchemaDefinition {
        SchemaDefinition {
            schema_types: HashMap::new(),
            enums: HashMap::new(),
            functions: HashMap::new(),
            directives: HashMap::new()
        }
    }
}