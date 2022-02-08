use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::CastleError;

use crate::{token::{Token, token::TokenKind}, parser::schema_parser::types::{option_type::OptionType, type_system::Type}};

pub fn get_option_type_from_word<R>(cursor: &mut Cursor<R>, word: String, start: Position) -> Result<Token, CastleError>
where R: Read {
    let mut option_as_string = word;
    loop {
        let end_of_option = add_next_char_to_option_type(cursor, &mut option_as_string)?;
        if end_of_option { break; }
    }
    let option_type = OptionType::new(&option_as_string)?;
    return create_token_from_option_type(option_type, start, cursor.pos());
}

fn add_next_char_to_option_type<R>(cursor: &mut Cursor<R>, option_as_string: &mut String) -> Result<bool, CastleError>
where R: Read {
    let char = cursor.next_char()?.unwrap();
    let char = char::try_from(char).ok().ok_or(CastleError::lex("invalid character",cursor.pos()))?;

    option_as_string.push(char); 
    if char == '>' {  return Ok(true) }
    else { return Ok(false) }
}

fn create_token_from_option_type(option_type: Option<Type>, start: Position, end: Position) -> Result<Token, CastleError> {
    match option_type {
        Some(type_) => return Ok(Token::new(TokenKind::OptionType(OptionType::get_option_type_struct(type_)), Span::new(start, end))),
        None => return Err(CastleError::AbruptEOF("Error found in 'get_option_type_from_word'".into()))
    }
}
