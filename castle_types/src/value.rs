use std::{fmt::Debug, collections::HashMap};
use serde::{Serialize, Deserialize};

use crate::{Number, ResolvesFields, ConvertFrom};

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "", untagged)]
pub enum Value {
    Bool(bool),
    Number(Number),
    String(String),
    Vec(Vec<Value>),
    Object(HashMap<Box<str>, Value>),
    #[serde(skip)]
    ResolveFields(Box<dyn ResolvesFields>),
    Void,
}


impl<IV: Into<Value>> ConvertFrom<IV> for Result<Value, anyhow::Error> {
    fn from(value: IV) -> Self {
        Ok(value.into())
    }
}

impl<IV: Into<Value>> ConvertFrom<Result<IV, anyhow::Error>> for Result<Value, anyhow::Error> {
    fn from(value: Result<IV, anyhow::Error>) -> Self {
        value.map(Into::into)
    }
}

// impl<T> ConvertFrom<T> for T {
//     fn from(value: T) -> Self {
//         value
//     }
// }

impl From<Number> for Value {
    fn from(number: Number) -> Self {
        Value::Number(number)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Void
    }
}

impl<VT> From<Vec<VT>> for Value where VT: Into<Value> {
    fn from(value: Vec<VT>) -> Self {
        Value::Vec(value.into_iter().map(Into::into).collect())
    }
}

impl<IntoV: Into<Value>, AsStr: AsRef<str>> From<HashMap<AsStr, IntoV>> for Value {
    fn from(value: HashMap<AsStr, IntoV>) -> Self {
        Value::Object(value.into_iter().map(|(k, v)| (k.as_ref().into(), v.into())).collect())
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Vec(l0), Self::Vec(r0)) => l0 == r0,
            (Self::Object(l0), Self::Object(r0)) => l0 == r0,
            (Self::Void, Self::Void) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

// Implement From for all the primitive numeric types
macro_rules! impl_from_primitive {
    ($($t:ty,)*) => {
        $(
            impl From<$t> for Value {
                fn from(value: $t) -> Self {
                    Value::Number(value.into())
                }
            }
        )*
    };
}

impl_from_primitive!(
    usize,
    u8,
    u16,
    u32,
    u64,
    isize,
    i8,
    i16,
    i32,
    i64,
    f32,
    f64,
);