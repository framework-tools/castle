use crate::{ast::syntax_definitions::directive_definition::DirectiveDefinition};

use super::type_system::Type;

#[derive(Debug, PartialEq)]
pub struct SchemaField {
    pub name: Box<str>,
    pub type_: Type,
    pub directives: Option<DirectiveDefinition>
}

impl SchemaField {
    pub fn new(name: Box<str>, type_: Type, directives: Option<DirectiveDefinition>) -> Self {
        SchemaField {
            name,
            type_,
            directives
        }
    }
}