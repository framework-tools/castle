use std::collections::HashSet;

use castle_error::CastleError;
use tokenizer::{Tokenizable, extensions::{ExpectIdentifier, ExpectKeyword, ExpectPunctuator}, Keyword, TokenKind, Punctuator};

use crate::types::{DirectiveDefinition, DirectiveLocation};

use super::parse_type_definition::parse_input_definitions;




pub fn parse_directive_definition(tokenizer: &mut impl Tokenizable) -> Result<DirectiveDefinition, CastleError> {
    tokenizer.expect_keyword(&Keyword::Directive, true)?;
    Ok(DirectiveDefinition{
        name: tokenizer.expect_identifier(true)?, // 
        input_definitions: parse_input_definitions(tokenizer)?,
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
            "FieldDirective" => directive_locations.insert(DirectiveLocation::FieldDirective),
            "EnumDirective" => directive_locations.insert(DirectiveLocation::EnumDirective),
            "VariantDirective" => directive_locations.insert(DirectiveLocation::VariantDirective),
            "InputDirective" => directive_locations.insert(DirectiveLocation::InputDirective),
            "TypeDirective" => directive_locations.insert(DirectiveLocation::TypeDirective),
            str => return Err(CastleError::Schema(format!("Expected directive location, found: {:?}", str).into(), err_location))
        };
        if let TokenKind::Punctuator(Punctuator::Or) = tokenizer.peek_expect(true)?.kind {
            tokenizer.expect_punctuator(Punctuator::Or, true)?;
        } else {
            break;
        }
    }
    return Ok(directive_locations)
}