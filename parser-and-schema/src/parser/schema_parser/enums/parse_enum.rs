use std::{collections::HashMap, f64::consts::E, io::Read};

use shared::CastleError;

use crate::{tokenizer::tokenizer::{self, Tokenizer}, ast::syntax_definitions::{enum_definition::{EnumDefinition, EnumDataType, EnumVariant}, keyword::Keyword, want::Argument}, parser::schema_parser::{parse_schema_type::{get_identifier_skip_open_block,}, parse_schema_field::get_identifier, types::schema_field::Type}, token::{token::{TokenKind, Punctuator, Identifier}, self}};

pub fn parse_enum_definition<R>(tokenizer: &mut tokenizer::Tokenizer<R>) -> Result<EnumDefinition, CastleError>
    where R: Read {
        let mut variants = HashMap::new();
        let token = tokenizer.next(true)?; /// enum keyword is already passed
        let identifier = get_identifier_skip_open_block(token, tokenizer)?;

        loop {
            let end_of_schema_type = check_token_and_parse_enum_variant_or_break(tokenizer, &mut variants)?;
            if end_of_schema_type { break; }
        }

        return Ok(EnumDefinition { name: identifier, variants, directives: HashMap::new() });
}

fn check_token_and_parse_enum_variant_or_break<R>(tokenizer: &mut Tokenizer<R>, variants: &mut HashMap<Box<str>, EnumVariant>) -> Result<bool, CastleError> 
where R: Read {
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
        None => return Err(CastleError::AbruptEOF)
    }
}


fn insert_variant_in_enum<R>(name: Box<str>, tokenizer: &mut Tokenizer<R>, variants: &mut HashMap<Box<str>, EnumVariant>) -> Result<(), CastleError> 
where R: Read {
    let variant = parse_enum_variant(tokenizer)?;
    variants.insert(name,  variant);
    return Ok(())
}

fn parse_enum_variant<R>(tokenizer: &mut Tokenizer<R>) -> Result<EnumVariant, CastleError> 
where R: Read{
    let token = tokenizer.next(true)?; // should be identifier
    let identifier = match token {
        Some(token) => match token.kind {
            TokenKind::Identifier(identifier) => identifier,
            _ => return Err(CastleError::Schema(format!("3. Unexpected token: {:?}", token.kind).into(), token.span))
        },
        None => return Err(CastleError::AbruptEOF)
    };
    let enum_data_type = parse_enum_data_type(identifier.arguments)?;
    return Ok(EnumVariant { name: identifier.name, enum_data_type, directives: HashMap::new() });
}
/// takes in tokenizer and returns enum data type
/// - if next token is identifier or comma, return enum unit
/// - else if next token is openparen, return enum tuple
/// finish
fn parse_enum_data_type(arguments: Option<Vec<Argument>>) -> Result<EnumDataType, CastleError> {
    match arguments {
        Some(arguments) => return Ok(EnumDataType::EnumTuple(arguments)),
        None => return Ok(EnumDataType::EnumUnit)
    }
}