use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::castle_error::CastleError;

use crate::{token::{Token, token::TokenKind}, parsers::schema_parser::types::{option_type::OptionType, type_system::Type}};

use super::parse_identifier_type_or_keyword::get_next_char_and_unwrap;

pub fn get_option_type_from_word<R>(cursor: &mut Cursor<R>, word: String, start: Position) -> Result<Token, CastleError>
where R: Read {
    let mut option_as_string = word;
    let mut i = 0;
    let mut inner_special_type_count = 0;
    loop {
        let c = get_next_char_and_unwrap(cursor)?;
        option_as_string.push(c);
        if inner_special_type_count == 0 && c == '>' { break; }
        // To cover special inner typesVec<Vec<Vec<Type>>>
        if c == '<' && i != 0 { inner_special_type_count += 1; }
        else if c == '>' { inner_special_type_count -= 1; }
        i += 1;
    }

    let option_type = OptionType::new(&option_as_string);
    return create_token_from_option_type(Some(option_type), start, cursor.pos());
}

fn create_token_from_option_type(option_type: Option<Type>, start: Position, end: Position) -> Result<Token, CastleError> {
    match option_type {
        Some(type_) => return Ok(Token::new(TokenKind::OptionType(OptionType::get_option_type_struct(type_)), Span::new(start, end))),
        None => return Err(CastleError::AbruptEOF("Error found in 'get_option_type_from_word'".into()))
    }
}
