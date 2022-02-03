use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub enum Expression {
    PrimitiveValue(PrimitiveValue),
    Projection(ProjectionExpression), // obj::{ ... }
}


#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Hash, Eq)]
pub enum PrimitiveValue {
    String(Box<str>),
    Float(F64),
    Int(i64),
    UInt(u64),
    Boolean(bool),
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Hash, Eq)]
pub struct F64 {
    pub integer_part: i64,
    pub decimal_part: i64,
}

impl F64 {
    pub fn new(f: f64) -> Self {
        let integer_part = f.floor() as i64;
        let decimal_part = ((f - integer_part as f64) * 10_f64.powi(10)) as i64;

        return Self {
            integer_part,
            decimal_part,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ProjectionExpression {
    pub identifier: Box<str>,
    pub fields: Projection,
}

#[derive(Debug, PartialEq)]
pub enum Projection {
    Object(FieldProjection),
    Array(FieldProjection),
}

pub type FieldProjection = HashMap<String, FieldStatement>;

#[derive(Debug, PartialEq)]
pub struct FieldStatement {
    pub name: Box<str>,
    pub value: Option<Expression>,
    pub sub_projection: Option<Projection>,
}
