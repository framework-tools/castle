use std::collections::HashMap;

use crate::parsers::schema_parser::types::{schema_field::SchemaField, type_system::Type};

use super::directive_definition::Directive;

#[derive(Debug, PartialEq)]
pub struct TraitDefinition {
    pub name: Type,
    pub fields: HashMap<Box<str>, SchemaField>,
    pub directives: HashMap<Box<str>, Directive>,
}

