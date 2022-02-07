use std::collections::HashMap;

use crate::parser::schema_parser::types::schema_type::SchemaType;

use super::enum_definition::EnumDefinition;
use super::impl_definition::ImplDefinition;
use super::trait_definition::TraitDefinition;
use super::fn_definition::FnDefinition;

#[derive(Debug, PartialEq)]
pub struct SchemaDefinition {
    pub schema_types: HashMap<Box<str>, SchemaType>,
    pub traits: HashMap<Box<str>, TraitDefinition>,
    pub enums: HashMap<Box<str>, EnumDefinition>,
    pub impls: HashMap<Box<str>, ImplDefinition>,
    pub functions: HashMap<Box<str>, FnDefinition>,
}

impl SchemaDefinition {
    pub fn new() -> SchemaDefinition {
        SchemaDefinition {
            schema_types: HashMap::new(),
            traits: HashMap::new(),
            enums: HashMap::new(),
            functions: HashMap::new(),
            impls: HashMap::new(),
        }
    }
}