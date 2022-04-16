use std::io::Read;

use input_cursor::{Position, Span};
use shared::castle_error::CastleError;

use crate::{ast::{keyword::Keyword}, token::{Token, token::TokenKind}};

use super::{ tokenizer::Tokenizer};

pub fn get_keyword_or_return_none<R>(tokenizer: &mut Tokenizer<R>, word: &String, start: Position) 
-> Result<Option<Token>, CastleError> where R: Read {
    let option_keyword = Keyword::from_str_to_option_keyword(&word[..]);
    return match option_keyword {
        // return keyword
        Some(keyword) => Ok(Some(Token::new(TokenKind::Keyword(keyword), Span::new(start, tokenizer.cursor.pos())))),
        None => Ok(None)
    }
}