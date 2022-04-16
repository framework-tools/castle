use std::collections::HashMap;

use super::{Directive, Kind};



#[derive(Debug)]
pub struct EnumDefinition {
    pub directives: Vec<Directive>,
    pub name: Box<str>,
    pub variants: Vec<VariantDefinition>,
}

#[derive(Debug)]
pub struct VariantDefinition {
    pub directives: Vec<Directive>,
    pub name: Box<str>,
    pub kind: VariantKindDefinition,
}

#[derive(Debug)]
pub enum VariantKindDefinition {
    Unit,
    Tuple(Vec<Kind>),
    Map(HashMap<Box<str>, Kind>),
}