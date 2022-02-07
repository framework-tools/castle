use std::{collections::HashMap, io::Read};

use shared::CastleError;

use crate::{token::{Token, token::{TokenKind, Identifier, Punctuator}}, tokenizer::tokenizer::Tokenizer, ast::syntax_definitions::{keyword::Keyword, schema_definition::SchemaDefinition}};

use super::{types::{schema_type::SchemaType, schema_field::SchemaField}, parse_schema_field::parse_schema_field};


pub fn check_token_and_parse_schema_or_break<R>(
    token: Option<Token>, 
    tokenizer: &mut Tokenizer<R>,
    parsed_schema: &mut SchemaDefinition
) -> Result<bool, CastleError> where R: Read {
    match token {
        Some(token) => match token.kind {
            TokenKind::Keyword(Keyword::Type) => {
                let schema_type = parse_schema_type(tokenizer)?;
                parsed_schema.schema_types.insert(schema_type.identifier.clone(), schema_type);
                return Ok(false)
            },
            TokenKind::Keyword(Keyword::Enum) => {
                let enum_definition = parse_enum_definition(tokenizer)?;
                parsed_schema.enums.insert(enum_definition.name.clone(), enum_definition);
                return Ok(false)
            },  
            _ => return Err(CastleError::Schema(format!("1. Unexpected token: {:?}", token.kind).into(), token.span))
        },
        None => return Ok(true)
    }
}

/// takes in tokenizer and returns parsed type
///    - start loop
///    - if next token is identifier, parse identifier
///    - call next token to skip openblock
///    - if next token is identifier, parse field
///    - else if next token is closeblock, break loop
///    - return parsed type
fn parse_schema_type<R>(tokenizer: &mut Tokenizer<R>) -> Result<SchemaType, CastleError> 
where R: Read{
    let mut fields = HashMap::new();
    let token = tokenizer.next(true)?;
    let identifier = get_identifier_skip_open_block(token, tokenizer)?;

    loop {
        let end_of_schema_type = check_token_and_parse_schema_field_or_break(tokenizer, &mut fields)?;
        if end_of_schema_type { break; }
    }
    return Ok(SchemaType { identifier, fields });
}

fn get_identifier_skip_open_block<R>(token: Option<Token>, tokenizer: &mut Tokenizer<R>) -> Result<Box<str>, CastleError> 
where R: Read{
    let identifier = match token {
        Some(token) => match token.kind {
            TokenKind::Identifier(identifier) => identifier,
            _ => return Err(CastleError::Schema(format!("1. Expected identifier, found: {:?}.", token.kind).into(), token.span))
        },
        None => return Err(CastleError::AbruptEOF)
    };
    tokenizer.next(true)?; // skip openblock
    return Ok(identifier.name)
}

fn check_token_and_parse_schema_field_or_break<R>(tokenizer: &mut Tokenizer<R>, fields: &mut HashMap<Box<str>, SchemaField>) -> Result<bool, CastleError> 
where R: Read {
    let token = tokenizer.peek(true)?; // get field identifier or closeblock
    match token {
        Some(token) => match &token.kind {
            TokenKind::Identifier(Identifier { name , .. }) => {
                insert_field_in_schema_type(name.clone(), tokenizer, fields)?;
                return Ok(false) //should not break loop
            },
            TokenKind::Punctuator(Punctuator::CloseBlock) => {
                tokenizer.next(true)?; // skip closeblock
                return Ok(true) //should break loop
            },
            _ => return Err(CastleError::Schema(format!("2. Unexpected token: {:?}", token.kind).into(), token.span))
        },
        None => return Err(CastleError::AbruptEOF)
    }
}

fn insert_field_in_schema_type<R>(name: Box<str>, tokenizer: &mut Tokenizer<R>, fields: &mut HashMap<Box<str>, SchemaField>) -> Result<(), CastleError> 
where R: Read {
    let field = parse_schema_field(tokenizer)?;
    fields.insert(name, field);
    return Ok(())
}