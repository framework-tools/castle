use std::io::Read;

use shared::CastleError;

use crate::{tokenizer::tokenizer::Tokenizer, token::{token::{TokenKind, Punctuator, VecType}, Token}};

use super::types::schema_field::{SchemaField, Type, PrimitiveType};


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
/// takes in tokenizer and returns parsed type
///  - get next token
///  - match token kind to a Type
///  - else if token kind identifier parse identifier as schematype
///  - return parsed type
///  
fn parse_type<R>(tokenizer: &mut Tokenizer<R>) -> Result<Type, CastleError> 
where R: Read{
    let token = tokenizer.next(true)?;
    match token {
        Some(token) => match token.kind {
            TokenKind::PrimitiveType(primitive_type) => return get_primitive_type(primitive_type, tokenizer),
            TokenKind::Identifier(identifier) => return get_schema_type(identifier.name, tokenizer),
            TokenKind::VecType(type_) => return get_vec_type(type_, tokenizer),
            // Need to implement with types from AST
            _ => Err(CastleError::Schema(format!("Expected type, found: {:?}", token.kind).into(), token.span))
        },
        None => Err(CastleError::AbruptEOF)
    }
}

pub fn get_identifier<R>(token: Option<Token>, tokenizer: &mut Tokenizer<R>) -> Result<Box<str>, CastleError> 
where R: Read{
    match token {
        Some(token) => match token.kind {
            TokenKind::Identifier(identifier) => return Ok(identifier.name),
            _ => return Err(CastleError::Schema(format!("2. Expected identifier, found: {:?}", token.kind).into(), token.span))
        },
        None => return Err(CastleError::AbruptEOF)
    }
}

fn get_primitive_type<R>(primitive_type: PrimitiveType, tokenizer: &mut Tokenizer<R>) -> Result<Type, CastleError> 
where R: Read{
    skip_comma(tokenizer)?;
    return Ok(Type::PrimitiveType(primitive_type))
}

fn get_schema_type<R>(identifier: Box<str>, tokenizer: &mut Tokenizer<R>) -> Result<Type, CastleError> 
where R: Read{
    skip_comma(tokenizer)?;
    return Ok(Type::SchemaTypeOrEnum(identifier))
}

fn get_vec_type<R>(vec_type: VecType, tokenizer: &mut Tokenizer<R>) -> Result<Type, CastleError>
    where R: Read{
    skip_comma(tokenizer)?;
    return Ok(Type::VecType(vec_type))
}

fn skip_comma<R>(tokenizer: &mut Tokenizer<R>) -> Result<(), CastleError> 
where R: Read{
    let option_peeked_token = tokenizer.peek(true)?;
    let peeked_token = match option_peeked_token {
        Some(token) => token,
        None => return Err(CastleError::AbruptEOF)
    };
    if peeked_token.kind == TokenKind::Punctuator(Punctuator::Comma) {
        tokenizer.next(true)?; // skip comma
    }
    return Ok(());
}