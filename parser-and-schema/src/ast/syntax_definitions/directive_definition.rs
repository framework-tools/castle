use serde::{Deserialize, Serialize};

use super::expressions::PrimitiveValue;



#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DirectiveDefinition {
    pub name: Box<str>,
    pub arguments: Vec<DirectiveArgument>
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DirectiveArgument {
    pub name: Box<str>,
    pub value: PrimitiveValue
}
