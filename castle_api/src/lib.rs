#![feature(if_let_guard)]
pub use castle_query_parser::Input;
pub use castle_query_parser::{Field, Inputs, Projection};
use context::Context;
use tokio::sync::{oneshot, mpsc};
use std::fmt::Debug;
use std::future::Future;
pub use types::value::Value;

pub use crate::castle::Castle;
pub use castle_tokenizer::{Number, Primitive};

pub mod castle;
pub mod context;
pub mod types;
pub(crate) mod executor;
pub(crate) mod validation;

impl PartialEq for dyn Resolver {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
    }
}

impl Debug for dyn Resolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Resolver {}", std::any::type_name::<Self>())
    }
}

//A resolver takes in a field and context and returns the resolved value
#[async_trait::async_trait]
pub trait Resolver: Send + Sync {
    async fn resolve(&self, field: &Field, ctx: &Context) -> Result<Value, anyhow::Error>;
}

/// ### Allows async closures to `impl` the [Resolver] trait.
///
/// This is ***dark magic***. Don't ask how it works I won't be able to explain it.
/// This is needed in order to make any async functions work
/// with the add_resolver function.
///
/// They are called higher-ranked trait bounds.
trait Fn2Args<Arg1, Arg2>: Fn(Arg1, Arg2) -> <Self as Fn2Args<Arg1, Arg2>>::Output {
    type Output;
}
impl<F: Fn(Arg1, Arg2) -> Out, Arg1, Arg2, Out> Fn2Args<Arg1, Arg2> for F {
    type Output = Out;
}

#[async_trait::async_trait]
impl<F> Resolver for F
where
    F: for<'a, 'b> Fn2Args<&'a Field, &'b Context> + Sync + Send,
    for<'a, 'b> <F as Fn2Args<&'a Field, &'b Context>>::Output:
        Future<Output = Result<Value, anyhow::Error>> + Send,
{
    async fn resolve(&self, field: &Field, ctx: &Context) -> Result<Value, anyhow::Error> {
        self(field, ctx).await
    }
}
pub struct Next {
    pub(crate) sender: mpsc::Sender<oneshot::Sender<Result<Value, anyhow::Error>>>,
}

impl Next {
    async fn resolve(self) -> Result<Value, anyhow::Error> {
        let (sender, receiver) = oneshot::channel();
        self.sender.send(sender).await?;
        receiver.await?
    }
}

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
