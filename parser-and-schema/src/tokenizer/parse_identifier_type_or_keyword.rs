use std::{io::Read};

use input_cursor::{Cursor, Position};
use shared::CastleError;

use crate::{token::{Token}};

use super::{parse_keyword::get_keyword_or_continue, parse_arguments::get_arguments, parse_vec_type::get_vec_type_from_word, tokenizer::Tokenizer};

pub fn parse_identifier_or_keyword_or_type<R>(tokenizer: &mut Tokenizer<R>, start: Position) -> Result<Token, CastleError> 
where R: Read {
    let (word, field_has_arguments) = get_word_from_chars(&mut tokenizer.cursor)?;

    if word == "Vec" { return get_vec_type_from_word(&mut tokenizer.cursor, word, start) }
    let arguments;
    if field_has_arguments { arguments = Some(get_arguments(tokenizer)?); } // this also is used for tuples on enums
    else { arguments = None }
    // get keyword or continue will check every case of word
    // and will return a keyword, type, or identifier token
    let token = get_keyword_or_continue(&mut tokenizer.cursor, word, start, arguments);
    return token
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
                            cursor.next_char()?;
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
