
use crate::parsers::schema_parser::types::type_system::Type;

use super::{argument::{Argument}, fn_definition::FnDefinition};


#[derive(Debug, PartialEq)]
pub struct DirectiveDefinition {
    pub function: FnDefinition,
}

#[derive(Debug, PartialEq)]
pub struct Directive {
    pub name: Box<str>,
    pub arguments: Option<Vec<Argument>>
}

impl Directive {
    pub fn new(name: Box<str>, arguments: Option<Vec<Argument>>) -> Self {
        Directive {
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