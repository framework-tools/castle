use std::collections::{HashMap};

use crate::parser::schema_parser::types::schema_field::SchemaField;

use super::{directive_definition::DirectiveDefinition};

use super::{argument::Argument};


#[derive(Debug, PartialEq)]
pub struct EnumDefinition {
    pub name: Box<str>,
    pub variants: HashMap<Box<str>, EnumVariant>,
    pub directives: HashMap<Box<str>, DirectiveDefinition>
}

impl EnumDefinition {
    pub fn new(name: Box<str>, variants: HashMap<Box<str>, EnumVariant>, directives: HashMap<Box<str>, DirectiveDefinition>) -> Self {
        EnumDefinition {
            name,
            variants,
            directives
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct EnumVariant {
    pub name: Box<str>,
    pub enum_data_type: EnumDataType,
    pub directives: HashMap<Box<str>, DirectiveDefinition>
}

impl EnumVariant {
    pub fn new(name: Box<str>, enum_data_type: EnumDataType, directives: HashMap<Box<str>, DirectiveDefinition>) -> Self {
        EnumVariant {
            name,
            enum_data_type,
            directives
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum EnumDataType {
    EnumUnit,
    EnumTuple(Vec<Argument>),
    EnumObject(HashMap<Box<str>, SchemaField>)
}

impl EnumDataType {
    pub fn new_enum_object(fields_in_vec: Vec<(Box<str>, SchemaField)>) -> Self {
        let mut fields = HashMap::new();
        for (identifier, field) in fields_in_vec {
            fields.insert(identifier, field);
        }
        return EnumDataType::EnumObject(fields)
    }
}