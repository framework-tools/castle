use std::io::Read;

use input_cursor::{Position, Span};
use shared::castle_error::CastleError;

use crate::{token::{Token, token::TokenKind}, parsers::schema_parser::types::primitive_type::PrimitiveType};

use super::{tokenizer::Tokenizer};

pub fn get_primitive_type_or_return_none<R>(tokenizer: &mut Tokenizer<R>, word: &String, start: Position)
-> Result<Option<Token>, CastleError> where R: Read {
    let primitive_type = PrimitiveType::from_str_to_option_primitive_type(&word[..]);
    match primitive_type {
        Some(primitive_type) => Ok(Some(Token::new(TokenKind::PrimitiveType(primitive_type), Span::new(start, tokenizer.cursor.pos())))),
        None => Ok(None)
    }
}