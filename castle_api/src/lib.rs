#![feature(if_let_guard)]
pub use castle_query_parser::Input;
pub use castle_query_parser::{Field, Inputs, Projection};
use tokio::sync::{oneshot, mpsc};
use std::fmt::Debug;
use std::future::Future;
pub use types::value::Value;

pub use crate::castle::Castle;
pub use castle_tokenizer::{Number, Primitive};

pub mod castle;
pub(crate) mod executor;
pub mod types;
pub(crate) mod validation;

impl<Ctx, E> PartialEq for dyn Resolver<Ctx, E> {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
    }
}

impl<Ctx, E> Debug for dyn Resolver<Ctx, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Resolver {}", std::any::type_name::<Self>())
    }
}

//A resolver takes in a field and context and returns the resolved value
#[async_trait::async_trait]
pub trait Resolver<Ctx, E>: Send + Sync {
    async fn resolve(&self, field: &Field, ctx: &Ctx) -> Result<Value<Ctx, E>, E>;
    async fn resolve_recursively(&self, field: &Field, ctx: &Ctx) -> Result<Value<Ctx, E>, E> where
        Ctx: Send + Sync,
        E: Send + Sync + 'static {
        match self.resolve(field, ctx).await? {
            Value::Resolver(resolver) => resolver.resolve_recursively(field, ctx).await,
            value => Ok(value),
        }
    }
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
impl<F, Ctx: Send + Sync, E: Send + Sync> Resolver<Ctx, E> for F
where
    F: for<'a, 'b> Fn2Args<&'a Field, &'b Ctx> + Sync + Send,
    for<'a, 'b> <F as Fn2Args<&'a Field, &'b Ctx>>::Output:
        Future<Output = Result<Value<Ctx, E>, E>> + Send,
{
    async fn resolve(&self, field: &Field, ctx: &Ctx) -> Result<Value<Ctx, E>, E> {
        self(field, ctx).await
    }
}
pub struct Next<Ctx, E> {
    pub(crate) sender: mpsc::Sender<oneshot::Sender<Result<Value<Ctx, E>, E>>>,
}

impl<Ctx, E> Next<Ctx, E> {
    async fn resolve(self) -> Result<Value<Ctx, E>, E> {
        let (sender, receiver) = oneshot::channel();
        self.sender.send(sender);
        receiver.await.unwrap()
    }
}

#[async_trait::async_trait]
pub trait Directive<Ctx, E>: Send + Sync {
    async fn field_visitor(
        &self,
        field: &Field,
        directive_args: &Inputs,
        next: Next<Ctx, E>,
        context: &Ctx,
    ) -> Result<Value<Ctx, E>, E>
    where
        Ctx: Send + Sync,
        E: Send + Sync + 'static
    {
        unimplemented!()
    }
}
