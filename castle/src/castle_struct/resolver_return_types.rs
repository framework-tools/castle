use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::enum_definition::EnumValue;

#[derive(Debug, PartialEq)]
pub enum ReturnValue<R = ()> {
    Null,
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    String(String),
    EnumValue(EnumValue),
    Vec(Vec<ReturnValue<R>>),
    Object(HashMap<String, ReturnValue<R>>),
    Custom(Box<R>),
}