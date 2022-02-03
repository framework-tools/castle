use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::CastleError;

use crate::{token::{Token, token::TokenKind}, ast::syntax_definitions::{keyword::Keyword, expressions::PrimitiveValue}};

pub fn parse_identifier_or_keyword<R>(cursor: &mut Cursor<R>, start: Position) -> Result<Token, CastleError> 
where R: Read {
    let (word, field_has_arguments) = get_word_from_chars(cursor)?;
    let arguments;
    if field_has_arguments { arguments = get_field_arguments(cursor, start)?; }
    else { arguments = None }
    let option_keyword = Keyword::from_str_to_option_keyword(&word[..]);
    return match option_keyword {
        Some(keyword) => Ok(Token::new(TokenKind::Keyword(keyword), Span::new(start, cursor.pos()))),
        None => Ok(Token::new(TokenKind::Identifier(word.into()), Span::new(start, cursor.pos())))
    }
}

fn get_word_from_chars<R>(cursor: &mut Cursor<R>) -> Result<(String, bool), CastleError> where R: Read {
    let mut identifier_name = String::new();
    loop {
        let c = cursor.peek_char();
        match c {
            Ok(c) => {
                match c {
                    Some(c) => if let Ok(ch) = char::try_from(c) {
                        if ch == '(' { //start of arguments
                            return Ok((identifier_name, true))
                            
                        } else if ch.is_ascii_alphanumeric() || ch == '_' {
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
    return Ok((identifier_name, false))
}

//need to parse in tokenizer not cursor
pub fn get_field_arguments<R>(cursor: &mut Cursor<R>, start: Position) -> Result<Option<Vec<PrimitiveValue>>, CastleError> 
where R: Read {
    return Ok(None)
    // let mut arguments = Vec::new();
    // loop {
    //     let c = cursor.next_char()?;
    //     match c {
    //         Some(ch) => {
    //             if ch == ')' {
    //                 return Ok(Some(arguments))
    //             } else {
    //                 let token = tokeniser
    //                 //need to parse primitive values then push
    //             }
    //         }
    //         None => return Ok(Some(arguments)),
    //     }
    // }
}