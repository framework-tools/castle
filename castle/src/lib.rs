#![feature(if_let_guard)]
use std::{fmt::Debug};
pub use query_parser::{Field, Inputs, Projection};
pub use types::value::Value;

mod validation;
pub mod castle;
pub mod types;

impl<Ctx, E: Debug> PartialEq for dyn Resolver<Ctx, E> + {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
    }
}

impl<Ctx, E: Debug> Debug for dyn Resolver<Ctx, E> + {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Resolver {}", std::any::type_name::<Self>())
    }
}


//A resolver takes in fields (inner wants), arguments and context and returns the resolved want
#[async_trait::async_trait]
pub trait Resolver<Ctx: Debug, E: Debug>: Send + Sync {
    async fn resolve(&self, field: &Field, ctx: &Ctx) -> Result<Value<Ctx, E>, E>;
}


// This allows closures to impl the resolver trait
#[async_trait::async_trait]
impl<Ctx, F, E: Debug> Resolver<Ctx, E> for F
where
    Ctx: Debug + Sync + Send + 'static,
    F: Fn(&Field, &Ctx) -> Result<Value<Ctx, E>, E> + Send + Sync + 'static,
{
    async fn resolve(&self, field: &Field, ctx: &Ctx) -> Result<Value<Ctx, E>, E> {
        self(field, ctx)
    }
}

#[async_trait::async_trait]
pub trait Directive<Ctx: Send + 'static + Debug, E: Debug + 'static>: Send + Sync {
    async fn field_visitor(&self, field: &Field, directive_args: &Inputs, value: Box<dyn Resolver<Ctx, E>>, context: Ctx) -> Result<Value<Ctx, E>, E> {
        unimplemented!()
    }
}
