use std::{collections::HashMap};

use castle_error::CastleError;
use tokenizer::{extensions::{ExpectIdentifier, ExpectPunctuator}, Punctuator, Tokenizable, TokenKind, Token};

use crate::types::{TypeDefinition, FieldDefinition, Directive, InputDefinition};

use super::{parse_directives::parse_directives, parse_kind::parse_kind};



pub(crate) fn parse_type_definition(tokenizer: &mut impl Tokenizable, directives: Vec<Directive>) -> Result<TypeDefinition, CastleError> {
    Ok(TypeDefinition{
        identifier: tokenizer.expect_identifier(true)?,
        fields: parse_fields(tokenizer)?,
        directives,
    })
}

fn parse_fields(tokenizer: &mut impl Tokenizable) -> Result<HashMap<Box<str>, FieldDefinition>, CastleError> {
    let mut fields = HashMap::new();
    tokenizer.expect_punctuator(Punctuator::OpenBlock, true)?;
    loop {
        if let Token { kind: TokenKind::Punctuator(Punctuator::CloseBlock), ..} = tokenizer.peek_expect(true)? {
            tokenizer.expect_punctuator(Punctuator::CloseBlock, true)?;
            return Ok(fields);
        }
        let name = tokenizer.expect_identifier(true)?;
        tokenizer.expect_punctuator(Punctuator::Colon, true)?;
        fields.insert(name.clone(), parse_field_definition(tokenizer, name)?);
    }
}

fn parse_field_definition(tokenizer: &mut impl Tokenizable, name: Box<str>) -> Result<FieldDefinition, CastleError> {
    Ok(FieldDefinition{
        name,
        input_definitions: parse_input_definitions(tokenizer)?,
        return_kind: parse_kind(tokenizer)?,
        directives: parse_directives(tokenizer)?,
    })
}

pub fn parse_input_definitions(tokenizer: &mut impl Tokenizable) -> Result<HashMap<Box<str>, InputDefinition>, CastleError> {
    let mut inputs = HashMap::new();
    if TokenKind::Punctuator(Punctuator::OpenBlock) != tokenizer.peek_expect(true)?.kind {
        return Ok(inputs)
    };
    tokenizer.expect_punctuator(Punctuator::OpenParen, true)?;
    loop {
        if let Token { kind: TokenKind::Punctuator(Punctuator::CloseParen), ..} = tokenizer.peek_expect(true)? {
            tokenizer.expect_punctuator(Punctuator::CloseParen, true)?;
            return Ok(inputs);
        }
        let name = tokenizer.expect_identifier(true)?;
        tokenizer.expect_punctuator(Punctuator::Colon, true)?;
        let input_value = InputDefinition{
            name: name.clone(),
            input_kind: parse_kind(tokenizer)?,
            default: None,
            directives: parse_directives(tokenizer)?,
        };
        inputs.insert(name, input_value);
    }
}

