use std::collections::HashSet;

use crate::parser::schema_parser::types::schema_field::Type;

use super::{directive_definition::DirectiveDefinition};


#[derive(Debug, PartialEq)]
pub struct EnumDefinition {
    pub name: String,
    pub marks: HashSet<Mark>,
    pub variants: LinkedHashMap<String, EnumData>,
    pub directives: LinkedHashMap<String, DirectiveDefinition>
}

#[derive(Debug, PartialEq)]
pub struct EnumData {
    pub name: String,
    pub marks: HashSet<Mark>,
    pub variant: EnumVariant,
    pub directives: LinkedHashMap<String, DirectiveDefinition>
}

#[derive(Debug, PartialEq)]
pub enum EnumVariant {
    EnumType(Type),
    // EnumObject
    // EnumValue
    // EnumTuple
}