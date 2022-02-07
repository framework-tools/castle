

use serde::{Deserialize, Serialize};

use crate::token::token::{TokenKind, Numeric};

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

impl PrimitiveValue {
    pub fn new_from_token_kind(token_kind: TokenKind) -> Option<Self> {
        match token_kind {
            TokenKind::StringLiteral(s) => Some(PrimitiveValue::String(s)),
            TokenKind::NumericLiteral(numeric) => {
                match numeric {
                    Numeric::Float(f) => Some(PrimitiveValue::Float(f)),
                    Numeric::Integer(i) => Some(PrimitiveValue::Int(i)),
                    Numeric::UnsignedInteger(u) => Some(PrimitiveValue::UInt(u)),
                }
            },
            TokenKind::BooleanLiteral(b) => Some(PrimitiveValue::Boolean(b)),
            _ => None
        }
    }
}
