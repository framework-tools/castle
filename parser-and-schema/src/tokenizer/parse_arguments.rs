use std::io::Read;

use input_cursor::Cursor;
use shared::CastleError;

use crate::{ast::syntax_definitions::argument::Argument, token::{token::{Punctuator, TokenKind}, Token}};

use super::{tokenizer::{advance_and_parse_token, Tokenizer}, tokenizer_utils::{peek_next_token_and_unwrap, get_next_token_and_unwrap}};

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
    let token = get_next_token_and_unwrap(tokenizer)?;
    return match token.kind {
        TokenKind::Punctuator(Punctuator::CloseParen) => Ok(true), //break loop
        TokenKind::Punctuator(Punctuator::Comma) => Ok(false),
        _ => {
            add_argument_to_arguments_list(tokenizer, arguments, token)?;
            return Ok(false);
        }
    }
}

fn add_argument_to_arguments_list<R>(tokenizer: &mut Tokenizer<R>, arguments: &mut Vec<Argument>, token: Token)
-> Result<(), CastleError> 
where R: Read {
    let argument = Argument::new(token, tokenizer)?;
    arguments.push(argument);
    return Ok(())
}