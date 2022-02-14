use std::io::Read;

use input_cursor::{Position, Span};
use shared::CastleError;

use crate::{ast::syntax_definitions::argument::ArgumentOrTuple, token::{Token, token::{TokenKind, Identifier}}};

use super::{tokenizer::Tokenizer, parse_arguments::get_arguments};

pub fn parse_identifier_token<R>(tokenizer: &mut Tokenizer<R>, word: String, start: Position, has_arguments: bool)
-> Result<Token, CastleError> where R: Read {
    let arguments = parse_arguments(tokenizer, start, has_arguments)?;

    return Ok(Token::new(TokenKind::Identifier(Identifier {
        name: word.into(),
        arguments
    }), Span::new(start, tokenizer.cursor.pos())))
}

pub fn parse_arguments<R>(tokenizer: &mut Tokenizer<R>, start: Position, has_arguments: bool) -> Result<Option<Vec<ArgumentOrTuple>>, CastleError> 
where R: Read {
    let arguments;
    if has_arguments { arguments = Some(get_arguments(tokenizer)?); } // this also is used for tuples on enums
    else { arguments = None }
    return Ok(arguments)
}