use crate::{Field, Context, Value};


pub trait ResolvesFields {
    fn resolve(&self, field: &Field, ctx: &Context) -> Result<Value, anyhow::Error>;
}