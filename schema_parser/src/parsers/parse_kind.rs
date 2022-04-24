use castle_error::CastleError;
use shared_parser::parse_inputs::has_separator;
use tokenizer::{Tokenizable, TokenKind, extensions::{ExpectIdentifier, ExpectPunctuator, IsPunctuator}, Punctuator, Token};

use crate::types::Kind;


pub(crate) fn parse_kind(tokenizer: &mut impl Tokenizable) -> Result<Kind, CastleError> {
    Ok(Kind {
        name: tokenizer.expect_identifier(true)?,
        generics: parse_generics(tokenizer)?
    })
}

/// if we see a `<` then we have a generic type
/// otherwise we have a non-generic type
/// we have to check if it is a punctuator
pub(crate) fn parse_generics(tokenizer: &mut impl Tokenizable) -> Result<Vec<Kind>, CastleError> {
    let mut generics = Vec::new();
    let Token { kind, span } = tokenizer.peek_expect(true)?;
    let span = span.clone();
    match kind {
        TokenKind::Punctuator(Punctuator::GenericOpen) => {
            tokenizer.expect_punctuator(Punctuator::GenericOpen, true)?;
            loop {
                if tokenizer.peek_is_punctuator(Punctuator::GenericClose, true)? {
                    break
                }
                generics.push(parse_kind(tokenizer)?);
                if !has_separator(tokenizer)? {
                    break
                }
            }

            if generics.is_empty() {
                return Err(CastleError::Schema("Generics <T> need at least one type argument".into(), span));
            }

            tokenizer.expect_punctuator(Punctuator::GenericClose, true)?;
            Ok(generics)
        },
        _ => Ok(generics),
    }
}