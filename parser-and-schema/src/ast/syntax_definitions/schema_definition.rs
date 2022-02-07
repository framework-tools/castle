use std::collections::HashMap;

use super::enum_definition::EnumDefinition;
use super::trait_definition::TraitDefinition;
use super::type_definition::TypeDefinition;
use super::impl_definition::ImplDefinition;
use super::fn_definition::FnDefinition;

#[derive(Debug, PartialEq)]
pub struct SchemaDefinition {
    pub types: HashMap<String, TypeDefinition>,
    pub traits: HashMap<String, TraitDefinition>,
    pub enums: HashMap<String, EnumDefinition>,
    pub impls: HashMap<String, ImplDefinition>,
    pub fns: HashMap<String, FnDefinition>,
}