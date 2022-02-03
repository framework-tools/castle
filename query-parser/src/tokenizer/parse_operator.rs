use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::CastleError;

use crate::token::{Token, token::{TokenKind, Punctuator}};


pub fn parse_operator<R>( cursor: &mut Cursor<R>, start: Position ) -> Result<Token, CastleError> 
where R: Read {
    
    let c = cursor.next_char()?.ok_or(CastleError::AbruptEOF)?;
    let ch = char::try_from(c).ok().ok_or(CastleError::lex(
        "invalid character",
        cursor.pos(),
    ))?;
    return Ok(Token::operator_as_str_to_token(&ch, start, cursor.pos()))
}

