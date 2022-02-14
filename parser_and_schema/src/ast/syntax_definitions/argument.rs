use std::io::Read;

use shared::CastleError;

use crate::{token::{Token, token::{TokenKind, Identifier, Punctuator}}, parsers::schema_parser::types::type_system::{Type, parse_type}, tokenizer::tokenizer::{Tokenizer}};

use super::expressions::PrimitiveValue;

//For Schema Resolvers/Functions
pub type IdentifierAndTypeArgument = (Box<str>, Type);
//For Query Object Projections/Resolvers
pub type IdentifierAndValueArgument = (Box<str>, PrimitiveValue);

#[derive(Debug, PartialEq)]
pub enum ArgumentOrTuple {
    PrimitiveValue(PrimitiveValue),
    IdentifierAndType(IdentifierAndTypeArgument),
    IdentifierAndValue(IdentifierAndValueArgument),
}

impl ArgumentOrTuple {
    pub fn new<R>(token: Token, tokenizer: &mut Tokenizer<R>) -> Result<Self, CastleError> 
    where R: Read {
        let argument = match token.kind {
            TokenKind::Identifier(Identifier { name, ..}) => parse_identifier_argument(name, tokenizer)?, //can be ident, type, enum or a combo
            //parse option argument
            _ => parse_primitive_value_argument(token.kind)?
        };
        return Ok(argument)
    }
}

fn parse_identifier_argument<R>(name: Box<str>, tokenizer: &mut Tokenizer<R>) -> Result<ArgumentOrTuple, CastleError>
where R: Read {
    let first_char = name.chars().nth(0);
    let token = tokenizer.next(true)?;
    match token {
        Some(token) => match_token_to_parse_argument(token, tokenizer, name),
        None => return Err(CastleError::AbruptEOF("Error found in 'parse_identifier_argument'".into()))
    }
}

fn match_token_to_parse_argument<R>(token: Token, tokenizer:&mut Tokenizer<R>, name: Box<str>) -> Result<ArgumentOrTuple, CastleError> 
where R: Read {
    match token.kind {
        TokenKind::Punctuator(Punctuator::Colon) => { //Identifier and Type Argument
            let type_ = parse_type(tokenizer)?;
            let ident_and_type: IdentifierAndTypeArgument = (name, type_);
            return Ok(ArgumentOrTuple::IdentifierAndType(ident_and_type));
        },
        _ => return Err(CastleError::Schema(format!("Expected ':' after identifier '{}'", name).into(), token.span))
    }
}


fn parse_primitive_value_argument(token_kind: TokenKind) -> Result<ArgumentOrTuple, CastleError> {
    let primitive_value = PrimitiveValue::new_from_token_kind(token_kind);
    match primitive_value {
        Some(primitive_value) => return Ok(ArgumentOrTuple::PrimitiveValue(primitive_value)),
        None => Err(CastleError::Unimplemented("argument cannot be empty 2".into()))
    }
}