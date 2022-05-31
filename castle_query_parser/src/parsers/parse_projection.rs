use std::collections::HashMap;

use castle_shared_parser::parse_inputs::{parse_inputs, consume_optional_separator};
use castle_tokenizer::{
    extensions::{ExpectIdentifier, ExpectKeyword, ExpectPunctuator, PeekKeyword, IsPunctuator},
    Keyword, Punctuator, TokenKind, Tokenizable,
};
use castle_types::{Field, CastleError, FieldKind};


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
        match tokenizer.peek(true)?.map(|t| (&t.kind, &t.span)) {
            Some((TokenKind::Identifier(_), ..)) => {
                let field = parse_field(tokenizer)?;
                projections.insert(field.name.clone(), field);
                consume_optional_separator(tokenizer)?;
            },
            _ => break, // EOF or something else
        }
    }

    Ok(projections)
}

fn parse_field(tokenizer: &mut impl Tokenizable) -> Result<Field, CastleError> {
    Ok(Field {
        name: tokenizer.expect_identifier(true)?,
        inputs: if tokenizer.peek_is_punctuator(Punctuator::OpenParen, true)? {
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

pub(crate) fn parse_projection(
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
