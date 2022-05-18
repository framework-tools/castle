use std::collections::HashMap;

use super::{AppliedDirective, Kind};



#[derive(Debug, PartialEq, Clone)]
pub struct EnumDefinition {
    pub ident: Box<str>,
    pub variants: HashMap<Box<str>, VariantDefinition>,
    pub directives: Vec<AppliedDirective>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariantDefinition {
    pub ident: Box<str>,
    pub kind: VariantKindDefinition,
    pub directives: Vec<AppliedDirective>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VariantKindDefinition {
    Unit,
    Tuple(Vec<Kind>),
    Map(HashMap<Box<str>, Kind>),
}