use std::collections::HashMap;

use castle_error::CastleError;
use tokenizer::{Tokenizable, Punctuator, extensions::{IsPunctuator, ExpectPunctuator, ExpectIdentifier}};

use crate::types::{InputTypeDefinition, Directive, InputDefinition};

use super::{parse_kind::parse_kind, parse_directives::parse_directives};



pub(crate) fn parse_input_type_definition(
    tokenizer: &mut impl Tokenizable,
    directives: Vec<Directive>,
) -> Result<InputTypeDefinition, CastleError> {
    Ok(InputTypeDefinition {
        ident: tokenizer.expect_identifier(true)?,
        input_definitions: parse_input_definitions(tokenizer, Punctuator::OpenBlock, Punctuator::CloseBlock)?,
        directives,
    })
}

pub fn parse_input_definitions(
    tokenizer: &mut impl Tokenizable,
    opening: Punctuator,
    closing: Punctuator,
) -> Result<HashMap<Box<str>, InputDefinition>, CastleError> {
    let mut inputs = HashMap::new();
    tokenizer.expect_punctuator(opening, true)?;
    loop {
        if tokenizer.peek_is_punctuator(closing, true)? {
            break;
        }
        let name = tokenizer.expect_identifier(true)?;
        tokenizer.expect_punctuator(Punctuator::Colon, true)?;
        let input_value = InputDefinition {
            ident: name.clone(),
            input_kind: parse_kind(tokenizer)?,
            default: None,
            directives: parse_directives(tokenizer)?,
        };
        inputs.insert(name, input_value);
    }
    tokenizer.expect_punctuator(closing, true)?;
    Ok(inputs)
}

pub(crate) fn parse_optional_input_definitions(
    tokenizer: &mut impl Tokenizable,
    opening: Punctuator,
    closing: Punctuator,
) -> Result<HashMap<Box<str>, InputDefinition>, CastleError> {
    if tokenizer.peek_is_punctuator(opening, true)? {
        Ok(parse_input_definitions(tokenizer, opening, closing)?)
    } else {
        Ok(HashMap::new())
    }
}