use serde::{Deserialize, Serialize};

use super::expressions::PrimitiveValue;



#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DirectiveDefinition {
    pub name: String,
    pub arguments: Vec<String, DirectiveArgument>
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DirectiveArgument {
    pub name: String,
    pub value: PrimitiveValue
}
