use std::collections::HashMap;

use crate::Primitive;

#[derive(Debug, PartialEq)]
pub struct Input {
    ident: Box<str>,
    value: InputValue,
}

#[derive(Debug, PartialEq)]
pub enum InputValue {
    Primitive(Primitive),
    Variant(Variant),
    Map(HashMap<Box<str>, InputValue>),
    List(Vec<InputValue>),
}

#[derive(Debug, PartialEq)]
pub struct Variant {
    ident: Box<str>,
    value: VariantType,
}

#[derive(Debug, PartialEq)]
pub enum VariantType {
    Unit,
    Tuple(Vec<InputValue>),
    Map(HashMap<Box<str>, InputValue>),
}