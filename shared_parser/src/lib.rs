use std::{collections::HashMap, fmt::Display};

pub use tokenizer::Primitive;

pub mod parse_inputs;


// (ident: primitive, ident2: primitive)
// (ident: { ident: value, ident2: value })
// (ident: [ value, value ])
// (ident: Variant { // this is a map variant
//   ident: value
// })
// (ident: Variant (value, value), ident_2: Primitive) // this is a tuple variant
// (ident: Variant, ident_2: primitive) // is a unit variant

#[derive(Debug, PartialEq)]
pub enum Input {
    Primitive(Primitive),
    Variant(Variant),
    Map(HashMap<Box<str>, Input>),
    List(Vec<Input>),
}

#[derive(Debug, PartialEq)]
pub struct Variant {
    ident: Box<str>,
    value: VariantType,
}

#[derive(Debug, PartialEq)]
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
            Input::List(list) => write!(f, "{:#?}", list),
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