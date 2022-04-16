use std::io::Read;


use shared::castle_error::CastleError;

use crate::{ast::syntax_definitions::argument::{ArgumentOrTuple, match_token_to_parse_argument}, token::{token::{Punctuator, TokenKind}, Token}};

use super::{tokenizer::{ Tokenizer}, tokenizer_utils::{ get_next_token_and_unwrap}};

pub fn parse_arguments_with_colon<R>(tokenizer: &mut Tokenizer<R>) -> Result<Vec<ArgumentOrTuple>, CastleError> 
where R: Read {
    let mut arguments = Vec::new();
    loop {
        let end_of_arguments = parse_argument(&mut arguments, tokenizer)?;
        if end_of_arguments { break; }
    }
    return Ok(arguments)
}

pub(crate) fn parse_arguments<R>(tokenizer: &mut Tokenizer<R>) -> Result<Vec<ArgumentOrTuple>, CastleError> 
where R: Read {
    let mut arguments = Vec::new();
    loop {
        if let Some(Token { kind: TokenKind::Punctuator(Punctuator::CloseParen), .. }) = tokenizer.peek(true)? {
            break;
        }
        arguments.push(parse_argument(tokenizer)?);
    }
    return Ok(arguments)
}

fn parse_argument<R>(tokenizer: &mut Tokenizer<R>) -> Result<Ar, CastleError>
where R: Read {
    let token = get_next_token_and_unwrap(tokenizer)?;
    match token.kind {
        TokenKind::Identifier(identifier) => {
            let name = identifier.name;
            let token = get_next_token_and_unwrap(tokenizer)?;
            let argument = match_token_to_parse_argument(token, tokenizer, name)?;
            arguments.push(argument);
            return Ok(false);
        },
        TokenKind::Punctuator(Punctuator::Comma) => return Ok(false),
        TokenKind::Punctuator(Punctuator::CloseParen) => return Ok(true),
        _ => return Err(CastleError::Schema(format!("Expected identifier, Comma or CloseParen found: {:?}", token.kind).into(), token.span))
    }
}