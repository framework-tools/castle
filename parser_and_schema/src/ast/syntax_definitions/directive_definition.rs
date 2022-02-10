
use crate::parsers::schema_parser::types::type_system::Type;

use super::argument::{Argument};


#[derive(Debug, PartialEq)]
pub struct DirectiveDefinition {
    pub name: Box<str>,
    pub arguments: Option<Vec<Argument>>
}

impl DirectiveDefinition {
    pub fn new(name: Box<str>, arguments: Option<Vec<Argument>>) -> Self {
        DirectiveDefinition {
            name,
            arguments
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Into {
    pub type_: Type
}

impl Into {
    pub fn new(type_: Type) -> Self {
        Into {
            type_
        }
    }
}