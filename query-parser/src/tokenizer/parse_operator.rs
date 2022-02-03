use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::CastleError;

use crate::token::{Token, token::{TokenKind, Punctuator}};

use super::tokenizer::get_character_with_peek;


pub fn parse_operator<R>( cursor: &mut Cursor<R>, start: Position ) -> Result<Token, CastleError> 
where R: Read {
    
    let ch = get_character_with_peek(cursor, start)?;

    return Ok(Token::operator_as_str_to_token(&ch, start, cursor.pos()))
}

