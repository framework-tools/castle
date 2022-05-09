use std::io::Read;

use castle_error::CastleError;
use castle_input_cursor::{Position, Cursor, Span};


use crate::{Token, TokenKind, Primitive};

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum NumericKind {
    UnsignedInteger,
    Float,
    Integer
}

pub fn parse_number(cursor: &mut Cursor<impl Read>, start_pos: Position) -> Result<Token, CastleError> {
    let ch = if let Some(ch) = cursor.next_char()? {
        ch as u8
    } else {
        return Err(CastleError::syntax("unexpected end of file", cursor.pos()));
    };

    let mut kind = match ch {
        b'-' => NumericKind::Integer,
        b'0'..=b'9' => NumericKind::UnsignedInteger,
        _ => return Err(CastleError::syntax("unexpected numeric character", cursor.pos()))
    };

    let mut buf = vec![ch];

    loop {
        let c = cursor.peek()?;

        if let Some(ch) = c {
            match ch {
                b'0'..=b'9' => {
                    buf.push(ch);
                    cursor.next_byte()?;
                },
                b'_' => {
                    cursor.next_byte()?; // ignore underscore eg: 1_000
                },
                b'.' => {
                    if kind == NumericKind::Integer || kind == NumericKind::UnsignedInteger {
                        kind = NumericKind::Float;
                        buf.push(ch);
                        cursor.next_byte()?;
                    } else {
                        break // could be a field access
                    }
                },
                _ => {
                    break;
                }
            }
        }
    }

    let num_str = unsafe { std::str::from_utf8_unchecked(buf.as_slice()) };

    let num = match kind {
        NumericKind::Float => {
            let val: f64 = fast_float::parse(num_str).expect("Failed to parse float after checks");
            let int_val = val as i64;

            if (int_val as f64) == val {
                Primitive::Number(int_val.into())
            } else {
                Primitive::Number(val.into())
            }
        },
        NumericKind::UnsignedInteger => {
            Primitive::Number(u64::from_str_radix(num_str, 10).expect("Failed to parse unsigned integer").into())
        }
        NumericKind::Integer => {
            Primitive::Number(i64::from_str_radix(num_str, 10).expect("Failed to parse integer").into())
        }
    };

    Ok(Token::new(
        TokenKind::Primitive(num),
        Span::new(start_pos, cursor.pos())
    ))
}
