
use std::collections::HashMap;

use shared::CastleError;

use crate::{token::{Token, token::{TokenKind, Identifier}}, parser::schema_parser::types::schema_field::Type};

use super::{expressions::PrimitiveValue, keyword::Keyword};

#[derive(Debug, PartialEq)]
pub enum Want {
    SingleField(SingleField),
    Projection(ObjectProjection)
}

#[derive(Debug, PartialEq)]
pub struct SingleField {
    pub identifier: Box<str>,
    pub arguments: Option<Vec<Argument>>
}

#[derive(Debug, PartialEq)]
pub enum Argument {
    Type(Type),
    Identifier(Box<str>),
    PrimitiveValue(PrimitiveValue)
}

impl Argument {
    pub fn new(token: Token) -> Result<Self, CastleError> {
        match token.kind {
            TokenKind::PrimitiveType(primitive_type) => return Ok(Argument::Type(Type::PrimitiveType(primitive_type))),
            TokenKind::VecType(vec_type) => return Ok(Argument::Type(Type::VecType(vec_type))),
            TokenKind::Identifier(Identifier { name, ..}) => {
                let first_char = name.chars().nth(0);
                match first_char {
                    Some(first_char) => {
                        if first_char.is_uppercase() { return Ok(Argument::Type(Type::SchemaTypeOrEnum(name))) } 
                        else { return Ok(Argument::Identifier(name)) }
                    },
                    None => Err(CastleError::Unimplemented("argument cannot be empty".into()))
                }
            },
            _ => {
                let primitive_value = PrimitiveValue::new_from_token_kind(token.kind);
                match primitive_value {
                    Some(primitive_value) => return Ok(Argument::PrimitiveValue(primitive_value)),
                    None => Err(CastleError::Unimplemented("argument cannot be empty".into()))
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ObjectProjection {
    pub identifier: Box<str>,
    pub fields: Option<HashMap<Box<str>, Want>>,
    pub match_statements: Option<HashMap<Box<str>, Want>>
}

impl Want {
    pub fn new_single_field(identifier: Box<str>, arguments: Option<Vec<Argument>>) -> Self {
        Want::SingleField(SingleField {
            identifier,
            arguments
        })
    }

    pub fn new_projection(identifier: Box<str>, fields: Option<HashMap<Box<str>, Want>>, match_statements: Option<HashMap<Box<str>, Want>>) -> Self {
        Want::Projection(ObjectProjection {
            identifier,
            fields,
            match_statements
        })
    }

    pub fn get_identifier(&self) -> Box<str> {
        return match self {
            Want::SingleField(single_field) => single_field.identifier.clone(),
            Want::Projection(projection) => projection.identifier.clone()
        }
    }
}

impl SingleField {
    pub fn new(identifier: Box<str>, arguments: Option<Vec<Argument>>) -> Want {
        Want::SingleField(SingleField {
            identifier,
            arguments
        })
    }
}

impl ObjectProjection {
    pub fn new(identifier: Box<str>, fields: Option<HashMap<Box<str>, Want>>, match_statements: Option<HashMap<Box<str>, Want>>) -> Want {
        Want::Projection(ObjectProjection {
            identifier,
            fields,
            match_statements
        })
    }
}