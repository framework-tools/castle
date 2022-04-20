use std::collections::HashMap;

use super::{TypeDefinition, EnumDefinition, DirectiveDefinition};

#[derive(Debug, PartialEq)]
pub struct SchemaDefinition {
    pub types: HashMap<Box<str>, TypeDefinition>,
    pub enums: HashMap<Box<str>, EnumDefinition>,
    pub directives: HashMap<Box<str>, DirectiveDefinition>,
}

impl SchemaDefinition {
    pub fn new() -> SchemaDefinition {
        SchemaDefinition {
            types: HashMap::new(),
            enums: HashMap::new(),
            directives: HashMap::new()
        }
    }
}