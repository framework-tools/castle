use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub enum Expression {
    PrimitiveValue(PrimitiveValue),
}


#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum PrimitiveValue {
    String(Box<str>),
    Float(f64),
    Int(i64),
    UInt(u64),
    Boolean(bool),
}
