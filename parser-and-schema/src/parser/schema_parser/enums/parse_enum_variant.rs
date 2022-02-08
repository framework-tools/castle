use std::{collections::HashMap, io::Read};

use shared::CastleError;

use crate::{tokenizer::tokenizer::Tokenizer, ast::syntax_definitions::{enum_definition::{EnumVariant, EnumDataType}, argument::Argument}, token::token::{TokenKind, Identifier, Punctuator}, parser::schema_parser::{types::schema_field::SchemaField, parse_schema_type::check_token_and_parse_schema_field_or_break}};

use super::parse_enum::insert_variant_in_enum;

pub fn check_token_and_parse_enum_variant_or_break<R>(
    tokenizer: &mut Tokenizer<R>, 
    variants: &mut HashMap<Box<str>, EnumVariant>
) -> Result<bool, CastleError>  where R: Read {
    let token = tokenizer.peek(true)?; // get field identifier or closeblock
    match token {
        Some(token) => match &token.kind {
            TokenKind::Identifier(Identifier { name , .. }) => {
                insert_variant_in_enum(name.clone(), tokenizer, variants)?;
                return Ok(false) //should not break loop
            },
            TokenKind::Punctuator(Punctuator::CloseBlock) => {
                tokenizer.next(true)?; // skip closeblock
                return Ok(true) //should break loop
            },
            TokenKind::Punctuator(Punctuator::Comma) => {
                tokenizer.next(true)?; // skip closeblock
                return Ok(false) //should not break loop
            }, 
            _ => return Err(CastleError::Schema(format!("2. Unexpected token: {:?}", token.kind).into(), token.span))
        },
        None => return Err(CastleError::AbruptEOF("Error found in 'check_token_and_parse_enum_variant_or_break'".into()))
    }
}

pub fn parse_enum_variant<R>(tokenizer: &mut Tokenizer<R>) -> Result<EnumVariant, CastleError> 
where R: Read{
    let token = tokenizer.next(true)?; // should be identifier
    let identifier = match token {
        Some(token) => match token.kind {
            TokenKind::Identifier(identifier) => identifier,
            _ => return Err(CastleError::Schema(format!("4. Unexpected token: {:?}", token.kind).into(), token.span))
        },
        None => return Err(CastleError::AbruptEOF("Error found in 'parse_enum_variant'".into()))
    };
    let enum_data_type = parse_enum_data_type(identifier.arguments, tokenizer)?;
    return Ok(EnumVariant { name: identifier.name, enum_data_type, directives: HashMap::new() });
}

/// takes in tokenizer and returns enum data type
/// - if next token is identifier or comma, return enum unit
/// - else if next token is openparen, return enum tuple
/// finish
fn parse_enum_data_type<R>(arguments: Option<Vec<Argument>>, tokenizer: &mut Tokenizer<R>)
-> Result<EnumDataType, CastleError> where R: Read {
    match arguments {
        Some(arguments) => return Ok(EnumDataType::EnumTuple(arguments)), // tuple enum type
        None => {
            let enum_data_type = get_object_or_unit_type(tokenizer)?;
            return Ok(enum_data_type) //unit or object enum type
        }
    }
}

fn get_object_or_unit_type<R>(tokenizer: &mut Tokenizer<R>) -> Result<EnumDataType, CastleError>
where R: Read {
    let peeked_token = tokenizer.peek(true)?;
    match peeked_token {
        Some(token) => match token.kind {
            TokenKind::Punctuator(Punctuator::OpenBlock) => {
                let object_fields = get_object_fields_for_enum(tokenizer)?;
                return Ok(EnumDataType::EnumObject(object_fields))
            },
            _ => return Ok(EnumDataType::EnumUnit)
        },
        None => Err(CastleError::AbruptEOF("get_object_or_unit_type'".into()))
    }
}

fn get_object_fields_for_enum<R>(tokenizer: &mut Tokenizer<R>) -> Result<HashMap<Box<str>, SchemaField>, CastleError>
where R: Read {
    tokenizer.next(true)?; // skip openblock
    let mut fields: HashMap<Box<str>, SchemaField> = HashMap::new();
    loop {
        let end_of_object = check_token_and_parse_schema_field_or_break(tokenizer, &mut fields)?;
        if end_of_object { break; }
    }
    return Ok(fields)
}