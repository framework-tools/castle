use std::collections::HashMap;

use castle_error::CastleError;
use shared_parser::parse_inputs::{has_separator, parse_inputs};
use tokenizer::{
    extensions::{ExpectIdentifier, ExpectKeyword, ExpectPunctuator, PeekKeyword},
    Keyword, Punctuator, TokenKind, Tokenizable,
};

use crate::{types::Field, FieldKind};

/// Parses a object projection, except without the {} brackets (so just the fields)
/// ```text
/// {
///     first_name,
///     last_name
///     profile {
///         name
///     }
/// }
/// ```
pub fn parse_projection_inner(
    tokenizer: &mut impl Tokenizable,
) -> Result<HashMap<Box<str>, Field>, CastleError> {
    let mut projections = HashMap::new();

    loop {
        // peek to check if there is an identifier (EOF is allowed since this can be used for top level projections)
        if let Some(TokenKind::Identifier(_)) = tokenizer.peek_token_kind(true)? {
            let field = parse_field(tokenizer)?;
            dbg!(&field.name);
            projections.insert(field.name.clone(), field);
            if !has_separator(tokenizer)? {
                dbg!("test");
                break;
            }
        } else {
            break;
        }
    }

    Ok(projections)
}

fn parse_field(tokenizer: &mut impl Tokenizable) -> Result<Field, CastleError> {
    Ok(Field {
        name: tokenizer.expect_identifier(true)?,
        inputs: if let Some(TokenKind::Punctuator(Punctuator::OpenParen)) =
            tokenizer.peek_token_kind(true)?
        {
            parse_inputs(tokenizer)?
        } else {
            HashMap::new()
        },
        rename: parse_rename_optional(tokenizer)?,
        kind: parse_field_kind(tokenizer)?,
    })
}

/// field_name as rename_name
///
/// as should be a keyword.
fn parse_rename_optional(
    tokenizer: &mut impl Tokenizable,
) -> Result<Option<Box<str>>, CastleError> {
    if let Some(Keyword::As) = tokenizer.peek_keyword(false)? {
        tokenizer.expect_keyword(Keyword::As, true)?;
        Ok(Some(tokenizer.expect_identifier(true)?))
    } else {
        Ok(None)
    }
}

fn parse_projection(
    tokenizer: &mut impl Tokenizable,
    opening: Punctuator,
    closing: Punctuator,
) -> Result<HashMap<Box<str>, Field>, CastleError> {
    tokenizer.expect_punctuator(opening, true)?;
    let projections = parse_projection_inner(tokenizer)?;
    tokenizer.expect_punctuator(closing, true)?;
    Ok(projections)
}

/// 3 types of field kinds
/// - object (hashmap)
/// - list (hashmap) - this is like object but it projects the fields of children instead
/// - scalar/field - no sub projections
fn parse_field_kind(tokenizer: &mut impl Tokenizable) -> Result<FieldKind, CastleError> {
    match tokenizer.peek_token_kind(false)? {
        Some(TokenKind::Punctuator(Punctuator::OpenBlock)) => Ok(FieldKind::Object(
            parse_projection(tokenizer, Punctuator::OpenBlock, Punctuator::CloseBlock)?,
        )),
        Some(TokenKind::Punctuator(Punctuator::OpenBracket)) => Ok(FieldKind::List(
            parse_projection(tokenizer, Punctuator::OpenBracket, Punctuator::CloseBracket)?,
        )),
        _ => Ok(FieldKind::Field),
    }
}
