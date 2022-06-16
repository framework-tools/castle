use crate::{Field, Inputs, Next, Value, State};

#[allow(unused_variables)]
#[async_trait::async_trait]
pub trait Directive: Send + Sync {
    async fn field_visitor(
        &self,
        field: Field,
        directive_args: &Inputs,
        next: Next,
        context: &State,
    ) -> Result<Value, anyhow::Error> {
        next.resolve(field).await
    }
}
