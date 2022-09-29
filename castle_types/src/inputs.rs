use std::{collections::HashMap, fmt::Display};

use crate::{Primitive, CastleError};

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

impl TryFrom<&Input> for Option<String> {
    type Error = CastleError;

    fn try_from(input: &Input) -> Result<Self, Self::Error> {
        match input {
            Input::Variant(Variant { ident, value: VariantType::Tuple(tuple)}) if &**ident == "Some" => {
                match tuple.first() {
                    Some(item) => Ok(Some(item.try_into()?)),
                    _ => Err(CastleError::Validation("Expected value in tuple".into())),
                }
            },
            Input::Variant(Variant { ident, value: VariantType::Unit}) if &**ident == "None" => Ok(None),
            _ => Err(CastleError::Validation("Expected variant 'Some(..)' or 'None'".into())),
        }
    }
}

impl TryFrom<&Input> for String {
    type Error = CastleError;

    fn try_from(input: &Input) -> Result<Self, Self::Error> {
        match input {
            Input::Primitive(Primitive::String(str)) => Ok(str.to_string()),
            _ => Err(CastleError::Validation("Expected string".into())),
        }
    }
}

impl<'a> TryFrom<&'a Input> for Option<&'a str> {
    type Error = CastleError;

    fn try_from(input: &'a Input) -> Result<Self, Self::Error> {
        match input {
            Input::Variant(Variant { ident, value: VariantType::Tuple(tuple)}) if &**ident == "Some" => {
                match tuple.first() {
                    Some(item) => Ok(Some(item.try_into()?)),
                    _ => Err(CastleError::Validation("Expected value in tuple".into())),
                }
            }
            Input::Variant(Variant { ident, value: VariantType::Unit}) if &**ident == "None" => Ok(None),
            _ => Err(CastleError::Validation("Expected variant 'Some(..)' or 'None'".into())),
        }
    }
}

impl<'a> TryFrom<&'a Input> for &'a str {
    type Error = CastleError;

    fn try_from(input: &'a Input) -> Result<Self, Self::Error> {
        match input {
            Input::Primitive(Primitive::String(str)) => Ok(&**str),
            _ => Err(CastleError::Validation("Expected string".into())),
        }
    }
}

impl TryFrom<&Input> for bool {
    type Error = CastleError;

    fn try_from(input: &Input) -> Result<Self, Self::Error> {
        match input {
            Input::Primitive(Primitive::Boolean(bool)) => Ok(*bool),
            _ => Err(CastleError::Validation("Expected boolean".into())),
        }
    }
}

impl TryFrom<&Input> for Option<bool> {
    type Error = CastleError;

    fn try_from(input: &Input) -> Result<Self, Self::Error> {
        match input {
            Input::Variant(Variant { ident, value: VariantType::Tuple(tuple)}) if &**ident == "Some" => {
                match tuple.first() {
                    Some(item) => Ok(Some(item.try_into()?)),
                    _ => panic!("Expected value in tuple"),
                }
            }
            Input::Variant(Variant { ident, value: VariantType::Unit}) if &**ident == "None" => Ok(None),
            _ => Err(CastleError::Validation("Expected variant 'Some(..)' or 'None'".into())),
        }
    }
}

impl<'a, T: TryFrom<&'a Input, Error = CastleError>> TryFrom<&'a Input> for Vec<T> {
    type Error = CastleError;
    fn try_from(input: &'a Input) -> Result<Self, Self::Error> {
        match input {
            Input::List(list) => list.iter().map(|input| T::try_from(input)).collect(),
            _ => Ok(vec![]),
        }
    }
}

// Implement From for all the primitive numeric types
macro_rules! impl_from_input {
    ($($t:ty),*) => {
        $(
            impl TryFrom<&Input> for Option<$t> {
                type Error = CastleError;

                fn try_from(input: &Input) -> Result<Self, Self::Error> {
                    match input {
                        Input::Variant(Variant { ident, value: VariantType::Tuple(tuple)}) if &**ident == "Some" => {
                            match tuple.first() {
                                Some(item) => Ok(Some(item.try_into()?)),
                                _ => Err(CastleError::Validation("Expected value in tuple".into())),
                            }
                        }
                        Input::Variant(Variant { ident, value: VariantType::Unit}) if &**ident == "None" => Ok(None),
                        _ => Err(CastleError::Validation("Expected variant 'Some(..)' or 'None'".into())),
                    }
                }
            }

            impl TryFrom<&Input> for $t {
                type Error = CastleError;

                fn try_from(input: &Input) -> Result<Self, Self::Error> {
                    match input {
                        Input::Primitive(Primitive::Number(number)) => Ok(number.clone().into()),
                        _ => Err(CastleError::Validation("Expected number".into())),
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
            Input::Variant(variant) => write!(f, "{}", variant),
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
