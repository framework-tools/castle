use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::castle_error::CastleError;

use crate::{Token, TokenKind};

pub fn parse_newline(cursor: &mut Cursor<impl Read>, start: Position) -> Result<Token, CastleError> {
    cursor.next_char()?;
    Ok(Token::new(TokenKind::LineTerminator, Span::new(start, cursor.pos())))
}