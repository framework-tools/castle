use std::{fmt::Debug, collections::HashMap};

use crate::{Field, Context, Value};

#[async_trait::async_trait]
pub trait ResolvesFields: Send + Sync {
    async fn resolve(&self, field: &Field, ctx: &Context) -> Result<Value, anyhow::Error>;
}

impl<RF: ResolvesFields + 'static> From<RF> for Value {
    fn from(object: RF) -> Value {
        Value::ResolveFields(Box::new(object))
    }
}

impl Debug for dyn ResolvesFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResolvesFields<{:?}>", std::any::type_name::<Self>())
    }
}

#[async_trait::async_trait]
impl ResolvesFields for HashMap<Box<str>, Value> {
    async fn resolve(&self, field: &Field, ctx: &Context) -> Result<Value, anyhow::Error> {

    }
}