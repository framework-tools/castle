use std::collections::HashMap;

use crate::parsers::schema_parser::types::{parse_type::Type};

use super::{directive_definition::Directive, field_definition::FieldDefinition};

#[derive(Debug, PartialEq)]
pub struct TraitDefinition {
    pub name: Type,
    pub fields: HashMap<Box<str>, FieldDefinition>,
    pub directives: Directive,
}

