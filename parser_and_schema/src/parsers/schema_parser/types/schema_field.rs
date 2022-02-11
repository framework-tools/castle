use crate::{ast::syntax_definitions::directive_definition::Directive};

use super::type_system::Type;

#[derive(Debug, PartialEq)]
pub struct SchemaField {
    pub name: Box<str>,
    pub type_: Type,
    pub directives: Vec<Directive>
}

impl SchemaField {
    pub fn new(name: Box<str>, type_: Type, directives: Vec<Directive>) -> Self {
        SchemaField {
            name,
            type_,
            directives
        }
    }
}