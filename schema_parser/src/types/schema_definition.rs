use std::collections::HashMap;

use super::{TypeDefinition, EnumDefinition, DirectiveDefinition};

#[derive(Debug)]
pub struct SchemaDefinition {
    types: HashMap<String, TypeDefinition>,
    enums: HashMap<String, EnumDefinition>,
    directives: HashMap<String, DirectiveDefinition>,
}