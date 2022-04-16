use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::{castle_error::CastleError, Primitive};

use crate::{TokenKind, Token};

pub fn parse_string(cursor: &mut Cursor<impl Read>, start_pos: Position) -> Result<Token, CastleError> {
    let mut string = String::new();
    // consume the "
    cursor.next_char()?;

    loop {
        let c = cursor
            .next_char()?
            .ok_or(CastleError::syntax("unexpected end of file", cursor.pos()))?;

        let ch = char::try_from(c)
            .ok()
            .ok_or(CastleError::syntax("invalid character", cursor.pos()))?;

        // handle escape character \ (backslash)
        if ch == '\\' {
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

            let c = cursor
                .next_char()?
                .ok_or(CastleError::syntax("unexpected end of file", cursor.pos()))?;

            let ch = char::try_from(c)
                .ok()
                .ok_or(CastleError::syntax("invalid character", cursor.pos()))?;

            match ch {
                'b' => string.push('\u{0008}'),
                'f' => string.push('\u{000C}'),
                'n' => string.push('\n'),
                'r' => string.push('\r'),
                't' => string.push('\t'),
                'u' => {
                    let mut hex_string = String::new();
                    for _ in 0..4 {
                        let c = cursor
                            .next_char()?
                            .ok_or(CastleError::syntax("unexpected end of file", cursor.pos()))?;

                        let ch = char::try_from(c)
                            .ok()
                            .ok_or(CastleError::syntax("invalid character", cursor.pos()))?;

                        if ch.is_ascii_hexdigit() {
                            hex_string.push(ch);
                        } else {
                            return Err(CastleError::syntax(
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
                    return Err(CastleError::syntax(
                        format!("Invalid escape sequence: {}", c),
                        cursor.pos(),
                    ));
                }
            }
        } else if ch == '"' {
            break;
        } else {
            string.push(ch);
        }
    }

    Ok(Token::new(
        TokenKind::Primitive(Primitive::String(string.into_boxed_str())),
        Span::new(start_pos, cursor.pos()),
    ))
}
