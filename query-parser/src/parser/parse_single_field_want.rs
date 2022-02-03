use std::io::Read;

use shared::CastleError;

use crate::{tokenizer::tokenizer::Tokenizer, ast::syntax_definitions::want::Want, token::{Token, token::TokenKind}};

pub fn parse_single_field_want<R>(tokenizer: &mut Tokenizer<R>, token: Token) -> Result<Want, CastleError>
where R: Read {
    return match token.kind {
        TokenKind::Identifier(identifier) => Ok(Want::SingleField(identifier)),
        _ => Err(CastleError::Parser(
            format!("Expected identifier, found: {:?}", token.kind).into(),
            token.span,
        ))
    }  
}

