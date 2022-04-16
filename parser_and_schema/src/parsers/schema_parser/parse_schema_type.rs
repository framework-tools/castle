use std::{collections::HashMap, io::Read};



use shared::castle_error::CastleError;

use crate::{token::{Token, token::{TokenKind, Identifier, Punctuator}}, tokenizer::{tokenizer::Tokenizer, tokenizer_utils::{get_next_token_and_unwrap}}, ast::syntax_definitions::{field_definition::FieldDefinition}};

use super::{parse_schema_field::parse_schema_field};


/// takes in tokenizer and returns parsed type
///    - start loop
///    - if next token is identifier, parse identifier
///    - call next token to skip openblock
///    - if next token is identifier, parse field
///    - else if next token is closeblock, break loop
///    - return parsed type
pub fn parse_schema_type<R>(tokenizer: &mut Tokenizer<R>) -> Result<FieldDefinition, CastleError> 
where R: Read{
    let mut fields = HashMap::new();
    let token = tokenizer.next(true)?;
    let identifier = get_identifier_skip_open_block(token, tokenizer)?;
    loop {
        let end_of_schema_type = check_token_and_parse_schema_field_or_break(tokenizer, &mut fields)?;
        if end_of_schema_type { break; }
    }
    return Ok(FieldDefinition { name: identifier, args: todo!(), return_type: todo!(), directives: todo!() });
}

pub fn get_identifier_skip_open_block<R>(token: Option<Token>, tokenizer: &mut Tokenizer<R>) -> Result<Box<str>, CastleError> 
where R: Read{
    let identifier = match token {
        Some(token) => match token.kind {
            TokenKind::Identifier(identifier) => identifier,
            _ => return Err(CastleError::Schema(format!("1. Expected identifier, found: {:?}.", token.kind).into(), token.span))
        },
        None => return Err(CastleError::AbruptEOF("Error found in 'get_identifier_skip_open_block'".into()))
    };
    tokenizer.next(true)?; // skip openblock
    return Ok(identifier.name)
}

pub fn check_token_and_parse_schema_field_or_break<R>(tokenizer: &mut Tokenizer<R>, fields: &mut HashMap<Box<str>, FieldDefinition>) -> Result<bool, CastleError> 
where R: Read {
    let token = get_next_token_and_unwrap(tokenizer)?; // get field identifier or closeblock
    return match &token.kind {
        TokenKind::Identifier(Identifier { name , .. }) => {
            insert_field_in_schema_type(name.clone(), tokenizer, fields, token)?;
            Ok(false) //should not break loop
        },
        TokenKind::Punctuator(Punctuator::CloseBlock) => Ok(true), //should break loop
        TokenKind::Punctuator(Punctuator::Comma) => Ok(false), //should not break loop
        _ => Err(CastleError::Schema(format!("3. Unexpected token: {:?}", token.kind).into(), token.span))
    }
}

fn insert_field_in_schema_type<R>(
    name: Box<str>, 
    tokenizer: &mut Tokenizer<R>, 
    fields: &mut HashMap<Box<str>, FieldDefinition>,
    token: Token
) -> Result<(), CastleError> 
where R: Read {

    let field = parse_schema_field(tokenizer, token)?;
    fields.insert(name, field);
    return Ok(())
}