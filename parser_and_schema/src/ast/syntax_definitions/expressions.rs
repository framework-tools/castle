

use serde::{Deserialize, Serialize};
use shared::CastleError;

use crate::token::token::{TokenKind, Numeric, Identifier};

use super::{keyword::Keyword, enum_definition::EnumValue};

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
    
    pub fn new_from_token_kind(token_kind: TokenKind) -> Option<Self> {
        match token_kind {
            TokenKind::StringLiteral(s) => Some(PrimitiveValue::String(s)),
            TokenKind::NumericLiteral(numeric) => match_numeric_token_to_primitive(numeric),
            TokenKind::BooleanLiteral(b) => Some(PrimitiveValue::Boolean(b)),
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::True => Some(PrimitiveValue::Boolean(true)),
                Keyword::False => Some(PrimitiveValue::Boolean(false)),
                _ => None,
            },
            _ => None
        }
    }

    pub fn check_if_primitive_value(token_kind: &TokenKind) -> bool {
        match token_kind {
            TokenKind::StringLiteral(_s) => true,
            TokenKind::NumericLiteral(_numeric) => true,
            TokenKind::BooleanLiteral(_b) => true,
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::True => true,
                Keyword::False => true,
                _ => false,
            },
            _ => false
        }
    }
}
fn match_numeric_token_to_primitive(numeric:Numeric) -> Option<PrimitiveValue> {
    match numeric {
        Numeric::Float(f) => Some(PrimitiveValue::Float(f)),
        Numeric::Integer(i) => Some(PrimitiveValue::Int(i)),
        Numeric::UnsignedInteger(u) => Some(PrimitiveValue::UInt(u)),
    }
}