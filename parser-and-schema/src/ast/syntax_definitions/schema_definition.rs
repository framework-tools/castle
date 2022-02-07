use std::collections::HashMap;

use crate::parser::schema_parser::types::schema_type::SchemaType;

use super::enum_definition::EnumDefinition;
use super::trait_definition::TraitDefinition;
use super::fn_definition::FnDefinition;

#[derive(Debug, PartialEq)]
pub struct SchemaDefinition {
    pub schema_types: HashMap<Box<str>, SchemaType>,
    pub traits: HashMap<String, TraitDefinition>,
    pub enums: HashMap<String, EnumDefinition>,
    //pub impls: HashMap<String, ImplDefinition>,
    pub fns: HashMap<String, FnDefinition>,
}

impl SchemaDefinition {
    pub fn new() -> SchemaDefinition {
        SchemaDefinition {
            schema_types: HashMap::new();
        }
    }
}