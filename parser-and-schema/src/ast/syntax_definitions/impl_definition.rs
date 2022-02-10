use std::collections::HashMap;

use super::fn_definition::FnDefinition;





#[derive(Debug, PartialEq)]
pub struct ImplDefinition {
    pub impl_for: Box<str>, //identifier for the type or enum
    pub functions: HashMap<Box<str>, FnDefinition>
}

impl ImplDefinition {
    pub fn new(impl_for: Box<str>, functions: HashMap<Box<str>, FnDefinition> ) -> ImplDefinition {
        ImplDefinition {
            impl_for,
            functions, 
        }
    }
}