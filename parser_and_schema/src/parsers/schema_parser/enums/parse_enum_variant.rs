use std::{collections::HashMap, io::Read};



use shared::castle_error::CastleError;

use crate::{tokenizer::{tokenizer::Tokenizer, tokenizer_utils::{peek_next_token_and_unwrap, get_next_token_and_unwrap}}, ast::syntax_definitions::{enum_definition::{EnumVariant, EnumDataType}, argument::ArgumentOrTuple, field_definition::FieldDefinition}, token::token::{TokenKind, Identifier, Punctuator}, parsers::schema_parser::{types::{ parse_directive::parse_directives}, parse_schema_type::check_token_and_parse_schema_field_or_break}};

use super::parse_enum::insert_variant_in_enum;

pub fn check_token_and_parse_enum_variant_or_break<R>(
    tokenizer: &mut Tokenizer<R>, 
    variants: &mut HashMap<Box<str>, EnumVariant>
) -> Result<bool, CastleError>  where R: Read {
    let peeked_token = peek_next_token_and_unwrap(tokenizer)?; // get field identifier or closeblock
    match &peeked_token.kind {
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
        _ => return Err(CastleError::Schema(format!("2. Unexpected token: {:?}", peeked_token.kind).into(), peeked_token.span))
    }
}

pub fn parse_enum_variant<R>(tokenizer: &mut Tokenizer<R>) -> Result<EnumVariant, CastleError> 
where R: Read{
    let token = get_next_token_and_unwrap(tokenizer)?; // should be identifier
    let identifier = match token.kind {
        TokenKind::Identifier(identifier) => identifier,
        _ => return Err(CastleError::Schema(format!("4. Unexpected token: {:?}", token.kind).into(), token.span))
    };
    let enum_data_type = parse_enum_data_type(identifier.arguments, tokenizer)?;
    let directives = parse_directives(tokenizer)?;
    return Ok(EnumVariant { name: identifier.name, enum_data_type, directives});
}

/// takes in tokenizer and returns enum data type
/// - if next token is identifier or comma, return enum unit
/// - else if next token is openparen, return enum tuple
/// finish
fn parse_enum_data_type<R>(arguments: Option<Vec<ArgumentOrTuple>>, tokenizer: &mut Tokenizer<R>)
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
    let peeked_token = peek_next_token_and_unwrap(tokenizer)?;
    match peeked_token.kind {
        TokenKind::Punctuator(Punctuator::OpenBlock) => {
            let object_fields = get_object_fields_for_enum(tokenizer)?;
            return Ok(EnumDataType::EnumObject(object_fields))
        },
        _ => return Ok(EnumDataType::EnumUnit)
    }
}

fn get_object_fields_for_enum<R>(tokenizer: &mut Tokenizer<R>) -> Result<HashMap<Box<str>, FieldDefinition>, CastleError>
where R: Read {
    tokenizer.next(true)?; // skip openblock
    let mut fields: HashMap<Box<str>, FieldDefinition> = HashMap::new();
    loop {
        let end_of_object = check_token_and_parse_schema_field_or_break(tokenizer, &mut fields)?;
        if end_of_object { break; }
    }
    return Ok(fields)
}