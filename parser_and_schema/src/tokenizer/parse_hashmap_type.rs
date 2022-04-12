use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::castle_error::CastleError;

use crate::{parsers::schema_parser::types::type_system::{get_type_from_string}, token::{Token, token::TokenKind}};

use super::parse_identifier_type_or_keyword::get_next_char_and_unwrap;

pub fn get_hashmap_type_from_word <R: Read>(cursor: &mut Cursor<R>, word: String, start: Position) -> Result<Token, CastleError>{
    let mut hashmap_as_str = word;
    let mut i = 0;
    let mut inner_special_type_count = 0;
    loop {
        let c = get_next_char_and_unwrap(cursor)?;
        hashmap_as_str.push(c);
        if inner_special_type_count == 0 && c == '>' { break; }
        // To cover special inner typesVec<Vec<Vec<Type>>>
        if c == '<' && i != 0 { inner_special_type_count += 1; }
        else if c == '>' { inner_special_type_count -= 1; }
        i += 1;
    }

    let inner_type_as_str = hashmap_as_str[8..hashmap_as_str.len() - 1].to_string();
    let inner_type= get_type_from_string(&inner_type_as_str);
    let token = Token::new(TokenKind::HashMapType(inner_type.into()), Span::new(start, cursor.pos()));
    return Ok(token)
}