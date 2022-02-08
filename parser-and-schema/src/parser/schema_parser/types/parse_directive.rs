use std::io::Read;

use shared::CastleError;

use crate::{tokenizer::tokenizer::Tokenizer, ast::syntax_definitions::{directive_definition::DirectiveDefinition, keyword::Keyword}, token::{token::TokenKind, Token}};

use super::type_system::parse_type;


/// takes in tokenizer and returns parsed directive
///     - get next token
///     - match token
///     - if token is into keyword, get next token and parse directive type and return
///     - else return none 
pub fn parse_directive<R>(tokenizer: &mut Tokenizer<R>) -> Result<Option<DirectiveDefinition>, CastleError> 
where R: Read{
    let token = unwrap_peeked_token(tokenizer)?;
    match token.kind {
        TokenKind::Keyword(Keyword::Into) => {
            tokenizer.next(true)?; // skip into
            let type_ = parse_type(tokenizer)?;
            return Ok(Some(DirectiveDefinition { type_ }));
        },
        _ => return Ok(None)
    }
}

pub fn unwrap_peeked_token<R>(tokenizer: &mut Tokenizer<R>) -> Result<&Token, CastleError>
where R: Read{
    let option_token = tokenizer.peek(true)?;
    return match option_token {
        Some(token) => Ok(token),
        None => Err(CastleError::AbruptEOF("Error found in 'unwrap_next_token'".into()))
    }
}
