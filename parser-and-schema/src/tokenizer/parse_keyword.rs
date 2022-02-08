use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::CastleError;

use crate::{ast::syntax_definitions::{argument::Argument, keyword::Keyword}, token::{Token, token::TokenKind}};

use super::{parse_primitive_type::get_primitive_type_or_continue, tokenizer::Tokenizer};

pub fn get_keyword_or_continue<R>(tokenizer: &mut Tokenizer<R>, word: String, start: Position) 
-> Result<Token, CastleError> where R: Read {
    let option_keyword = Keyword::from_str_to_option_keyword(&word[..]);
    return match option_keyword {
        // return keyword
        Some(keyword) => Ok(Token::new(TokenKind::Keyword(keyword), Span::new(start, tokenizer.cursor.pos()))),
        None => get_primitive_type_or_continue(tokenizer, word, start) // if not keyword, continue
    }
}