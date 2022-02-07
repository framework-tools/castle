use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::CastleError;

use crate::{ast::syntax_definitions::argument::Argument, token::{Token, token::{TokenKind, Identifier}}};

pub fn parse_identifier_token<R>(cursor: &mut Cursor<R>, word: String, start: Position, arguments: Option<Vec<Argument>>)
-> Result<Token, CastleError> where R: Read {
    return Ok(Token::new(TokenKind::Identifier(Identifier {
        name: word.into(),
        arguments
        }), Span::new(start, cursor.pos())))
}