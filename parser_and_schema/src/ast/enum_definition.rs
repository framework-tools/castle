use std::collections::{HashMap};

use super::field_definition::FieldDefinition;
use super::{directive_definition::Directive};

use super::{argument::ArgumentOrTuple};


#[derive(Debug, PartialEq)]
pub struct EnumDefinition {
    pub name: Box<str>,
    pub variants: HashMap<Box<str>, EnumVariant>,
}

impl EnumDefinition {
    pub fn new(name: Box<str>, variants: HashMap<Box<str>, EnumVariant>) -> Self {
        EnumDefinition {
            name,
            variants,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct EnumVariant {
    pub name: Box<str>,
    pub enum_data_type: EnumDataType,
    pub directives: Vec<Directive>
}

impl EnumVariant {
    pub fn new(name: Box<str>, enum_data_type: EnumDataType, directives: Vec<Directive>) -> Self {
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
    EnumTuple(Vec<ArgumentOrTuple>),
    EnumObject(HashMap<Box<str>, FieldDefinition>)
}

impl EnumDataType {
    pub fn new_enum_object(fields_in_vec: Vec<(Box<str>, FieldDefinition)>) -> Self {
        let mut fields = HashMap::new();
        for (identifier, field) in fields_in_vec {
            fields.insert(identifier, field);
        }
        return EnumDataType::EnumObject(fields)
    }
}


///currently only enum unit data type is supported for use in queries
#[derive(Debug, PartialEq)]
pub struct EnumValue {
    pub identifier: Box<str>,
    pub enum_parent: Box<str>, //the name of the enum that this value is a part of
    pub variant: Box<str>, // the variant in that enum that this value is
    pub data_type: EnumDataType
}

impl EnumValue {
    pub fn new(enum_parent: Box<str>, variant: Box<str>, data_type: EnumDataType) -> Self {
        let mut identifier = enum_parent.to_string();
        identifier.push_str("::");
        identifier.push_str(&variant);
        let identifier = identifier.into();
        EnumValue {
            enum_parent,
            variant,
            data_type,
            identifier
        }
    }
}