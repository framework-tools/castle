use std::{collections::HashMap, io::Read};

use shared::CastleError;

use crate::{tokenizer::{tokenizer::Tokenizer}, ast::syntax_definitions::keyword::Keyword, token::token::{TokenKind, Punctuator, Identifier}};

use super::types::{schema_type::{Type}, schema_field::{SchemaField, SchemaType}};
/// takes in schema as string and returns parsed schema as hashmap
///     - creates tokenizer
///     - loops through tokens
///     - if token is type keyword, parse type
///     - insert parsed type into hashmap
///     - if token is none, break loop
///     - return parsed schema
pub fn parse_schema(schema: &str) -> Result<HashMap<Box<str>, Type>, CastleError> {
    let bytes = schema.as_bytes();
    let mut tokenizer = Tokenizer::new(bytes);
    let parsed_schema: HashMap<Box<str>, Type> = HashMap::new();
    let mut err: Option<Result<HashMap<Box<str>, Type>, CastleError>> = None;
    loop {
        let token = tokenizer.next(true)?;
        match token {
            Some(token) => match token.kind {
                TokenKind::Keyword(Keyword::Type) => {
                    let schema_type = parse_schema_type(&mut tokenizer)?;
                    parsed_schema.insert(schema_type.identifier, schema_type);
                },
                _ => {
                    err = Some(Err(CastleError::Schema(format!("Unexpected token: {:?}", token.kind).into(), token.span))); 
                }
            }
            None => break
        }
    }
    return Ok(parsed_schema)
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
    let identifier = match token {
        Some(token) => match token.kind {
            TokenKind::Identifier(identifier) => identifier,
            _ => return Err(CastleError::Schema(format!("Expected identifier, found: {:?}", token.kind).into(), token.span))
        },
        None => return Err(CastleError::AbruptEOF)
    };
    tokenizer.next(true)?; // skip openblock

    loop {
        match token {
            Some(token) => match token.kind {
                    TokenKind::Identifier(Identifier { name , .. }) => {
                        let field = parse_schema_field(tokenizer)?;
                        fields.insert(name, field);
                    },
                    TokenKind::Punctuator(Punctuator::CloseBlock) => {
                        break;
                    },
                    _ => {
                        return Err(CastleError::Schema(format!("Unexpected token: {:?}", token.kind).into(), token.span));
                    }
            },
            None => break
        }
    }
    return Ok(Type {
        identifier: identifier.name,
        fields
    });
}

/// takes in tokenizer and returns parsed field
///   - get next token
///   - if next token is identifier, parse identifier
///   - skip next token to skip colon
///   - get next token and parse into type
///   - return parsed field

fn parse_schema_field<R>(tokenizer: &mut Tokenizer<R>) -> Result<SchemaField, CastleError> 
where R: Read{
    let token = tokenizer.next(true)?;
    let identifier = match token {
        Some(token) => match token.kind {
            TokenKind::Identifier(identifier) => identifier,
            _ => return Err(CastleError::Schema(format!("Expected identifier, found: {:?}", token.kind).into(), token.span))
        },
        None => return Err(CastleError::AbruptEOF)
    };
    tokenizer.next(true)?; // skip colon
    let schema_type = parse_schema_type(tokenizer)?;
    return Ok(SchemaField {
        name: identifier.name,
        schema_type: SchemaType::PrimitiveType()
    });
}
/// takes in tokenizer and returns parsed type
///  - get next token
///  - match token kind to a Type
/// - return parsed type
///  
fn parse_type<R>(tokenizer: &mut Tokenizer<R>) -> Result<Type, CastleError> 
where R: Read{
    let token = tokenizer.next(true)?;
    match token {
        Some(token) => match token.kind {
            TokenKind::Identifier(identifier) => Ok(identifier.name),
            _ => Err(CastleError::Schema(format!("Expected identifier, found: {:?}", token.kind).into(), token.span))
        },
        None => Err(CastleError::AbruptEOF)
    }
}