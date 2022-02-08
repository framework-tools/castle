use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::CastleError;

use crate::{ast::syntax_definitions::argument::Argument, token::{Token, token::TokenKind}, parser::schema_parser::types::primitive_type::PrimitiveType};

use super::{parse_identifier::parse_identifier_token, tokenizer::Tokenizer};

pub fn get_primitive_type_or_continue<R>(tokenizer: &mut Tokenizer<R>, word: String, start: Position)
-> Result<Token, CastleError> where R: Read {
    let primitive_type = PrimitiveType::from_str_to_option_primitive_type(&word[..]);
    match primitive_type {
        Some(primitive_type) => Ok(Token::new(TokenKind::PrimitiveType(primitive_type), Span::new(start, tokenizer.cursor.pos()))),
        None => parse_identifier_token(tokenizer, word, start, false)
    }
}