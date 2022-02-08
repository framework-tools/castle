use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::CastleError;

use crate::{token::{Token, token::TokenKind}, parser::schema_parser::types::vec_type::VecType};

pub fn get_vec_type_from_word<R>(cursor: &mut Cursor<R>, word: String, start: Position) -> Result<Token, CastleError>
where R: Read {
    let mut word = word;
    loop {
        let char = cursor.next_char()?.unwrap();
        let char = char::try_from(char).ok().ok_or(CastleError::lex("invalid character",cursor.pos()))?;
        println!("char: {:?}", char);
        word.push(char);
        if char == '>' { break; } 
    }
    let vec_type = VecType::new(&word);
    match vec_type {
        Some(type_) => return Ok(Token::new(TokenKind::VecType(VecType::get_vec_type_struct(type_)), Span::new(start, cursor.pos()))),
        None => return Err(CastleError::AbruptEOF("Error found in 'get_vec_type_from_word'".into()))
    }
}