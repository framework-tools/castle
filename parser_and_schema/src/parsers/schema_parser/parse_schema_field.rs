use std::io::Read;



use shared::castle_error::CastleError;

use crate::{tokenizer::{tokenizer::Tokenizer, tokenizer_utils::{peek_next_token_and_unwrap, get_next_token_and_unwrap}, parse_identifier::parse_arguments}, token::{token::{TokenKind, Punctuator}, Token}, ast::syntax_definitions::field_definition::FieldDefinition};

use super::types::{parse_type::parse_type, parse_directive::parse_directives};


/// takes in tokenizer and returns parsed field
///   - get next token
///   - if next token is identifier, parse identifier
///   - skip next token to skip colon
///   - get next token and parse into type
///   - return parsed field

pub fn parse_schema_field<R>(tokenizer: &mut Tokenizer<R>, token: Token) -> Result<FieldDefinition, CastleError> 
where R: Read{
    Ok(FieldDefinition {
        name: tokenizer.expect_identifier(true)?,
        args: parse_arguments_and_colon(tokenizer),
        return_type: parse_type(tokenizer)?, // get fields type
        directives: parse_directives(tokenizer)?;
    })
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

