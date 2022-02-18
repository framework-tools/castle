use std::io::Read;



use shared::castle_error::CastleError;

use crate::{tokenizer::{tokenizer::Tokenizer, tokenizer_utils::{peek_next_token_and_unwrap, get_next_token_and_unwrap}}, token::{token::{TokenKind, Punctuator}, Token}};

use super::types::{schema_field::{SchemaField}, type_system::parse_type, parse_directive::parse_directives};


/// takes in tokenizer and returns parsed field
///   - get next token
///   - if next token is identifier, parse identifier
///   - skip next token to skip colon
///   - get next token and parse into type
///   - return parsed field

pub fn parse_schema_field<R>(tokenizer: &mut Tokenizer<R>, token: Token) -> Result<SchemaField, CastleError> 
where R: Read{
    let identifier = get_identifier(token)?;
    tokenizer.next(true)?; // skip colon
    let token = get_next_token_and_unwrap(tokenizer)?;
    let type_ = parse_type(token)?; // get fields type
    let directives = parse_directives(tokenizer)?;
    return Ok(SchemaField { name: identifier, type_, directives });
}

pub fn get_identifier(token: Token) -> Result<Box<str>, CastleError> {
    match token.kind {
        TokenKind::Identifier(identifier) => return Ok(identifier.name),
        _ => return Err(CastleError::Schema(format!("2. Expected identifier, found: {:?}", token.kind).into(), token.span))
    }
}

pub fn skip_comma<R>(tokenizer: &mut Tokenizer<R>) -> Result<(), CastleError> 
where R: Read{
    let peeked_token = peek_next_token_and_unwrap(tokenizer)?;
    if peeked_token.kind == TokenKind::Punctuator(Punctuator::Comma) {
        tokenizer.next(true)?; // skip comma
    }
    return Ok(());
}