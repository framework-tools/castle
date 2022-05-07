use std::collections::HashMap;

use super::{Directive, Kind};



#[derive(Debug, PartialEq)]
pub struct EnumDefinition {
    pub ident: Box<str>,
    pub variants: HashMap<Box<str>, VariantDefinition>,
    pub directives: Vec<Directive>,
}

#[derive(Debug, PartialEq)]
pub struct VariantDefinition {
    pub ident: Box<str>,
    pub kind: VariantKindDefinition,
    pub directives: Vec<Directive>,
}

#[derive(Debug, PartialEq)]
pub enum VariantKindDefinition {
    Unit,
    Tuple(Vec<Kind>),
    Map(HashMap<Box<str>, Kind>),
}