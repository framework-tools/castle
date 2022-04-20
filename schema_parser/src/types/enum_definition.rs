use std::collections::HashMap;

use super::{Directive, Kind};



#[derive(Debug, PartialEq)]
pub struct EnumDefinition {
    pub name: Box<str>,
    pub variants: Vec<VariantDefinition>,
    pub directives: Vec<Directive>,
}

#[derive(Debug, PartialEq)]
pub struct VariantDefinition {
    pub name: Box<str>,
    pub kind: VariantKindDefinition,
    pub directives: Vec<Directive>,
}

#[derive(Debug, PartialEq)]
pub enum VariantKindDefinition {
    Unit,
    Tuple(Vec<Kind>),
    Map(HashMap<Box<str>, Kind>),
}