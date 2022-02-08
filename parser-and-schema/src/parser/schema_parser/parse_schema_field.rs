use std::io::Read;

use shared::CastleError;

use crate::{tokenizer::tokenizer::Tokenizer, token::{token::{TokenKind, Punctuator}, Token}};

use super::types::{schema_field::{SchemaField}, type_system::{parse_type}};


/// takes in tokenizer and returns parsed field
///   - get next token
///   - if next token is identifier, parse identifier
///   - skip next token to skip colon
///   - get next token and parse into type
///   - return parsed field

pub fn parse_schema_field<R>(tokenizer: &mut Tokenizer<R>) -> Result<SchemaField, CastleError> 
where R: Read{
    let token = tokenizer.next(true)?; // should be identifier
    let identifier = get_identifier(token, tokenizer)?;
    tokenizer.next(true)?; // skip colon
    let type_ = parse_type(tokenizer)?; // get fields type
    return Ok(SchemaField { name: identifier, type_, directives: None });
}

pub fn get_identifier<R>(token: Option<Token>, tokenizer: &mut Tokenizer<R>) -> Result<Box<str>, CastleError> 
where R: Read{
    println!("1. token: {:#?}", token);
    match token {
        Some(token) => match token.kind {
            TokenKind::Identifier(identifier) => return Ok(identifier.name),
            _ => return Err(CastleError::Schema(format!("2. Expected identifier, found: {:?}", token.kind).into(), token.span))
        },
        None => return Err(CastleError::AbruptEOF("Error found in 'get_identifier'".into()))
    }
}

pub fn skip_comma<R>(tokenizer: &mut Tokenizer<R>) -> Result<(), CastleError> 
where R: Read{
    let option_peeked_token = tokenizer.peek(true)?;
    println!("2. token: {:#?}", option_peeked_token);
    let peeked_token = match option_peeked_token {
        Some(token) => token,
        None => return Err(CastleError::AbruptEOF("Error found in 'skip_comma'".into()))
    };
    if peeked_token.kind == TokenKind::Punctuator(Punctuator::Comma) {
        tokenizer.next(true)?; // skip comma
    }
    return Ok(());
}