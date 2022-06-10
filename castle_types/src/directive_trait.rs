use crate::{Field, Inputs, Next, Value, Context};

#[allow(unused_variables)]
#[async_trait::async_trait]
pub trait Directive: Send + Sync {
    async fn field_visitor(
        &self,
        field: &Field,
        directive_args: &Inputs,
        next: Next,
        context: &Context,
    ) -> Result<Value, anyhow::Error>
    where
        Context: Send + Sync,

    {
        unimplemented!()
    }
}
