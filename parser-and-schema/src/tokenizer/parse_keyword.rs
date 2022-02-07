use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::CastleError;

use crate::{ast::syntax_definitions::{argument::Argument, keyword::Keyword}, token::{Token, token::TokenKind}};

use super::parse_primitive_type::get_primitive_type_or_continue;

pub fn get_keyword_or_continue<R>(cursor: &mut Cursor<R>, word: String, start: Position, arguments: Option<Vec<Argument>>) 
-> Result<Token, CastleError> where R: Read {
    let option_keyword = Keyword::from_str_to_option_keyword(&word[..]);
    return match option_keyword {
        // return keyword
        Some(keyword) => Ok(Token::new(TokenKind::Keyword(keyword), Span::new(start, cursor.pos()))),
        None => get_primitive_type_or_continue(cursor, word, start, arguments) // if not keyword, continue
    }
}