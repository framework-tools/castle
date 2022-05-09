use std::io::Read;

use castle_error::CastleError;
use castle_input_cursor::{Cursor, Position, Span};


use crate::{Token, TokenKind};

pub fn parse_newline(cursor: &mut Cursor<impl Read>, start: Position) -> Result<Token, CastleError> {
    // peek the next char in a loop and coalesce all line terminators into a single newline token
    loop {
        match cursor.peek()? {
            Some(b'\n' | b'\r') => cursor.next_byte()?,
            _ => break Ok(Token::new(TokenKind::LineTerminator, Span::new(start, cursor.pos()))),
        };
    }
}