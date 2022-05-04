#![feature(if_let_guard)]
use std::{collections::HashMap, fmt::Debug};
use castle_error::CastleError;
use dyn_partial_eq::{DynPartialEq, dyn_partial_eq};
use query_parser::{Field, Inputs, Projection};

mod validation;
pub mod castle;

#[derive(Debug)]
pub enum Value<Ctx> {
    Empty,
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    String(String),
    Vec(Vec<Value<Ctx>>),
    Object(HashMap<Box<str>, Value<Ctx>>),
    Resolver(Box<dyn Resolver<Ctx>>),
}

impl <Ctx: Debug> PartialEq for Value<Ctx> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::UInt(l0), Self::UInt(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Vec(l0), Self::Vec(r0)) => l0 == r0,
            (Self::Object(l0), Self::Object(r0)) => l0 == r0,
            (Self::Resolver(l0), Self::Resolver(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl<Ctx> PartialEq for dyn Resolver<Ctx> + {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
    }
}

//A resolver takes in fields (inner wants), arguments and context and returns the resolved want
// pub type Resolver<Ctx: Debug, F: Fn> = Fn(&mut Field, &Ctx) -> Result<Value<Ctx>, CastleError>;
#[async_trait::async_trait]
pub trait Resolver<Ctx: Debug>: Send + Sync + Debug {
    async fn resolve(&self, field: &Field, ctx: &Ctx) -> Result<Value<Ctx>, CastleError>;
}

// This allows closures to impl the resolver trait
#[async_trait::async_trait]
impl <Ctx: Debug + Sync, F: Fn(&Field, &Ctx) -> Result<Value<Ctx>, CastleError> + Debug + Send + Sync> Resolver<Ctx> for F {
    async fn resolve(&self, field: &Field, ctx: &Ctx) -> Result<Value<Ctx>, CastleError> {
        self(field, ctx)
    }
}

#[async_trait::async_trait]
pub trait Directive<Ctx: Send + 'static + Debug>: Send + Sync {
    async fn field_visitor(&self, field: &Field, directive_args: &Inputs, value: Box<dyn Resolver<Ctx>>, context: Ctx) -> Result<Value<Ctx>, CastleError> {
        unimplemented!()
    }
}
