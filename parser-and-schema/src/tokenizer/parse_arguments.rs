use std::io::Read;

use input_cursor::Cursor;
use shared::CastleError;

use crate::{ast::syntax_definitions::argument::Argument, token::token::{Punctuator, TokenKind}};

use super::tokenizer::{advance_and_parse_token, Tokenizer};

/// Takes in Cursor returns arguments token
///  - The '(' is already consumed
///  - if ')' return token
///  - else if, ',' create token from argument, then push token to arguments
///  - else push character to current argument
pub fn get_arguments<R>(tokenizer: &mut Tokenizer<R>) -> Result<Vec<Argument>, CastleError> 
where R: Read {
    let mut arguments = Vec::new();
    loop {
        let end_of_arguments = unwrap_char_parse_argument_or_end(&mut arguments, tokenizer)?;
        if end_of_arguments { break; }
    }
    return Ok(arguments)
}

fn unwrap_char_parse_argument_or_end<R>(arguments: &mut Vec<Argument>, tokenizer: &mut Tokenizer<R>) -> Result<bool, CastleError>
where R: Read {
    let peeked_token = tokenizer.peek(true)?;
    match peeked_token {
        Some(peeked_token) => match peeked_token.kind {
            TokenKind::Punctuator(Punctuator::CloseParen) => {
                tokenizer.next(true)?; // skip close parenthesis
                return Ok(true); //break loop
            },
            TokenKind::Punctuator(Punctuator::Comma) => {
                tokenizer.next(true)?; // skip comma
                return Ok(false);
            },
            _ => {
                let token = tokenizer.next(true)?.unwrap(); //we know it's not None
                let argument = Argument::new(token, tokenizer)?;
                arguments.push(argument);
                return Ok(false);
            }
        }
        None =>return Err(CastleError::AbruptEOF("Error found in 'unwrap_char_parse_argument_or_end'".into())),
    }
}