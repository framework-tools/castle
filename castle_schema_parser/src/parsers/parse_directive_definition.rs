use std::collections::HashSet;

use castle_error::CastleError;
use castle_tokenizer::{Tokenizable, extensions::{ExpectIdentifier, ExpectPunctuator, IsPunctuator}, Punctuator};

use crate::types::{DirectiveDefinition, DirectiveLocation};

use super::parse_input_type_definition::parse_optional_input_definitions;

pub fn parse_directive_definition(tokenizer: &mut impl Tokenizable) -> Result<DirectiveDefinition, CastleError> {
    tokenizer.expect_punctuator(Punctuator::At, true)?;
    Ok(DirectiveDefinition{
        ident: tokenizer.expect_identifier(false)?,
        input_definitions: parse_optional_input_definitions(tokenizer, Punctuator::OpenParen, Punctuator::CloseParen)?,
        locations: parse_directive_locations(tokenizer)?,
    })
}

pub fn parse_directive_locations(tokenizer: &mut impl Tokenizable) -> Result<HashSet<DirectiveLocation>, CastleError> {
    let mut directive_locations = HashSet::new();
    tokenizer.expect_identifier(true)?; // consume on
    loop {
        let err_location = tokenizer.peek(true)?.unwrap().span;
        let token = tokenizer.expect_identifier(true)?;
        match &token[..] {
            "FieldDefinition" => directive_locations.insert(DirectiveLocation::FieldDefinition),
            "EnumDefinition" => directive_locations.insert(DirectiveLocation::EnumDefinition),
            "VariantDefinition" => directive_locations.insert(DirectiveLocation::VariantDefinition),
            "InputDefinition" => directive_locations.insert(DirectiveLocation::InputDefinition),
            "TypeDefinition" => directive_locations.insert(DirectiveLocation::TypeDefinition),
            "InputFieldDefinition" => directive_locations.insert(DirectiveLocation::InputFieldDefinition),
            str => return Err(CastleError::Schema(format!("Expected directive location, found: {:?}", str).into(), err_location))
        };
        if tokenizer.peek_is_punctuator(Punctuator::Or, true)? {
            tokenizer.expect_punctuator(Punctuator::Or, true)?;
        } else {
            break
        }
    }
    return Ok(directive_locations)
}