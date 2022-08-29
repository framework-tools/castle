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
    fn from(value: &Input) -> Self {
        match value {
            Input::Variant(Variant { ident, value: VariantType::Tuple(tuple)}) if &**ident == "Some" => {
                match tuple.first() {
                    Some(item) => Some(item.into()),
                    _ => panic!("Expected value in tuple"),
                }
            }
            Input::Variant(Variant { ident, value: VariantType::Unit}) if &**ident == "None" => None,
            _ => panic!("Expected variant 'Some(..)' or 'None'"),
        }
    }
}

impl From<&Input> for String {
    fn from(input: &Input) -> Self {
        match input {
            Input::Primitive(Primitive::String(str)) => str.to_string(),
            _ => panic!("Expected string"),
        }
    }
}

impl<'a> From<&'a Input> for Option<&'a str> {
    fn from(value: &'a Input) -> Self {
        match value {
            Input::Variant(Variant { ident, value: VariantType::Tuple(tuple)}) if &**ident == "Some" => {
                match tuple.first() {
                    Some(item) => Some(item.into()),
                    _ => panic!("Expected value in tuple"),
                }
            }
            Input::Variant(Variant { ident, value: VariantType::Unit}) if &**ident == "None" => None,
            _ => panic!("Expected variant 'Some(..)' or 'None'"),
        }
    }
}

impl<'a> From<&'a Input> for &'a str {
    fn from(input: &'a Input) -> Self {
        match input {
            Input::Primitive(Primitive::String(str)) => &**str,
            _ => panic!("Expected string"),
        }
    }
}

impl From<&Input> for bool {
    fn from(input: &Input) -> Self {
        match input {
            Input::Primitive(Primitive::Boolean(bool)) => *bool,
            _ => panic!("Expected bool"),
        }
    }
}

impl From<&Input> for Option<bool> {
    fn from(input: &Input) -> Self {
        match input {
            Input::Variant(Variant { ident, value: VariantType::Tuple(tuple)}) if &**ident == "Some" => {
                match tuple.first() {
                    Some(item) => Some(item.into()),
                    _ => panic!("Expected value in tuple"),
                }
            }
            Input::Variant(Variant { ident, value: VariantType::Unit}) if &**ident == "None" => None,
            _ => panic!("Expected variant 'Some(..)' or 'None'"),
        }
    }
}

impl<'a, T: From<&'a Input>> From<&'a Input> for Vec<T> {
    fn from(input: &'a Input) -> Self {
        match input {
            Input::List(list) => list.iter().map(|input| T::from(input)).collect(),
            _ => vec![],
        }
    }
}

// Implement From for all the primitive numeric types
macro_rules! impl_from_input {
    ($($t:ty),*) => {
        $(
            impl From<&Input> for Option<$t> {
                fn from(value: &Input) -> Self {
                    match value {
                        Input::Variant(Variant { ident, value: VariantType::Tuple(tuple)}) if &**ident == "Some" => {
                            match tuple.first() {
                                Some(item) => Some(item.into()),
                                _ => panic!("Expected value in tuple"),
                            }
                        }
                        Input::Variant(Variant { ident, value: VariantType::Unit}) if &**ident == "None" => None,
                        _ => panic!("Expected variant 'Some(..)' or 'None'"),
                    }
                }
            }

            impl From<&Input> for $t {
                fn from(input: &Input) -> Self {
                    match input {
                        Input::Primitive(Primitive::Number(number)) => number.clone().into(),
                        _ => panic!("Expected number"),
                    }
                }
            }
        )*
    };
}

impl_from_input!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, usize, isize);

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
            Input::List(list) => write!(
                f,
                "{}",
                list.iter()
                    .map(|item| format!("{}", item))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ident)?;
        match &self.value {
            VariantType::Unit => write!(f, "()"),
            VariantType::Tuple(tuple) => write!(
                f,
                "({})",
                tuple
                    .iter()
                    .map(|val| format!("{}", val))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            VariantType::Map(map) => write!(f, "{:#?}", map),
        }
    }
}
