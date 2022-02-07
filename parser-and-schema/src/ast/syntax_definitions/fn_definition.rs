use crate::parser::schema_parser::types::type_system::Type;

use super::argument::Argument;




#[derive(Debug, PartialEq)]
pub struct FnDefinition {
    pub name: Box<str>,
    pub args: Option<Vec<Argument>>,
    pub return_type: Option<Type>,
    pub body: Vec<FnStatement>
}

impl FnDefinition {
    pub fn new() -> Self {
        Self {
            name: "".to_string().into(),
            args: None,
            return_type: None,
            body: Vec::new()
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct FnStatement; // TODO: statements
