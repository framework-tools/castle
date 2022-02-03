use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::CastleError;

use crate::{token::{Token, token::TokenKind}, ast::syntax_definitions::keyword::Keyword};

pub fn parse_identifier_or_keyword<R>(cursor: &mut Cursor<R>, start: Position) -> Result<Token, CastleError> 
where R: Read {
    let mut word = get_word_from_chars(cursor)?;
    let option_keyword = Keyword::from_str_to_option_keyword(&word[..]);
    return match option_keyword {
        Some(keyword) => Ok(Token::new(TokenKind::Keyword(keyword), Span::new(start, cursor.pos()))),
        None => Ok(Token::new(TokenKind::Identifier(word.into()), Span::new(start, cursor.pos())))
    }
}

fn get_word_from_chars<R>(cursor: &mut Cursor<R>) -> Result<String, CastleError> where R: Read {
    let mut identifier_name = String::new();
    loop {
        let c = cursor.peek_char();
        match c {
            Ok(c) => {
                match c {
                    Some(c) => if let Ok(ch) = char::try_from(c) {
                        if ch.is_ascii_alphanumeric() || ch == '_' {
                            identifier_name.push(ch);
                            cursor.next_char()?;
                        } else {
                            break;
                        }
                    }
                    None => break,
                };
            }
            Err(_) => break            
        };
    }
    return Ok(identifier_name);
}