#![feature(if_let_guard)]
use std::collections::HashMap;
use castle_error::CastleError;
use query_parser::{Field, Inputs, Projection};

mod validation;
pub mod castle;

#[derive(Debug, PartialEq)]
pub enum Value<Ctx> {
    Empty,
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    String(String),
    Vec(Vec<Value<Ctx>>),
    Object(HashMap<Box<str>, Value<Ctx>>),
    Resolver(Resolver<Ctx>),
}

//A resolver takes in fields (inner wants), arguments and context and returns the resolved want
pub type Resolver<Ctx> = fn(Field, Ctx) -> Result<Value<Ctx>, CastleError>;

#[async_trait::async_trait]
pub trait Directive<Ctx: Send + 'static>: Send + Sync {
    async fn field_visitor(&self, field: &Field, directive_args: &Inputs, value: Resolver<Ctx>, context: Ctx) -> Result<Value<Ctx>, CastleError> {
        unimplemented!()
    }
}