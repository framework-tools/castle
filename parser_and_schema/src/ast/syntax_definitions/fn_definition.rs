use std::collections::HashMap;

use crate::parsers::schema_parser::types::type_system::Type;

use super::argument::ArgumentOrTuple;




#[derive(Debug, PartialEq)]
pub struct FnDefinition {
    pub name: Box<str>,
    pub args: HashMap<Box<str>, ArgumentOrTuple>,
    pub return_type: Type,
}

impl FnDefinition {
    pub fn new(name: Box<str>, args: HashMap<Box<str>, ArgumentOrTuple>, return_type: Type) -> Self {
        Self {
            name,
            args,
            return_type,
        }
    }
}
