use std::{collections::HashMap, io::Read};

use shared::CastleError;

use crate::{tokenizer::{tokenizer::Tokenizer}, ast::syntax_definitions::keyword::Keyword, token::token::{TokenKind, Punctuator, Identifier}};

use super::types::{schema_field::{Type, SchemaField}, schema_type::SchemaType};

/// takes in schema as string and returns parsed schema as hashmap
///     - creates tokenizer
///     - loops through tokens
///     - if token is type keyword, parse type
///     - insert parsed type into hashmap
///     - if token is none, break loop
///     - return parsed schema
pub fn parse_schema(schema: &str) -> Result<HashMap<Box<str>, SchemaType>, CastleError> {
    let bytes = schema.as_bytes();
    let mut tokenizer = Tokenizer::new(bytes);
    let mut parsed_schema: HashMap<Box<str>, SchemaType> = HashMap::new();
    let mut err: Option<Result<HashMap<Box<str>, SchemaType>, CastleError>> = None;
    loop {
        let token = tokenizer.next(true)?;
        match token {
            Some(token) => match token.kind {
                TokenKind::Keyword(Keyword::Type) => {
                    let schema_type = parse_schema_type(&mut tokenizer)?;
                    parsed_schema.insert(schema_type.identifier.clone(), schema_type);
                },
                _ => {
                    err = Some(Err(CastleError::Schema(format!("Unexpected token: {:?}", token.kind).into(), token.span))); 
                }
            }
            None => break
        }
    }
    err = check_for_undefined_schema_types(&parsed_schema);
    if err.is_some() { return err.unwrap() }
    else { return Ok(parsed_schema) }
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
            _ => return Err(CastleError::Schema(format!("Expected identifier, found: {:?}. 1", token.kind).into(), token.span))
        },
        None => return Err(CastleError::AbruptEOF)
    };
    tokenizer.next(true)?; // skip openblock

    loop {
        let token = tokenizer.peek(true)?; // get field identifier or closeblock
        match token {
            Some(token) => match &token.kind {
                    TokenKind::Identifier(Identifier { name , .. }) => {
                        let name = name.clone();
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
    return Ok(SchemaType {
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
            _ => return Err(CastleError::Schema(format!("Expected identifier, found: {:?}. 2", token.kind).into(), token.span))
        },
        None => return Err(CastleError::AbruptEOF)
    };
    tokenizer.next(true)?; // skip colon
    let type_ = parse_type(tokenizer)?;
    return Ok(SchemaField {
        name: identifier.name,
        type_
    });
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
    return match token {
        Some(token) => match token.kind {
            TokenKind::PrimitiveType(primitive_type) => {
                let option_peeked_token = tokenizer.peek(true)?;
                let peeked_token = match option_peeked_token {
                    Some(token) => token,
                    None => return Err(CastleError::AbruptEOF)
                };
                if peeked_token.kind == TokenKind::Punctuator(Punctuator::Comma) {
                    tokenizer.next(true)?; // skip comma
                }
                return Ok(Type::PrimitiveType(primitive_type))
            },
            TokenKind::Identifier(identifier) => {
                let option_peeked_token = tokenizer.peek(true)?;
                let peeked_token = match option_peeked_token {
                    Some(token) => token,
                    None => return Err(CastleError::AbruptEOF)
                };
                if peeked_token.kind == TokenKind::Punctuator(Punctuator::Comma) {
                    tokenizer.next(true)?; // skip comma
                }
                return Ok(Type::SchemaType(identifier.name))
            },
            _ => Err(CastleError::Schema(format!("Expected type, found: {:?}", token.kind).into(), token.span))
        },
        None => Err(CastleError::AbruptEOF)
    }
}

/// Takes in parsed schema and checks each field for any undefined types
///     - For each Type loop
///     - For each Field in Type Loop
///     - If Field Type is a SchemaType, check this type is defined in the schema (Hashmap)
///     - If FieldType is not defined, return Some(error)
///     - Else if no errors found, return None
fn check_for_undefined_schema_types(schema: &HashMap<Box<str>, SchemaType>) -> Option<Result<HashMap<Box<str>, SchemaType>, CastleError>> {
    let mut err = None;
    for (_schema_type_name, schema_type) in schema {
        for (_field_name, field) in &schema_type.fields {
            match &field.type_ {
                Type::SchemaType(schema_type_name) => {
                    if !schema.contains_key(schema_type_name) {
                        err = Some(Err(CastleError::UndefinedSchemaType(format!("Undefined schema type: {}", schema_type_name).into())));
                        break;
                    }
                },
                _ => {}
            }
        }
    }
    return err;
}