use std::collections::HashMap;

use crate::parsers::schema_parser::types::type_system::Type;

use super::{argument::{IdentifierAndTypeArgument}, directive_definition::Directive};




#[derive(Debug, PartialEq)]
pub struct FnDefinition {
    pub name: Box<str>,
    pub args: HashMap<Box<str>, IdentifierAndTypeArgument>,
    pub return_type: Type,
    pub directives: Vec<Directive>
}

impl FnDefinition {
    pub fn new(name: Box<str>, args: HashMap<Box<str>, IdentifierAndTypeArgument>, return_type: Type, directives: Vec<Directive>) -> Self {
        Self {
            name,
            args,
            return_type,
            directives,
        }
    }
}
