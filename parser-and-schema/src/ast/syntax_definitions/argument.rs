use std::io::Read;

use input_cursor::Cursor;
use shared::CastleError;

use crate::{token::{Token, token::{TokenKind, Identifier, Punctuator}}, parser::schema_parser::types::type_system::{Type, parse_type}, tokenizer::tokenizer::{Tokenizer, advance_and_parse_token}};

use super::expressions::PrimitiveValue;


#[derive(Debug, PartialEq)]
pub enum Argument {
    Type(Type),
    Identifier(Box<str>),
    PrimitiveValue(PrimitiveValue),
    IdentifierAndType(Box<str>, Type)
}

impl Argument {
    pub fn new<R>(token: Token, tokenizer: &mut Tokenizer<R>) -> Result<Self, CastleError> 
    where R: Read {
        let argument = match token.kind {
            TokenKind::PrimitiveType(primitive_type) => Argument::Type(Type::PrimitiveType(primitive_type)),
            TokenKind::VecType(vec_type) => Argument::Type(Type::VecType(vec_type)),
            TokenKind::Identifier(Identifier { name, ..}) => parse_identifier_argument(name, tokenizer)?, //can be ident, type, enum or a combo
            _ => parse_primitive_value_argument(token.kind)?
        };
        return Ok(argument)
    }
}

fn parse_identifier_argument<R>(name: Box<str>, tokenizer: &mut Tokenizer<R>) -> Result<Argument, CastleError>
where R: Read {
    let first_char = name.chars().nth(0);
    match first_char {
        Some(first_char) => {
            if first_char.is_uppercase() { return Ok(Argument::Type(Type::SchemaTypeOrEnum(name))) } //Enum or Type Argument
            else { 
                let token = tokenizer.next(true)?;
                match token {
                    Some(token) => match token.kind {
                        TokenKind::Punctuator(Punctuator::Colon) => { //Identifier and Type Argument
                            let type_ = parse_type(tokenizer)?;
                            return Ok(Argument::IdentifierAndType(name, type_));
                        },
                        _ => return Ok(Argument::Identifier(name)) //Identifier Argument
                    },
                    None => return Err(CastleError::AbruptEOF)
                }
            }
        },
        None => Err(CastleError::Unimplemented("argument cannot be empty 1".into()))
    }
}

fn parse_primitive_value_argument(token_kind: TokenKind) -> Result<Argument, CastleError> {
    let primitive_value = PrimitiveValue::new_from_token_kind(token_kind);
    match primitive_value {
        Some(primitive_value) => return Ok(Argument::PrimitiveValue(primitive_value)),
        None => Err(CastleError::Unimplemented("argument cannot be empty 2".into()))
    }
}