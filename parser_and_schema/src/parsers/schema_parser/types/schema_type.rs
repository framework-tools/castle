use std::collections::HashMap;

use crate::ast::syntax_definitions::field_definition::FieldDefinition;


#[derive(Debug, PartialEq)]
pub struct SchemaType {
    pub identifier: Box<str>,
    pub fields: HashMap<Box<str>, FieldDefinition>,
}


