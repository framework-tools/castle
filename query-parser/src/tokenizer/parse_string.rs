use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::CastleError;

use crate::token::{Token, token::TokenKind};

pub fn parse_string<R>( cursor: &mut Cursor<R>, start: Position ) -> Result<Token, CastleError> 
where R: Read {
    cursor.next_char(); // skip the first quote
    let mut string = String::new();
    loop {
        let c = cursor.next_char()?.ok_or(CastleError::AbruptEOF)?;
        let ch = char::try_from(c).ok().ok_or(CastleError::lex(
            "invalid character",
            cursor.pos(),
        ))?;

        // handle escape character \ (backslash)
        if ch == '\\' { string = handle_escape_characters(cursor, string)?; } 
        else if ch == '"' { break; }
        else { string.push(ch);}
    };
    return Ok(Token::new(TokenKind::StringLiteral(string.into_boxed_str()), Span::new(start, cursor.pos())))
}


fn handle_escape_characters<R>(cursor: &mut Cursor<R>, string: String) -> Result<String, CastleError> 
where R: Read {
// list of escape characters: (based on JSON)
    // \b	Backspace (ascii 8)
    // \f	Form feed (ascii 12)
    // \n	New line
    // \r	Carriage return
    // \t	Horizontal tab
    // \uXXXX	Character with 16 bit hex value XXXX
    // \\     Backslash
    // \/     Forward slash
    // \"     Double quote

    let c = cursor.next_char()?.ok_or(CastleError::AbruptEOF)?;

    let ch = char::try_from(c).ok().ok_or(CastleError::lex(
        "invalid character",
        cursor.pos(),
    ))?;
    let x = 'b';
    match ch {
        'b' => string.push('\u{0008}'),
        'f' => string.push('\u{000C}'),
        'n' => string.push('\n'),
        'r' => string.push('\r'),
        't' => string.push('\t'),
        'u' => {
            let mut hex_string = String::new();
            for _ in 0..4 {
                let c = cursor.next_char()?.ok_or(CastleError::AbruptEOF)?;
                let ch = char::try_from(c).ok().ok_or(CastleError::lex(
                    "invalid character",
                    cursor.pos(),
                ))?;

                if ch.is_ascii_hexdigit() {
                    hex_string.push(ch);
                } else {
                    return Err(CastleError::lex(
                        "Invalid hexadecimal escape sequence: missing hexadecimal value",
                        cursor.pos(),
                    ));
                }
            }
            let hex_value = u32::from_str_radix(&hex_string, 16).unwrap();
            string.push(std::char::from_u32(hex_value).unwrap());
        }
        '\\' => string.push('\\'),
        '/' => string.push('/'),
        '"' => string.push('"'),
        _ => {
            return Err(CastleError::lex(
                format!("Invalid escape sequence: {}", c),
                cursor.pos(),
            ));
        }
    };
    return Ok(string)
}