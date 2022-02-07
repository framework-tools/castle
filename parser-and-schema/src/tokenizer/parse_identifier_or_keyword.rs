use std::{io::Read};

use input_cursor::{Cursor, Position, Span};
use shared::CastleError;

use crate::{token::{Token, token::{TokenKind, Identifier, VecType}}, ast::syntax_definitions::{keyword::Keyword, expressions::{PrimitiveValue}, want::Argument}, parser::schema_parser::types::{schema_field::{PrimitiveType}}};

use super::tokenizer::advance_and_parse_token;

pub fn parse_identifier_or_keyword_or_type<R>(cursor: &mut Cursor<R>, start: Position) -> Result<Token, CastleError> 
where R: Read {
    let (word, field_has_arguments) = get_word_from_chars(cursor)?;
    let arguments;
    if field_has_arguments { arguments = Some(get_arguments(cursor)?); } // this also is used for tuples on enums
    else { arguments = None }
    // get keyword or continue will check every case of word
    // and will return a keyword, type, or identifier token
    let token = get_keyword_or_continue(cursor, word, start, arguments);
    return token
}

fn get_keyword_or_continue<R>(cursor: &mut Cursor<R>, word: String, start: Position, arguments: Option<Vec<Argument>>) -> Result<Token, CastleError>
where R: Read {
    let option_keyword = Keyword::from_str_to_option_keyword(&word[..]);
    return match option_keyword {
        Some(keyword) => Ok(Token::new(TokenKind::Keyword(keyword), Span::new(start, cursor.pos()))), // return keyword
        None => get_primitive_type_or_continue(cursor, word, start, arguments) // if not keyword, 
    }
}

fn get_primitive_type_or_continue<R>(cursor: &mut Cursor<R>, word: String, start: Position, arguments: Option<Vec<Argument>>) -> Result<Token, CastleError>
where R: Read {
    let primitive_type = PrimitiveType::from_str_to_option_primitive_type(&word[..]);
    match primitive_type {
        Some(primitive_type) => Ok(Token::new(TokenKind::PrimitiveType(primitive_type), Span::new(start, cursor.pos()))),
        None => get_vec_type_or_identifier(cursor, word, start, arguments)
    }
}

fn get_vec_type_or_identifier<R>(cursor: &mut Cursor<R>, word: String, start: Position, arguments: Option<Vec<Argument>>) -> Result<Token, CastleError>
where R: Read {
    let ch = cursor.peek()?;
    match ch {
        Some(ch) => {
            let char = char::try_from(ch).ok().ok_or(CastleError::lex("invalid character",cursor.pos()))?;
            if char == '<' {
                get_vec_type_from_word(cursor, word, start)
            } else {
                return Ok(Token::new(TokenKind::Identifier(Identifier {
                    name: word.into(),
                    arguments
                    }), Span::new(start, cursor.pos())))
            }
        },
        None => return Ok(Token::new(TokenKind::Identifier(Identifier {
            name: word.into(),
            arguments
            }), Span::new(start, cursor.pos())))
    }
}
fn get_vec_type_from_word<R>(cursor: &mut Cursor<R>, word: String, start: Position) -> Result<Token, CastleError>
where R: Read {
    let mut word = word;
    loop {
        let char = cursor.next_char()?.unwrap();
        let char = char::try_from(char).ok().ok_or(CastleError::lex("invalid character",cursor.pos()))?;
        if char == '>' { word.push(char); break; } 
        else { word.push(char); }
    }
    let vec_type = VecType::new(&word);
    match vec_type {
        Some(type_) => return Ok(Token::new(TokenKind::VecType(VecType::get_vec_type_struct(type_)), Span::new(start, cursor.pos()))),
        None => return Err(CastleError::AbruptEOF)
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

/// Takes in Cursor returns arguments token
///  - The '(' is already consumed
///  - if ')' return token
///  - else if, ',' create token from argument, then push token to arguments
///  - else push character to current argument
pub fn get_arguments<R>(cursor: &mut Cursor<R> ) -> Result<Vec<Argument>, CastleError> 
where R: Read {
    let mut arguments = Vec::new();
    let err ;
    loop {
        let c = cursor.peek()?;
        match c {
            Some(ch) => {
                let ch = char::try_from(ch).ok().ok_or(CastleError::lex("invalid character", cursor.pos()))?;
                if ch == ')' {
                    cursor.next_char()?; // skip close paren
                    return Ok(arguments)
                } 
                else if ch == ','{
                    cursor.next_char()?; // skip comma
                    let token = advance_and_parse_token(cursor)?;
                    match token {
                        Some(token) => {
                            let argument = Argument::new(token)?;
                            arguments.push(argument)
                        },
                        None => return Err(CastleError::AbruptEOF)
                    };
                }
                else if ch == ' ' || ch == '\n'{ 
                    cursor.next_char()?;
                }
                else {
                    let token = advance_and_parse_token(cursor)?;
                    match token {
                        Some(token) => {
                            let argument = Argument::new(token)?;
                            arguments.push(argument)
                        },
                        None => return Err(CastleError::AbruptEOF)
                    };
                }
            }
            None => { err = Some(Err(CastleError::AbruptEOF)); break; }
        }
    }
    return match err {
        Some(err) => err,
        None => Ok(arguments)
    } 
}

fn parse_primitive_value(value: String) -> Result<PrimitiveValue, CastleError> {
    if value.contains('"') {
        return Ok(PrimitiveValue::String(value.into()))
    }
    else if value == "true" {
        return Ok(PrimitiveValue::Boolean(true))
    }
    else if value == "false" {
        return Ok(PrimitiveValue::Boolean(false))
    }
    else if value.contains('.') {
        return Ok(PrimitiveValue::Float(value.parse().unwrap()))
    }
    else if value.contains('-'){
        return Ok(PrimitiveValue::Int(value.parse().unwrap()))
    }
    else {
        return Ok(PrimitiveValue::UInt(value.parse().unwrap()))
    }
}

