use std::io::Read;

use shared::CastleError;

use crate::token::Token;

use super::tokenizer::Tokenizer;

pub fn get_next_token_and_unwrap<R>(tokenizer: &mut Tokenizer<R>) -> Result<Token, CastleError>
where R: Read {
    let token = tokenizer.next(true)?;
    return match token {
        Some(token) => Ok(token),
        None => Err(CastleError::AbruptEOF("get_next_token_and_unwrap: token is None".into()))
    }
}

pub fn peek_next_token_and_unwrap<R>(tokenizer: &mut Tokenizer<R>) -> Result<&Token, CastleError>
where R: Read {
    let token = tokenizer.peek(true)?;
    return match token {
        Some(token) => Ok(token),
        None => Err(CastleError::AbruptEOF("peek_next_token_and_unwrap: token is None".into()))
    }
}