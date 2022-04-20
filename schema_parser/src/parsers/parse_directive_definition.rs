use castle_error::CastleError;
use tokenizer::Tokenizable;

use crate::types::DirectiveDefinition;

use super::parse_type_definition::parse_input_definitions;




pub fn parse_directive_definition(tokenizer: &mut impl Tokenizable) -> Result<DirectiveDefinition, CastleError> {
    Ok(DirectiveDefinition{
        name: tokenizer.expect_identifier(true)?,
        input_definitions: parse_input_definitions(tokenizer)?,
        locations: parse_directive_locations(tokenizer)?,
    })
}

pub fn parse_directive_locations(tokenizer: &mut impl Tokenizable) -> Result<Vec<String>, CastleError> {
    let mut locations = Vec::new();
    tokenizer.expect_punctuator(Punctuator::OpenParen, true)?;
    loop {
        if let Token { kind: TokenKind::Punctuator(Punctuator::CloseParen), ..} = tokenizer.peek_expect(true)?.kind {
            tokenizer.expect_punctuator(Punctuator::CloseParen, true);
            return Ok(locations);
        }
        locations.push(tokenizer.expect_identifier(true)?);
    }
}