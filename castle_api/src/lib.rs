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


//A resolver takes in fields (inner wants), arguments and context and returns the resolved want
#[async_trait::async_trait]
pub trait Resolver<Ctx, E>: Send + Sync {
    async fn resolve(&self, field: &Field, ctx: &Ctx) -> Result<Value<Ctx, E>, E>;
}

// This allows closures to impl the resolver trait
#[async_trait::async_trait]
impl<Ctx, F, E, Fut> Resolver<Ctx, E> for F
where
    Ctx: Debug + Sync + Send,
    F: Fn(&Field, &Ctx) -> Fut + Send + Sync,
    Fut: Future<Output = Result<Value<Ctx, E>, E>> + Send,
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
