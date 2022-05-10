use std::collections::HashMap;

use super::{TypeDefinition, EnumDefinition, DirectiveDefinition, InputTypeDefinition};

#[derive(Debug, PartialEq, Clone)]
pub struct SchemaDefinition {
    pub types: HashMap<Box<str>, TypeDefinition>,
    pub enums: HashMap<Box<str>, EnumDefinition>,
    pub input_types: HashMap<Box<str>, InputTypeDefinition>,
    pub directives: HashMap<Box<str>, DirectiveDefinition>,
}

impl SchemaDefinition {
    pub fn new() -> SchemaDefinition {
        SchemaDefinition {
            types: HashMap::new(),
            enums: HashMap::new(),
            input_types: HashMap::new(),
            directives: HashMap::new()
        }
    }
}