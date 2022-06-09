use std::collections::{HashMap, HashSet};


use super::{TypeDefinition, EnumDefinition, DirectiveDefinition, InputTypeDefinition};

#[derive(Debug, PartialEq, Clone)]
pub struct SchemaDefinition {
    pub types: HashMap<Box<str>, TypeDefinition>,
    pub enums: HashMap<Box<str>, EnumDefinition>,
    pub input_types: HashMap<Box<str>, InputTypeDefinition>,
    pub directives: HashMap<Box<str>, DirectiveDefinition>,
    pub scalars: HashSet<Box<str>>,
}

impl SchemaDefinition {
    pub fn new() -> SchemaDefinition {
        SchemaDefinition {
            types: HashMap::new(),
            enums: HashMap::new(),
            input_types: HashMap::new(),
            directives: HashMap::new(),
            scalars: HashSet::new(),
        }
    }
    pub fn kind_is_registered(&self, item: &str) -> bool {
        self.types.contains_key(item)
        || self.enums.contains_key(item)
        || self.input_types.contains_key(item)
        || self.directives.contains_key(item)
        || self.scalars.contains(item)
    }
    pub fn register_type(&mut self, item: TypeDefinition) {
        self.types.insert(item.ident.clone(), item);
    }
    pub fn register_input_type(&mut self, item: InputTypeDefinition) {
        self.input_types.insert(item.ident.clone(), item);
    }
    pub fn register_enum(&mut self, item: EnumDefinition) {
        self.enums.insert(item.ident.clone(), item);
    }
    pub fn register_directive_definition(&mut self, item: DirectiveDefinition) {
        self.directives.insert(item.ident.clone(), item);
    }
    pub fn register_scalar(&mut self, item: Box<str>) {
        self.scalars.insert(item.clone());
    }
    
}

