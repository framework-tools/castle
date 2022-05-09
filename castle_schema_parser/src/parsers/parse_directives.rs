use castle_error::CastleError;
use castle_shared_parser::parse_inputs::parse_optional_inputs;
use castle_tokenizer::{
    extensions::{ExpectIdentifier, ExpectPunctuator, IsPunctuator},
    Punctuator, Tokenizable,
};

use crate::types::Directive;
/// ```text
/// directive lowercase (
///     arg_definiton
/// )
/// type Root {
///     user(arg_definiton: String): User @directive(arg: 123)
/// }
/// user(user_id: 123) {
///     first_name
/// }
/// ```

pub fn parse_directives(tokenizer: &mut impl Tokenizable) -> Result<Vec<Directive>, CastleError> {
    let mut directives: Vec<Directive> = Vec::new();
    loop {
        match tokenizer.peek_is_punctuator(Punctuator::At, true)? {
            true => directives.push(expect_directive(tokenizer)?),
            false => return Ok(directives),
        }
    }
}

pub fn expect_directive(tokenizer: &mut impl Tokenizable) -> Result<Directive, CastleError> {
    tokenizer.expect_punctuator(Punctuator::At, true)?;
    Ok(Directive {
        ident: tokenizer.expect_identifier(false)?,
        inputs: parse_optional_inputs(tokenizer)?,
    })
}
