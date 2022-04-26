use std::io::Read;

use castle_error::CastleError;
use input_cursor::{Cursor, Position, Span};


use crate::{Token, TokenKind, Keyword, Primitive};


pub(crate) fn parse_ident_or_keyword(cursor: &mut Cursor<impl Read>, start_pos: Position) -> Result<Token, CastleError> {
    let mut identifier_name = String::new();

    loop {
        match cursor.peek_char()? {
            Some(c) => {
                let ch = char::try_from(c);

                if let Ok(ch) = ch {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        cursor.next_char()?;
                        identifier_name.push(ch);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            },
            None => break,
        }
    }

    let token_kind = if let Ok(keyword) = identifier_name.parse() {
        match keyword {
            Keyword::True => TokenKind::Primitive(Primitive::Boolean(true)),
            Keyword::False => TokenKind::Primitive(Primitive::Boolean(false)),
            _ => TokenKind::Keyword(keyword),
        }
    } else {
        TokenKind::Identifier(identifier_name.into_boxed_str())
    };

    Ok(Token::new(token_kind, Span::new(start_pos, cursor.pos())))
}