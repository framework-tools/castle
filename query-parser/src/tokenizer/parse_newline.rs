use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::CastleError;

use crate::token::{Token, token::TokenKind};

pub fn parse_newline<R>(
    cursor: &mut Cursor<R>,
    start: Position,
) -> Result<Token, CastleError> where
    R: Read,
{
    Ok(Token::new(TokenKind::LineTerminator, Span::new(start, cursor.pos())))
}