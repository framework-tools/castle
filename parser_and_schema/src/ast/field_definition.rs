use std::collections::HashMap;

use crate::parsers::schema_parser::types::parse_type::Type;

use super::{directive_definition::Directive};




#[derive(Debug, PartialEq)]
pub struct FieldDefinition {
    pub name: Box<str>,
    pub args: HashMap<Box<str>, Type>,
    pub return_type: Type,
    pub directives: Vec<Directive>
}

impl FieldDefinition {
    pub fn new(name: Box<str>, args: HashMap<Box<str>, Type>, return_type: Type, directives: Vec<Directive>) -> Self {
        Self {
            name,
            args,
            return_type,
            directives,
        }
    }
}
