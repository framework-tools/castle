use std::collections::HashMap;

use castle_error::CastleError;
use shared_parser::parse_inputs::{has_separator, parse_inputs};
use tokenizer::{Tokenizable, extensions::{ExpectIdentifier, PeekKeyword, ExpectKeyword}, TokenKind, Token, Keyword, Punctuator};

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
pub fn parse_projection_inner(tokenizer: &mut impl Tokenizable) -> Result<HashMap<Box<str>, Field>, CastleError> {
    let mut projections = HashMap::new();

    loop {
        // peek to check if there is an identifier (EOF is allowed since this can be used for top level projections)
        if let Some(Token { kind: TokenKind::Identifier(_), .. }) = tokenizer.peek(false)? {
            let field = parse_field(tokenizer)?;
            projections.insert(field.name.clone(), field);
            if !has_separator(tokenizer)? {
                break
            }
        } else {
            break
        }
    }

    Ok(projections)
}

fn parse_field(tokenizer: &mut impl Tokenizable) -> Result<Field, CastleError> {
    let name = tokenizer.expect_identifier(true)?;
    let inputs = parse_inputs(tokenizer)?;
    let rename = parse_rename_optional(tokenizer)?;
    let kind = parse_field_kind(tokenizer)?;

    Ok(Field {
        name,
        inputs,
        rename,
        kind,
    })
}

/// field_name as rename_name
///
/// as should be a keyword.
fn parse_rename_optional(tokenizer: &mut impl Tokenizable) -> Result<Option<Box<str>>, CastleError> {
    if let Some(Keyword::As) = tokenizer.peek_keyword(true)? {
        tokenizer.expect_keyword(Keyword::As, true)?;
        Ok(Some(tokenizer.expect_identifier(true)?))
    } else {
        Ok(None)
    }
}


/// 3 types of field kinds
/// - object (hashmap)
/// - list (hashmap) - this is like object but it projects the fields of children instead
/// - scalar/field - no sub projections
fn parse_field_kind(tokenizer: &mut impl Tokenizable) -> Result<FieldKind, CastleError> {
    if let Some(Token { kind: TokenKind::Punctuator(punc), ..}) = tokenizer.peek(true)? {
        match punc {
            Punctuator::OpenBlock => Ok(FieldKind::Object(parse_projection_inner(tokenizer)?)),
            Punctuator::OpenParen => Ok(FieldKind::List(parse_projection_inner(tokenizer)?)),
            _ => Ok(FieldKind::Field),
        }
    } else {
        Ok(FieldKind::Field)
    }
}