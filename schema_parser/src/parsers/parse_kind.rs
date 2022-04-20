use castle_error::CastleError;
use shared_parser::parse_inputs::has_more_fields;
use tokenizer::{Tokenizable, TokenKind, extensions::{ExpectIdentifier, ExpectPunctuator}, Punctuator};

use crate::types::Kind;


pub(crate) fn parse_kind(tokenizer: &mut impl Tokenizable) -> Result<Kind, CastleError> {
    let type_name = tokenizer.peek_expect(true)?;
    Ok(Kind {
        name: tokenizer.expect_identifier(true).map_err(|_| CastleError::Schema("Expected type name".into(), type_name.span))?,
        generics: parse_generics(tokenizer)?
    })
}

/// if we see a `<` then we have a generic type
/// otherwise we have a non-generic type
/// we have to check if it is a punctuator
pub(crate) fn parse_generics(tokenizer: &mut impl Tokenizable) -> Result<Vec<Kind>, CastleError> {
    let mut generics = Vec::new();
    match tokenizer.peek_expect(true)?.kind {
        TokenKind::Punctuator(Punctuator::GenericOpen) => {
            tokenizer.expect_punctuator(Punctuator::GenericOpen, true)?;
            loop {
                let token = tokenizer.peek_expect(true)?;
                if token.kind == TokenKind::Punctuator(Punctuator::GenericClose) {
                    if generics.is_empty() {
                        return Err(CastleError::Schema("provide type with no generics".into(), token.span));
                    }
                    tokenizer.expect_punctuator(Punctuator::GenericClose, true)?;
                    break Ok(generics);
                }
                if has_more_fields(tokenizer)? {
                    generics.push(parse_kind(tokenizer)?);
                }
            }
        },
        _ => Ok(generics),
    }
}