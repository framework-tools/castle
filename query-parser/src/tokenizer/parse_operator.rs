use std::io::Read;

use input_cursor::{Cursor, Position};
use shared::CastleError;

use crate::token::{Token};

use super::tokenizer::get_character_with_peek;


pub fn parse_operator<R>( cursor: &mut Cursor<R>, start: Position ) -> Result<Token, CastleError> 
where R: Read {
    
    let ch = get_character_with_peek(cursor, start)?;

    cursor.next_char()?; // consume operator

    return Ok(Token::operator_as_str_to_token(&ch, start, cursor.pos())?)
}

