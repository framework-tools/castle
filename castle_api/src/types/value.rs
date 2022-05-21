use std::{fmt::Debug, collections::HashMap};
use castle_tokenizer::Number;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum Value {
    Bool(bool),
    Number(Number),
    String(String),
    Vec(Vec<Value>),
    Object(HashMap<Box<str>, Value>),
    Void,
}

impl  From<Number> for Value {
    fn from(number: Number) -> Self {
        Value::Number(number)
    }
}

impl  PartialEq for Value {
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
    ($($t:ty),*) => {
        $(
            impl From<$t> for Value {
                fn from(value: $t) -> Self {
                    Value::Number(value.into())
                }
            }
        )*
    };
}

impl_from_primitive!(i8);
impl_from_primitive!(i16);
impl_from_primitive!(i32);
impl_from_primitive!(i64);
impl_from_primitive!(u8);
impl_from_primitive!(u16);
impl_from_primitive!(u32);
impl_from_primitive!(u64);
impl_from_primitive!(f32);
impl_from_primitive!(f64);

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_number(self) -> Option<Number> {
        match self {
            Self::Number(n) => Some(n.clone()),
            _ => None,
        }
    }

    pub fn as_bool(self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(b),
            _ => None,
        }
    }

    pub fn as_vec(self) -> Option<Vec<Value>> {
        match self {
            Self::Vec(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_object(self) -> Option<HashMap<Box<str>, Value>> {
        match self {
            Self::Object(o) => Some(o),
            _ => None,
        }
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