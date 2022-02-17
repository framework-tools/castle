use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Value<R = ()> {
    Null,
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    String(String),
    EnumValue(EnumResolverValue<R>),
    Vec(Vec<Value<R>>),
    Object(HashMap<Box<str>, Value<R>>),
    Custom(Box<R>),
}

#[derive(Debug, PartialEq)]
pub struct EnumResolverValue<R> {
    pub identifier: Box<str>,
    pub enum_parent: Box<str>,
    pub variant: Box<str>,
    pub fields: HashMap<Box<str>, Value<R>>,
}