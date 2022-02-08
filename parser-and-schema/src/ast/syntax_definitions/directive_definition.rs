
use crate::parser::schema_parser::types::type_system::Type;


#[derive(Debug, PartialEq)]
pub struct DirectiveDefinition {
    pub type_: Type
}
impl DirectiveDefinition {
    pub fn new(type_: Type) -> Self {
        DirectiveDefinition {
            type_
        }
    }
}
