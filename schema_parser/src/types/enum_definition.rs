use std::collections::HashMap;

use super::{Directive, Kind};



#[derive(Debug)]
pub struct EnumDefinition {
    pub name: Box<str>,
    pub variants: Vec<VariantDefinition>,
    pub directives: Vec<Directive>,
}

#[derive(Debug)]
pub struct VariantDefinition {
    pub name: Box<str>,
    pub kind: VariantKindDefinition,
    pub directives: Vec<Directive>,
}

#[derive(Debug)]
pub enum VariantKindDefinition {
    Unit,
    Tuple(Vec<Kind>),
    Map(HashMap<Box<str>, Kind>),
}