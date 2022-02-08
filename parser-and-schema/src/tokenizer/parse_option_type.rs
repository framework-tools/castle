use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::CastleError;

use crate::{token::{Token, token::TokenKind}, parser::schema_parser::types::{vec_type::VecType, option_type::OptionType}};

pub fn get_option_type_from_word<R>(cursor: &mut Cursor<R>, word: String, start: Position) -> Result<Token, CastleError>
where R: Read {
    let mut word = word;
    loop {
        let char = cursor.next_char()?.unwrap();
        let char = char::try_from(char).ok().ok_or(CastleError::lex("invalid character",cursor.pos()))?;
        if char == '>' { word.push(char); break; } 
        else { word.push(char); }
    }
    let option_type = OptionType::new(&word)?;
    match option_type {
        Some(type_) => return Ok(Token::new(TokenKind::OptionType(OptionType::get_option_type_struct(type_)), Span::new(start, cursor.pos()))),
        None => return Err(CastleError::AbruptEOF("Error found in 'get_option_type_from_word'".into()))
    }
}