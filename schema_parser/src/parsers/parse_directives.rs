use castle_error::CastleError;
use shared_parser::parse_inputs::parse_inputs;
use tokenizer::{Tokenizable, Token, TokenKind, Punctuator, extensions::{ExpectPunctuator, ExpectIdentifier}};

use crate::types::Directive;
// "

// directive lowercase (
//     arg_definiton
// )
// type Query {
//     user(arg_definiton: String): User @directive(arg: 123)
// }

// user(user_id: 123) {
//     first_name
// }
// "

pub fn parse_directives(tokenizer: &mut impl Tokenizable) -> Result<Vec<Directive>, CastleError> {
    let mut directives: Vec<Directive> = Vec::new();
    loop {
        match tokenizer.peek(true)? {
            Some(Token {kind: TokenKind::Punctuator(Punctuator::At), ..}) => {
                directives.push(expect_directive(tokenizer)?);
            },
            _ => return Ok(directives)
        }
    }
}


pub fn expect_directive(tokenizer: &mut impl Tokenizable) -> Result<Directive, CastleError> {
    tokenizer.expect_punctuator(Punctuator::At, true)?;
    Ok(Directive {
        name: tokenizer.expect_identifier(false)?,
        inputs: parse_inputs(tokenizer)?,
    })
}

