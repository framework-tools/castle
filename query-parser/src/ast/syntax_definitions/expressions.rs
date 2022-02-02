use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub enum Expression {
    PrimitiveValue(PrimitiveValue),
    Projection(ProjectionExpression), // obj::{ ... }
}


#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum PrimitiveValue {
    String(Box<str>),
    Float(f64),
    Int(i64),
    UInt(u64),
    Boolean(bool),
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
