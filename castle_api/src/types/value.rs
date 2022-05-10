use std::{fmt::Debug, collections::HashMap};
use castle_tokenizer::Number;
use castle_query_parser::Field;

use crate::Resolver;


#[derive(Debug)]
pub enum Value<Ctx, E> {
    Bool(bool),
    Number(Number),
    String(String),
    Vec(Vec<Value<Ctx, E>>),
    Object(HashMap<Box<str>, Value<Ctx, E>>),
    Resolver(Box<dyn Resolver<Ctx, E>>),
}

impl <Ctx, E> From<Number> for Value<Ctx, E> {
    fn from(number: Number) -> Self {
        Value::Number(number)
    }
}

impl <Ctx, E> PartialEq for Value<Ctx, E> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Vec(l0), Self::Vec(r0)) => l0 == r0,
            (Self::Object(l0), Self::Object(r0)) => l0 == r0,
            (Self::Resolver(l0), Self::Resolver(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

// Implement From for all the primitive numeric types
macro_rules! impl_from_primitive {
    ($($t:ty),*) => {
        $(
            impl<Ctx, E> From<$t> for Value<Ctx, E> {
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

impl<Ctx, E> Value<Ctx, E> {
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

    pub fn as_vec(self) -> Option<Vec<Value<Ctx, E>>> {
        match self {
            Self::Vec(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_object(self) -> Option<HashMap<Box<str>, Value<Ctx, E>>> {
        match self {
            Self::Object(o) => Some(o),
            _ => None,
        }
    }
}


impl<Ctx, E> From<bool> for Value<Ctx, E> {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl<Ctx, E> From<String> for Value<Ctx, E> {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl<Ctx, E> From<&str> for Value<Ctx, E> {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl<Ctx, VT, E> From<Vec<VT>> for Value<Ctx, E> where VT: Into<Value<Ctx, E>> {
    fn from(value: Vec<VT>) -> Self {
        Value::Vec(value.into_iter().map(Into::into).collect())
    }
}

impl<Ctx, IntoV: Into<Value<Ctx, E>>, AsStr: AsRef<str>, E> From<HashMap<AsStr, IntoV>> for Value<Ctx, E> {
    fn from(value: HashMap<AsStr, IntoV>) -> Self {
        Value::Object(value.into_iter().map(|(k, v)| (k.as_ref().into(), v.into())).collect())
    }
}

impl<Ctx, E> From<Box<dyn Resolver<Ctx, E>>> for Value<Ctx, E> {
    fn from(value: Box<dyn Resolver<Ctx, E>>) -> Self {
        Value::Resolver(value)
    }
}

#[async_trait::async_trait]
impl<Ctx: Sync + Send, E: Send + Sync> Resolver<Ctx, E> for Value<Ctx, E> {
    async fn resolve(&self, _field_: &Field, _ctx: &Ctx) -> Result<Value<Ctx, E>, E> {
        unimplemented!()
    }
}