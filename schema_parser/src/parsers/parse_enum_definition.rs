use std::collections::HashMap;

use castle_error::CastleError;
use shared_parser::parse_inputs::has_separator;
use tokenizer::{Tokenizable, Punctuator, TokenKind, extensions::{ExpectPunctuator, ExpectIdentifier, IsPunctuator}};

use crate::types::{Directive, EnumDefinition, VariantDefinition, VariantKindDefinition, Kind};

use super::{parse_directives::parse_directives, parse_kind::parse_kind};




pub(crate) fn parse_enum_definition(tokenizer: &mut impl Tokenizable, directives: Vec<Directive>) -> Result<EnumDefinition, CastleError> {
    Ok(EnumDefinition{
        name: tokenizer.expect_identifier(true)?,
        variants: parse_enum_variants(tokenizer)?,
        directives,
    })
}

fn parse_enum_variants(tokenizer: &mut impl Tokenizable) -> Result<Vec<VariantDefinition>, CastleError> {
    let mut values = Vec::new();
    tokenizer.expect_punctuator(Punctuator::OpenBlock, true)?;
    loop {
        if tokenizer.peek_is_punctuator(Punctuator::CloseBlock, true)? {
            break
        }
        values.push(parse_variant_definition(tokenizer)?);
        if !has_separator(tokenizer)? {
            break
        }
    }
    tokenizer.expect_punctuator(Punctuator::CloseBlock, true)?;
    Ok(values)
}

fn parse_variant_definition(tokenizer: &mut impl Tokenizable) -> Result<VariantDefinition, CastleError> {
    Ok(VariantDefinition{
        name: tokenizer.expect_identifier(true)?,
        kind: parse_variant_kind_definition(tokenizer)?,
        directives: parse_directives(tokenizer)?,
    })
}


fn parse_variant_kind_definition(tokenizer: &mut impl Tokenizable) -> Result<VariantKindDefinition, CastleError> {
    return match tokenizer.peek_expect(true)?.kind {
        TokenKind::Punctuator(Punctuator::OpenBlock) => Ok(VariantKindDefinition::Map(parse_map(
            tokenizer,
            Punctuator::OpenBlock,
            Punctuator::CloseBlock,
        )?)),
        TokenKind::Punctuator(Punctuator::OpenParen) => Ok(VariantKindDefinition::Tuple(parse_tuple(
            tokenizer,
            Punctuator::OpenParen,
            Punctuator::CloseParen,
        )?)),
        _ => Ok(VariantKindDefinition::Unit),
        };
}

pub fn parse_map(
    tokenizer: &mut impl Tokenizable,
    opening: Punctuator,
    closing: Punctuator,
) -> Result<HashMap<Box<str>, Kind>, CastleError> {
    tokenizer.expect_punctuator(opening, true)?;
    let mut inputs = HashMap::new();
    loop {
        if tokenizer.peek_is_punctuator(closing, true)? {
            break
        }
        inputs.insert(
            tokenizer.expect_identifier(true)?,
            expect_colon_and_kind(tokenizer)?,
        );
        if !has_separator(tokenizer)? {
            break
        }
    }
    tokenizer.expect_punctuator(closing, true)?;
    Ok(inputs)
}

fn expect_colon_and_kind(tokenizer: &mut impl Tokenizable) -> Result<Kind, CastleError> {
    tokenizer.expect_punctuator(Punctuator::Colon, true)?;
    parse_kind(tokenizer)
}

pub fn parse_tuple(
    tokenizer: &mut impl Tokenizable,
    opening: Punctuator,
    closing: Punctuator,
) -> Result<Vec<Kind>, CastleError> {
    let mut inputs_vec: Vec<Kind> = Vec::new();
    tokenizer.expect_punctuator(opening, true)?;
    loop {
        if tokenizer.peek_is_punctuator(closing, true)? {
            break
        }
        inputs_vec.push(parse_kind(tokenizer)?);
        if !has_separator(tokenizer)? {
            break
        }
    }
    tokenizer.expect_punctuator(closing, true)?;
    Ok(inputs_vec)
}