use std::collections::HashMap;

use castle_shared_parser::parse_inputs::{consume_optional_separator};
use castle_tokenizer::{
    extensions::{ExpectIdentifier, ExpectPunctuator, IsPunctuator},
    Punctuator, Tokenizable,
};
use castle_types::{AppliedDirective, TypeDefinition, CastleError, FieldDefinition};


use super::{
    parse_directives::parse_directives,
    parse_input_type_definition::parse_optional_input_definitions, parse_kind::parse_kind,
};

pub(crate) fn parse_type_definition(
    tokenizer: &mut impl Tokenizable,
    directives: Vec<AppliedDirective>,
) -> Result<TypeDefinition, CastleError> {
    Ok(TypeDefinition {
        ident: tokenizer.expect_identifier(true)?,
        fields: parse_fields(tokenizer)?,
        directives,
    })
}

fn parse_fields(
    tokenizer: &mut impl Tokenizable,
) -> Result<HashMap<Box<str>, FieldDefinition>, CastleError> {
    let mut fields = HashMap::new();
    tokenizer.expect_punctuator(Punctuator::OpenBlock, true)?;
    loop {
        if tokenizer.peek_is_punctuator(Punctuator::CloseBlock, true)? {
            break;
        }
        let name = tokenizer.expect_identifier(true)?;
        fields.insert(name.clone(), parse_field_definition(tokenizer, name)?);
        consume_optional_separator(tokenizer)?;
    }
    tokenizer.expect_punctuator(Punctuator::CloseBlock, true)?;
    return Ok(fields);
}

fn parse_field_definition(
    tokenizer: &mut impl Tokenizable,
    ident: Box<str>,
) -> Result<FieldDefinition, CastleError> {
    Ok(FieldDefinition {
        ident,
        input_definitions: parse_optional_input_definitions(
            tokenizer,
            Punctuator::OpenParen,
            Punctuator::CloseParen,
        )?,
        return_kind: {
            tokenizer.expect_punctuator(Punctuator::Colon, true)?;
            parse_kind(tokenizer)?
        },
        directives: parse_directives(tokenizer)?,
    })
}
