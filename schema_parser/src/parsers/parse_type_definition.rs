use std::collections::HashMap;

use castle_error::CastleError;
use shared_parser::parse_inputs::consume_optional_separator;
use tokenizer::{
    extensions::{ExpectIdentifier, ExpectPunctuator, IsPunctuator},
    Punctuator, Tokenizable,
};

use crate::types::{Directive, FieldDefinition, InputDefinition, TypeDefinition};

use super::{parse_directives::parse_directives, parse_kind::parse_kind};

pub(crate) fn parse_type_definition(
    tokenizer: &mut impl Tokenizable,
    directives: Vec<Directive>,
) -> Result<TypeDefinition, CastleError> {
    Ok(TypeDefinition {
        ident: tokenizer.expect_identifier(true)?,
        fields: parse_fields(tokenizer)?,
        directives,
    })
}

fn parse_fields(
    tokenizer: &mut impl Tokenizable,
) -> Result<HashMap<Box<str>, FieldDefinition>, CastleError> {
    let mut fields = HashMap::new();
    tokenizer.expect_punctuator(Punctuator::OpenBlock, true)?;
    loop {
        if tokenizer.peek_is_punctuator(Punctuator::CloseBlock, true)? {
            break;
        }
        let name = tokenizer.expect_identifier(true)?;
        tokenizer.expect_punctuator(Punctuator::Colon, true)?;
        fields.insert(name.clone(), parse_field_definition(tokenizer, name)?);
        consume_optional_separator(tokenizer)?;
    }
    tokenizer.expect_punctuator(Punctuator::CloseBlock, true)?;
    return Ok(fields);
}

fn parse_field_definition(
    tokenizer: &mut impl Tokenizable,
    name: Box<str>,
) -> Result<FieldDefinition, CastleError> {
    Ok(FieldDefinition {
        name,
        input_definitions: parse_input_definitions(tokenizer)?,
        return_kind: parse_kind(tokenizer)?,
        directives: parse_directives(tokenizer)?,
    })
}

pub fn parse_input_definitions(
    tokenizer: &mut impl Tokenizable,
) -> Result<HashMap<Box<str>, InputDefinition>, CastleError> {
    let mut inputs = HashMap::new();
    if !tokenizer.peek_is_punctuator(Punctuator::OpenParen, true)? {
        return Ok(inputs);
    };
    tokenizer.expect_punctuator(Punctuator::OpenParen, true)?;
    loop {
        if tokenizer.peek_is_punctuator(Punctuator::CloseParen, true)? {
            break;
        }
        let name = tokenizer.expect_identifier(true)?;
        tokenizer.expect_punctuator(Punctuator::Colon, true)?;
        let input_value = InputDefinition {
            name: name.clone(),
            input_kind: parse_kind(tokenizer)?,
            default: None,
            directives: parse_directives(tokenizer)?,
        };
        inputs.insert(name, input_value);
    }
    tokenizer.expect_punctuator(Punctuator::CloseParen, true)?;
    Ok(inputs)
}
