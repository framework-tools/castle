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
    pub fn is_type_registered(&self, item: &str) -> bool {
        if self.types.contains_key(item) {
            return true
        } else { return false }
    }
    pub fn register_type(&mut self, item: TypeDefinition) {
        self.types.insert(item.ident.clone(), item);
    }
    pub fn register_input(&mut self, item: InputTypeDefinition) {
        self.input_types.insert(item.ident.clone(), item);
    }
}