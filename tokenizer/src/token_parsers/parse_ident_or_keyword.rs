use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::{castle_error::CastleError, Primitive};

use crate::{Token, TokenKind, Keyword};


pub(crate) fn parse_ident_or_keyword(cursor: &mut Cursor<impl Read>, start_pos: Position) -> Result<Token, CastleError> {
    let mut identifier_name = String::new();

    loop {
        let c = cursor.peek_char()?.ok_or(CastleError::syntax(
            "Unexpected end of input while parsing identifier",
            cursor.pos(),
        ))?;

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