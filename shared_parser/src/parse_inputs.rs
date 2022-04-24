use std::collections::HashMap;
use castle_error::CastleError;
use tokenizer::{
    extensions::{ExpectIdentifier, ExpectPunctuator},
    Punctuator, TokenKind, Tokenizable, Token,
};
use crate::{Input, Variant, VariantType};


pub fn parse_inputs(
    tokenizer: &mut impl Tokenizable,
) -> Result<HashMap<Box<str>, Input>, CastleError> {
    parse_map(tokenizer, Punctuator::OpenParen, Punctuator::CloseParen)
}

fn parse_value(tokenizer: &mut impl Tokenizable) -> Result<Input, CastleError> {
    let value = tokenizer.peek_expect(true)?;
    Ok(match &value.kind {
        TokenKind::Identifier(_) => Input::Variant(parse_variant(tokenizer)?), // ident // this is for tuples
        TokenKind::Primitive(value) => Input::Primitive(value.clone()),
        TokenKind::Punctuator(Punctuator::OpenBlock) => Input::Map(parse_map(
            tokenizer,
            Punctuator::OpenBlock,
            Punctuator::CloseBlock,
        )?),
        TokenKind::Punctuator(Punctuator::OpenBracket) => Input::List(parse_list(
            tokenizer,
            Punctuator::OpenBracket,
            Punctuator::CloseBracket,
        )?),
        _ => Err(CastleError::Schema(
            "Expected primitive, map, list, or variant".into(),
            value.span,
        ))?
    })
}

fn expect_colon_and_value(tokenizer: &mut impl Tokenizable) -> Result<Input, CastleError> {
    tokenizer.expect_punctuator(Punctuator::Colon, true)?;
    parse_value(tokenizer)
}

fn parse_variant(tokenizer: &mut impl Tokenizable) -> Result<Variant, CastleError> {
    Ok(Variant {
        ident: tokenizer.expect_identifier(true)?,
        value: match tokenizer.peek_expect(true)?.kind {
            TokenKind::Punctuator(Punctuator::OpenBlock) => VariantType::Map(parse_map(
                tokenizer,
                Punctuator::OpenBlock,
                Punctuator::CloseBlock,
            )?),
            TokenKind::Punctuator(Punctuator::OpenParen) => VariantType::Tuple(parse_list(
                tokenizer,
                Punctuator::OpenParen,
                Punctuator::CloseParen,
            )?),
            _ => VariantType::Unit,
        },
    })
}

pub fn parse_map(
    tokenizer: &mut impl Tokenizable,
    opening: Punctuator,
    closing: Punctuator,
) -> Result<HashMap<Box<str>, Input>, CastleError> {
    tokenizer.expect_punctuator(opening, true)?;
    let mut inputs = HashMap::new();
    loop {
        inputs.insert(
            tokenizer.expect_identifier(true)?,
            expect_colon_and_value(tokenizer)?,
        );
        if !has_more_fields(tokenizer)? {
            tokenizer.expect_punctuator(closing, true)?;
            return Ok(inputs);
        }
    }
}

// array brackets OPENING value, value CLOSING
pub fn parse_list(
    tokenizer: &mut impl Tokenizable,
    opening: Punctuator,
    closing: Punctuator,
) -> Result<Vec<Input>, CastleError> {
    let mut inputs_vec: Vec<Input> = Vec::new();
    tokenizer.expect_punctuator(opening, true)?;
    loop {
        inputs_vec.push(parse_value(tokenizer)?);
        if !has_more_fields(tokenizer)? {
            tokenizer.expect_punctuator(closing, true)?;
            return Ok(inputs_vec);
        }
    }
}

/// has_separator checks and consumes for valid combination of seperators,
///     returns Ok(true) if there was a valid seperator
///     returns Ok(false) if there was no seperator
///
/// We expect that multiple new-lines have been already coaleced into a single newline by the
/// tokenizer.
///
/// The ,,
fn has_separator(tokenizer: &mut impl Tokenizable) -> Result<bool, CastleError> {
    match tokenizer.peek_token_kind(false)? {
        Some(TokenKind::Punctuator(Punctuator::Comma)) => {
            tokenizer.expect_punctuator(Punctuator::Comma, false)?;
            tokenizer.peek(true)?; // peek and skip any line terminators
            return Ok(true);
        },
        Some(TokenKind::LineTerminator) => match tokenizer.peek_token_kind(false)? {
            // handle the case where there is a newline followed by a comma
            Some(TokenKind::Punctuator(Punctuator::Comma)) => {
                tokenizer.expect_punctuator(Punctuator::Comma, true)?;
                // peek and skip any additional line terminators
                tokenizer.peek(true)?;
                return Ok(true);
            },
            // newlines alone are considered valid separators
            _ => return Ok(true),
        }
        _ => return Ok(false),
    }
}

pub fn has_more_fields(tokenizer: &mut impl Tokenizable) -> Result<bool, CastleError> {
    let token = tokenizer.peek_expect(false)?;
    match token.kind {
        TokenKind::Punctuator(
            Punctuator::CloseBlock | Punctuator::CloseBracket | Punctuator::CloseParen | Punctuator::GenericClose,
        ) => return Ok(false),
        TokenKind::Punctuator(Punctuator::Comma) => {
            tokenizer.expect_punctuator(Punctuator::Comma, false)?; // consume the comma
            match tokenizer.peek_expect(true)?.kind() {
                TokenKind::Punctuator(
                    Punctuator::CloseBlock | Punctuator::CloseBracket | Punctuator::CloseParen | Punctuator::GenericClose,
                ) => return Ok(false), // we don't care if the close is a block or bracket
                _ => return Ok(true),
            }
        }
        TokenKind::LineTerminator => {
            tokenizer.next(false)?; // consume the line terminator
            match tokenizer.peek_expect(true)?.kind() {
                TokenKind::Punctuator(Punctuator::Comma) => return Ok(true),
                _ => return Ok(false), // we don't care if the close is a block or bracket
            }
        },
        _ => Err(CastleError::parse(
            "Expected comma, line terminator or close block",
            token.span,
        ))?,
    }
}
