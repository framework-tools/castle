
use crate::parser::schema_parser::types::type_system::Type;


#[derive(Debug, PartialEq)]
pub struct DirectiveDefinition {
    pub name: Box<str>,
    pub type_: Type
}
impl DirectiveDefinition {
    pub fn new(name: Box<str>, type_: Type) -> Self {
        DirectiveDefinition {
            name,
            type_
        }
    }
}
