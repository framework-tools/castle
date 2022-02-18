use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::castle_error::CastleError;

use crate::{token::{Token, token::TokenKind}, parsers::schema_parser::types::{vec_type::VecType, type_system::Type}};

use super::parse_identifier_type_or_keyword::get_next_char_and_unwrap;

pub fn get_vec_type_from_word<R>(cursor: &mut Cursor<R>, word: String, start: Position) -> Result<Token, CastleError>
where R: Read {
    let mut vec_as_string = word;
    let mut i = 0;
    let mut inner_special_type_count = 0;
    loop {
        let c = get_next_char_and_unwrap(cursor)?;
        vec_as_string.push(c);
        if inner_special_type_count == 0 && c == '>' { break; }
        // To cover special inner typesVec<Vec<Vec<Type>>>
        if c == '<' && i != 0 { inner_special_type_count += 1; }
        else if c == '>' { inner_special_type_count -= 1; }
        i += 1;
    }

    let vec_type = VecType::new(&vec_as_string);
    match vec_type {
        Type::VecType(vec_type) => return Ok(Token::new(TokenKind::VecType(vec_type), Span::new(start, cursor.pos()))),
        _ => return Err(CastleError::lex("invalid vec type", cursor.pos()))
    }
}