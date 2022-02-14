use std::collections::HashMap;

use crate::parsers::schema_parser::types::type_system::Type;

use super::argument::Argument;




#[derive(Debug, PartialEq)]
pub struct FnDefinition {
    pub name: Box<str>,
    pub args: HashMap<Box<str>, Argument>,
    pub return_type: Type,
}

impl FnDefinition {
    pub fn initalise() -> Self {
        Self {
            name: "".to_string().into(),
            args: HashMap::new(),
            return_type: None,
        }
    }
    pub fn new(name: Box<str>, args: Option<Vec<Argument>>, return_type: Option<Type>) -> Self {
        Self {
            name,
            args,
            return_type,
        }
    }
}
