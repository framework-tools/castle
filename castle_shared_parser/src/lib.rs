use std::{collections::HashMap, fmt::Display};

pub use castle_tokenizer::Primitive;

pub mod parse_inputs;


// (ident: primitive, ident2: primitive)
// (ident: { ident: value, ident2: value })
// (ident: [ value, value ])
// (ident: Variant { // this is a map variant
//   ident: value
// })
// (ident: Variant (value, value), ident_2: Primitive) // this is a tuple variant
// (ident: Variant, ident_2: primitive) // is a unit variant

#[derive(Debug, PartialEq, Clone)]
pub enum Input {
    Primitive(Primitive),
    Variant(Variant),
    Map(HashMap<Box<str>, Input>),
    List(Vec<Input>),
}

impl Input {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Input::Primitive(Primitive::String(str)) => return Some(&**str),
            _ => None,
        }
    }
    pub fn as_map(&self) -> Option<&HashMap<Box<str>, Input>> {
        match self {
            Input::Map(map) => return Some(map),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&Vec<Input>> {
        match self {
            Input::List(list) => return Some(list),
            _ => None,
        }
    }
    pub fn as_variant(&self) -> Option<&Variant> {
        match self {
            Input::Variant(variant) => return Some(variant),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variant {
    ident: Box<str>,
    value: VariantType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VariantType {
    Unit,
    Tuple(Vec<Input>),
    Map(HashMap<Box<str>, Input>),
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Input::Primitive(primitive) => write!(f, "{}", primitive),
            Input::Variant(variant) => write!(f, "{:#?}", variant),
            Input::Map(map) => write!(f, "{:#?}", map),
            Input::List(list) => write!(f, "{}", list.iter().map(|item| format!("{}", item)).collect::<Vec<String>>().join(", ")),
        }
    }
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ident)?;
        match &self.value {
            VariantType::Unit => write!(f, "()"),
            VariantType::Tuple(tuple) => write!(f, "({})", tuple.iter().map(|val| format!("{}", val)).collect::<Vec<String>>().join(", ")),
            VariantType::Map(map) => write!(f, "{:#?}", map),
        }
    }
}
