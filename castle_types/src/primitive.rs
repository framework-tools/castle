use serde::{Serialize, Deserialize};

use crate::Number;


#[derive(Debug, PartialEq, Clone)]
pub enum Primitive {
    String(Box<str>),
    Number(Number),
    Boolean(bool),
}

impl std::fmt::Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::String(s) => write!(f, "\"{}\"", s),
            Primitive::Number(n) => write!(f, "{}", n),
            Primitive::Boolean(b) => write!(f, "{}", b),
        }
    }
}

