
use std::collections::HashMap;

use crate::parsers::schema_parser::types::type_system::Type;


#[derive(Debug, PartialEq)]
pub enum DirectiveOnValue {
    Field,
    EnumVariant,
    // Type,
}

#[derive(Debug, PartialEq)]
pub struct DirectiveDefinition {
    pub name: Box<str>,
    pub args: HashMap<Box<str>, Type>,
    pub on: DirectiveOnValue
}


impl DirectiveDefinition {
    pub fn new(name: Box<str>, args: HashMap<Box<str>, Type>, on: DirectiveOnValue) -> Self {
        Self {
            name,
            args,
            on
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct Directive {
    pub name: Box<str>,
    pub arguments: HashMap<Box<str>, Type>
}

impl Directive {
    pub fn new(name: Box<str>, arguments: HashMap<Box<str>, Type>) -> Self {
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