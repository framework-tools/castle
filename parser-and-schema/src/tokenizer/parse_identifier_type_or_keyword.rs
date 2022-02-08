use std::{io::Read};

use input_cursor::{Cursor, Position};
use shared::CastleError;

use crate::{token::{Token}};

use super::{parse_keyword::get_keyword_or_continue, parse_arguments::get_arguments, parse_vec_type::get_vec_type_from_word, tokenizer::Tokenizer, parse_option_type::get_option_type_from_word, parse_enum_value::parse_enum_value, parse_identifier::parse_identifier_token};

pub fn parse_identifier_or_keyword_or_type<R>(tokenizer: &mut Tokenizer<R>, start: Position) -> Result<Token, CastleError> 
where R: Read {
    let (word, field_has_arguments) = get_word_from_chars(&mut tokenizer.cursor)?;

    if word == "Vec" { return get_vec_type_from_word(&mut tokenizer.cursor, word, start) }
    else if word == "Option" { return get_option_type_from_word(&mut tokenizer.cursor, word, start) }
    else if word.contains("::") { return parse_enum_value(tokenizer, word, start)}

    if field_has_arguments { return parse_identifier_token(tokenizer, word, start, true) }

    // get keyword or continue will check every case of word
    // and will return a keyword, type, or identifier token
    let token = get_keyword_or_continue(tokenizer, word, start);
    return token
}

fn get_word_from_chars<R>(cursor: &mut Cursor<R>) -> Result<(String, bool), CastleError> where R: Read {
    let mut word = String::new();
    loop {
        let c = cursor.peek_char();
        match c {
            Ok(c) => {
                match c {
                    Some(c) => if let Ok(ch) = char::try_from(c) {
                        if ch == '(' { //start of arguments
                            cursor.next_char()?;
                            return Ok((word, true))
                            
                        } else if ch.is_ascii_alphanumeric() || ch == '_' {
                            word.push(ch);
                            cursor.next_char()?;
                        } 
                        //check for enum_value - if first letter uppercase & colon we can assume it's an enum value
                        else if ch == ':' && word.chars().nth(0).unwrap().is_uppercase() {
                            cursor.next_char()?; // skip first colon
                            let c = cursor.next_char(); //parse second colon
                            match c {
                                Ok(c) => {
                                    match c {
                                        Some(c) => if let Ok(ch) = char::try_from(c) {
                                            if ch == ':' { //keep pushing for enum_value
                                                word.push(':');
                                                word.push(':');
                                            } else {
                                                return Err(CastleError::Lexer("wrong syntax while parsing word from chars".into(), cursor.pos()))
                                            }
                                        },
                                        None => return Err(CastleError::AbruptEOF("parsing word from chars".into()))
                                    }
                                },
                                Err(e) => return Err(CastleError::AbruptEOF("get word from chars".into()))
                            }; 

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
    return Ok((word, false))
}
