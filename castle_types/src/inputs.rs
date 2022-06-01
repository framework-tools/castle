use std::{collections::HashMap, fmt::Display};

use crate::Primitive;


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

pub type Inputs = HashMap<Box<str>, Input>;

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

impl From<&Input> for Option<String> {
    fn from(input: &Input) -> Self {
        match input {
            Input::Primitive(Primitive::String(str)) => Some(String::from(&**str)),
            _ => None,
        }
    }
}

impl From<&Input> for String {
    fn from(input: &Input) -> Self {
        let opt: Option<String> = input.into();
        opt.unwrap()
    }
}

// Implement From for all the primitive numeric types
macro_rules! impl_from_input {
    ($($t:ty, $as:ident),*) => {
        $(
            impl From<&Input> for $t {
                fn from(value: &Input) -> Self {
                    match value {
                        Input::Primitive(Primitive::Number(number)) => number.into(),
                        _ => panic!("Cannot convert input to {}", stringify!($t)),
                    }
                }
            }
        )*
    };
}

impl_from_input!(
    i8, as_i64,
    i16, as_i64,
    i32, as_i64,
    i64, as_i64,
    u8, as_u64,
    u16, as_u64,
    u32, as_u64,
    u64, as_u64,
    f32, as_f64,
    f64, as_f64,
    usize, as_u64,
    isize, as_i64
);


#[derive(Debug, PartialEq, Clone)]
pub struct Variant {
    pub ident: Box<str>,
    pub value: VariantType,
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