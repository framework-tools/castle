use std::collections::HashMap;
use castle_error::CastleError;
use tokenizer::{
    extensions::{ExpectIdentifier, ExpectPunctuator, IsPunctuator, ExpectPrimitive},
    Punctuator, TokenKind, Tokenizable,
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
        TokenKind::Primitive(_) => Input::Primitive(tokenizer.expect_primitive(true)?),
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
        if tokenizer.peek_is_punctuator(closing, true)? {
            break
        }
        inputs.insert(
            tokenizer.expect_identifier(true)?,
            expect_colon_and_value(tokenizer)?,
        );
        consume_optional_separator(tokenizer)?;
    }
    tokenizer.expect_punctuator(closing, true)?;
    Ok(inputs)
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
        if tokenizer.peek_is_punctuator(closing, true)? {
            break
        }
        inputs_vec.push(parse_value(tokenizer)?);
        consume_optional_separator(tokenizer)?;
    }
    tokenizer.expect_punctuator(closing, true)?;
    return Ok(inputs_vec);
}

/// consume_optional_separator consumes a single valid set combination of seperators,
///
/// No separator is needed.
///
/// We expect that multiple new-lines have been already coaleced into a single newline by the
/// tokenizer, so we can just check for a newline.
pub fn consume_optional_separator(tokenizer: &mut impl Tokenizable) -> Result<(), CastleError> {
    match tokenizer.peek_token_kind(false)? {
        Some(TokenKind::Punctuator(Punctuator::Comma)) => {
            tokenizer.expect_punctuator(Punctuator::Comma, false)?;
            tokenizer.peek(true)?; // peek and skip any line terminators
        },
        Some(TokenKind::LineTerminator) => match tokenizer.peek_token_kind(true)? {
            // handle the case where there is a newline followed by a comma
            Some(TokenKind::Punctuator(Punctuator::Comma)) => {
                tokenizer.expect_punctuator(Punctuator::Comma, true)?;
                // peek and skip any additional line terminators
                tokenizer.peek(true)?;
            },
            // newlines alone are considered valid separators
            _ => {}
        }
        _ => {}
    }

    Ok(())
}