use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::CastleError;

use crate::{token::{Token, token::{TokenKind, Identifier}}, ast::syntax_definitions::{keyword::Keyword, expressions::{PrimitiveValue}}, parser::schema_parser::types::{schema_field::{PrimitiveType}}};

pub fn parse_identifier_or_keyword_or_type<R>(cursor: &mut Cursor<R>, start: Position) -> Result<Token, CastleError> 
where R: Read {
    let (word, field_has_arguments) = get_word_from_chars(cursor)?;
    let arguments;
    if field_has_arguments { arguments = Some(get_field_arguments(cursor)?); }
    else { arguments = None }
    let option_keyword = Keyword::from_str_to_option_keyword(&word[..]);
    return match option_keyword {
        Some(keyword) => Ok(Token::new(TokenKind::Keyword(keyword), Span::new(start, cursor.pos()))),
        None => {
            let primitive_type = PrimitiveType::from_str_to_option_primitive_type(&word[..]);
            match primitive_type {
                Some(primitive_type) => Ok(Token::new(TokenKind::PrimitiveType(primitive_type), Span::new(start, cursor.pos()))),
                None => Ok(Token::new(TokenKind::Identifier(Identifier {
                    name: word.into(),
                    arguments
                    }), Span::new(start, cursor.pos())))
            }
        }
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
pub fn get_field_arguments<R>(cursor: &mut Cursor<R> ) -> Result<Vec<PrimitiveValue>, CastleError> 
where R: Read {
    let mut arguments = Vec::new();
    let mut argument_as_string = String::new();
    let mut err = None;
    loop {
        let c = cursor.next_char()?;
        match c {
            Some(ch) => {
                let ch = char::try_from(ch).ok().ok_or(CastleError::lex("invalid character", cursor.pos()))?;
                if ch == ')' {
                    let primitive_value = parse_primitive_value(argument_as_string.clone())?;
                    arguments.push(primitive_value.into());
                    return Ok(arguments)
                } 
                else if ch == ','{
                    let primitive_value = parse_primitive_value(argument_as_string.clone())?;
                    arguments.push(primitive_value.into());
                    argument_as_string.clear();
                }
                else if ch == ' ' || ch == '\n'{
                    //do nothing
                }
                else {
                    argument_as_string.push(ch);
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

