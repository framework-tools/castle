use std::collections::HashSet;

use castle_error::CastleError;
use tokenizer::{Tokenizable, extensions::ExpectIdentifier};

use crate::types::{DirectiveDefinition, DirectiveLocation};

use super::parse_type_definition::parse_input_definitions;




pub fn parse_directive_definition(tokenizer: &mut impl Tokenizable) -> Result<DirectiveDefinition, CastleError> {
    Ok(DirectiveDefinition{
        name: tokenizer.expect_identifier(true)?,
        input_definitions: parse_input_definitions(tokenizer)?,
        locations: parse_directive_locations(tokenizer)?,
    })
}

pub fn parse_directive_locations(tokenizer: &mut impl Tokenizable) -> Result<HashSet<DirectiveLocation>, CastleError> {
    unimplemented!()
}