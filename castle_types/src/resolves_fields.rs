use std::{fmt::Debug};

use crate::{Field, State, Value};

#[async_trait::async_trait]
pub trait ResolvesFields: Send + Sync {
    async fn resolve(&self, field: &Field, ctx: &State) -> Result<Value, anyhow::Error>;
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

