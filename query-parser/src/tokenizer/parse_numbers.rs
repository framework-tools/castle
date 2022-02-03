use std::io::Read;

use input_cursor::{Position, Cursor, Span};
use shared::CastleError;

use crate::token::{Token, token::{TokenKind, Numeric}};

1234.5


/// peek next character
/// if character is digit or "." push to num_as_string
/// next character 
/// else break
/// convert string to number
/// create token and return
pub fn parse_number<R>(cursor: &mut Cursor<R>, start: Position) -> Result<Token, CastleError> 
where R: Read {
    let num_as_string = String::new();
    loop {
        let ch = get_character_with_peek(cursor, start);
        if ch.is_digit(10) || ch == '.' || ch == '-' {
            num_as_string.push(ch);
            cursor.next_char()?;
        } 
        else { break; }
    }
    let number =  convert_num_as_string_to_token(num_as_string, start, cursor);
    return number
}

fn convert_num_as_string_to_token<R>(num_as_string: String,start: Position, cursor: &mut Cursor<R>) -> Result<Token, CastleError> 
where R: Read {
    if num_as_string.contains('.') { return handle_float(&num_as_string, start, cursor); } 
    else if num_as_string.contains('-') { return handle_integer(&num_as_string, start, cursor); } 
    else { return handle_unsigned_integer(&num_as_string, start, cursor) }
}

fn handle_float<R>(num_as_string: &str, start: Position, cursor: &mut Cursor<R>) -> Result<Token, CastleError> where R: Read {
    let float = num_as_string.parse::<f64>();
    return match float {
        Ok(float) => Ok(Token::new(TokenKind::NumericLiteral(Numeric::Float(float)), Span::new(start, cursor.pos()))),
        Err(_) => Err(CastleError::lex("error processing number", start))
    }
}

fn handle_integer<R>(num_as_string: &str, start: Position, cursor: &mut Cursor<R>) -> Result<Token, CastleError> where R: Read {
    let int = num_as_string.parse::<i64>();
    return match int {
        Ok(int) => Ok(Token::new(TokenKind::NumericLiteral(Numeric::Integer(int)), Span::new(start, cursor.pos()))),
        Err(_) => Err(CastleError::lex("error processing number", start))
    }
}

fn handle_unsigned_integer<R>(num_as_string: &str, start: Position, cursor: &mut Cursor<R>) -> Result<Token, CastleError> where R: Read {
    let int = num_as_string.parse::<u64>();
    return match int {
        Ok(int) => Ok(Token::new(TokenKind::NumericLiteral(Numeric::UnsignedInteger(int)), Span::new(start, cursor.pos()))),
        Err(_) => Err(CastleError::lex("error processing number", start))
    }
}