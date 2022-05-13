#![feature(if_let_guard)]
use std::future::Future;
use std::{fmt::Debug};
pub use castle_query_parser::Input;
pub use castle_query_parser::{Field, Inputs, Projection};
pub use types::value::Value;

pub use crate::castle::Castle;
pub use castle_tokenizer::{Primitive, Number};

mod validation;
pub mod castle;
pub mod types;

impl<Ctx, E> PartialEq for dyn Resolver<Ctx, E> + {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
    }
}

impl<Ctx, E> Debug for dyn Resolver<Ctx, E> + {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Resolver {}", std::any::type_name::<Self>())
    }
}


//A resolver takes in a field and context and returns the resolved value
#[async_trait::async_trait]
pub trait Resolver<Ctx, E>: Send + Sync {
    async fn resolve(&self, field: &Field, ctx: &Ctx) -> Result<Value<Ctx, E>, E>;
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
    for<'a, 'b> <F as Fn2Args<&'a Field, &'b Ctx>>::Output: Future<Output = Result<Value<Ctx, E>, E>> + Send,
{
    async fn resolve(&self, field: &Field, ctx: &Ctx) -> Result<Value<Ctx, E>, E> {
        self(field, ctx).await
    }
}


#[async_trait::async_trait]
pub trait Directive<Ctx: Send + 'static, E: 'static>: Send + Sync {
    async fn field_visitor(&self, _field: &Field, _directive_args: &Inputs, _value: Box<dyn Resolver<Ctx, E>>, _context: Ctx) -> Result<Value<Ctx, E>, E> {
        unimplemented!()
    }
}
