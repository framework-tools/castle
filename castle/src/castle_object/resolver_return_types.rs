use std::collections::HashMap;
use parser_and_schema::ast::syntax_definitions::enum_definition::EnumValue;
use crate::resolvers::resolver_type::Resolver;

#[derive(Debug, PartialEq)]
pub enum Value<C, R = ()> {
    Empty,
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    String(String),
    EnumValue(EnumValue),
    Vec(Vec<Value<R>>),
    Object(HashMap<Box<str>, Value<R>>),
    Resolver(Resolver<C, R>),
    Custom(Box<R>)
}