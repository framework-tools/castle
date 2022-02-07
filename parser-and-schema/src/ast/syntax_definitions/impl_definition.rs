use std::collections::HashMap;

use crate::parser::schema_parser::types::schema_field::Type;

use super::fn_definition::FnDefinition;





#[derive(Debug, PartialEq)]
pub struct ImplDefinition {
    pub impl_trait: Option<Type>,
    pub impl_for: Type,
    pub functions: HashMap<Box<str>, FnDefinition>
}