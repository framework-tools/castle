use std::collections::{HashMap};

use crate::parser::schema_parser::types::schema_field::Type;

use super::{directive_definition::DirectiveDefinition};


#[derive(Debug, PartialEq)]
pub struct EnumDefinition {
    pub name: Box<str>,
    pub variants: HashMap<Box<str>, EnumVariant>,
    pub directives: HashMap<Box<str>, DirectiveDefinition>
}

#[derive(Debug, PartialEq)]
pub struct EnumVariant {
    pub name: Box<str>,
    pub enum_data_type: EnumDataType,
    pub directives: HashMap<Box<str>, DirectiveDefinition>
}

#[derive(Debug, PartialEq)]
pub enum EnumDataType {
    EnumUnit,
    EnumTuple(Type),
    // EnumObject
    // EnumValue
    // EnumTuple
}