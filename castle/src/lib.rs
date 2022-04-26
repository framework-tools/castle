use std::collections::HashMap;
use castle_error::CastleError;
use query_parser::{Field, Inputs, Projection};

mod validation;
pub mod castle;

#[derive(Debug, PartialEq)]
pub enum Value<C, R = ()> {
    Empty,
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    String(String),
    Vec(Vec<Value<R>>),
    Object(HashMap<Box<str>, Value<R>>),
    Resolver(Resolver<C, R>),
    Custom(Box<R>)
}

//A resolver takes in fields (inner wants), arguments and context and returns the resolved want
pub type Resolver<C, R> = fn(Field, C) -> Result<Value<R>, CastleError>;

#[async_trait::async_trait]
trait Directive<C, R> {
    // async fn input_visitor(&self, directive_args: &Inputs, field: &Field, context: C) -> Result<Value<R>, CastleError>;
    // async fn
    async fn field_visitor(&self, field: &Field, directive_args: &Inputs, value: Resolver<C, R>, context: C) -> Result<Value<R>, CastleError> {
        // value().await
        unimplemented!()
    }
}
