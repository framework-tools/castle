use std::collections::{HashMap};

use crate::parser::schema_parser::types::schema_field::Type;

use super::{directive_definition::DirectiveDefinition};


#[derive(Debug, PartialEq)]
pub struct EnumDefinition {
    pub name: Box<str>,
    pub variants: HashMap<Box<str>, EnumData>,
    pub directives: HashMap<Box<str>, DirectiveDefinition>
}

#[derive(Debug, PartialEq)]
pub struct EnumData {
    pub name: Box<str>,
    pub variant: EnumVariant,
    pub directives: HashMap<Box<str>, DirectiveDefinition>
}

#[derive(Debug, PartialEq)]
pub enum EnumVariant {
    EnumUnit,
    EnumTuple(Type),
    // EnumObject
    // EnumValue
    // EnumTuple
}