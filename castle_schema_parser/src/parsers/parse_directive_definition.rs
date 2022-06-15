use castle_tokenizer::{
    extensions::{ExpectIdentifier, ExpectPunctuator},
    Punctuator, Tokenizable,
};
use castle_types::{CastleError, DirectiveDefinition};

use super::parse_input_type_definition::parse_optional_input_definitions;

pub fn parse_directive_definition(
    tokenizer: &mut impl Tokenizable,
) -> Result<DirectiveDefinition, CastleError> {
    tokenizer.expect_punctuator(Punctuator::At, true)?;
    Ok(DirectiveDefinition {
        ident: tokenizer.expect_identifier(false)?,
        input_definitions: parse_optional_input_definitions(
            tokenizer,
            Punctuator::OpenParen,
            Punctuator::CloseParen,
        )?,
    })
}
