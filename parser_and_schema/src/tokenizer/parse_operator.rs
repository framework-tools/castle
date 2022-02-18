use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::castle_error::CastleError;

use crate::token::{Token, token::TokenKind};

use super::{tokenizer::get_character_with_peek, parse_identifier_type_or_keyword::get_next_char_and_unwrap};


pub fn parse_operator<R>( cursor: &mut Cursor<R>, start: Position ) -> Result<Token, CastleError> 
where R: Read {
    let ch = get_next_char_and_unwrap(cursor)?; // consume operator
    let peeked_ch = cursor.peek_char()?;
    if peeked_ch.is_some() {
        let peeked_ch = get_character_with_peek(cursor, start)?;
        if ch == '/' && peeked_ch == '/' {
            return parse_comment(cursor, start);
        }
    }
    return Ok(Token::operator_as_str_to_token(&ch, start, cursor.pos())?)
}


fn parse_comment<R>( cursor: &mut Cursor<R>, start: Position ) -> Result<Token, CastleError> 
where R: Read {
    get_next_char_and_unwrap(cursor)?; // consume second '/'
    let mut comment = String::new();
    loop {
        
        let ch = get_next_char_and_unwrap(cursor)?;
        if ch == '\n' { break; }
        comment.push(ch);
    }
    return Ok(Token::new(TokenKind::LineTerminator, Span::new(start, cursor.pos())));
}