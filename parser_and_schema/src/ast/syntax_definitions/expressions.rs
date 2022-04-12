

use serde::{Deserialize, Serialize};
use shared::castle_error::CastleError;



use crate::token::token::Identifier;

use super::{enum_definition::EnumValue};

#[derive(Debug, PartialEq)]
pub enum Expression {
    PrimitiveValue(PrimitiveValue),
    EnumValue(EnumValue),
    Identifier(Identifier),
}

impl Expression {
    pub fn get_identifier(&self) -> Box<str> {
        match self {
            Expression::PrimitiveValue(primitive_value) => match primitive_value {
                PrimitiveValue::String(string) => string.clone().into(),
                PrimitiveValue::Boolean(value) => value.clone().to_string().into(),
                PrimitiveValue::Float(value) => value.clone().to_string().into(),
                PrimitiveValue::Int(value) => value.clone().to_string().into(),
                PrimitiveValue::UInt(value) => value.clone().to_string().into(),
            },
            Expression::EnumValue(enum_value) => enum_value.identifier.clone(),
            Expression::Identifier(identifier) => identifier.name.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum PrimitiveValue {
    String(Box<str>),
    Float(f64),
    Int(i64),
    UInt(u64),
    Boolean(bool),
}

impl PrimitiveValue {
    pub fn new_from_str(value: String) -> Result<PrimitiveValue, CastleError> {
        if value.contains('"') {
            return Ok(PrimitiveValue::String(value.into()))
        }
        else if value == "true" {
            return Ok(PrimitiveValue::Boolean(true))
        }
        else if value == "false" {
            return Ok(PrimitiveValue::Boolean(false))
        }
        else if value.contains('.') {
            return Ok(PrimitiveValue::Float(value.parse().unwrap()))
        }
        else if value.contains('-'){
            return Ok(PrimitiveValue::Int(value.parse().unwrap()))
        }
        else {
            return Ok(PrimitiveValue::UInt(value.parse().unwrap()))
        }
    }
}
