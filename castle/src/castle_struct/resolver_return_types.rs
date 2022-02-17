use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::enum_definition::EnumValue;

#[derive(Debug, PartialEq)]
pub enum Value<R = ()> {
    Null,
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    String(String),
    EnumValue(EnumValue),
    Vec(Vec<Value<R>>),
    Object(HashMap<Box<str>, Value<R>>),
    Custom(Box<R>),
}