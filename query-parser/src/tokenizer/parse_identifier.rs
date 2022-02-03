use std::io::Read;

use input_cursor::{Cursor, Position, Span};
use shared::CastleError;

use crate::{token::{Token, token::TokenKind}, ast::syntax_definitions::keyword::Keyword};


pub fn parse_identifier<R>(cursor: &mut Cursor<R>, start: Position) -> Result<Token, CastleError> 
where R: Read {
    let mut identifier_name = get_identifier_from_chars(cursor)?;

    let keyword = Keyword::from(&identifier_name[..]);
    let token_kind = TokenKind::from(keyword);
    return Ok(Token::new(token_kind, Span::new(start, cursor.pos())))
}

fn get_identifier_from_chars<R>(cursor: &mut Cursor<R>) -> Result<String, CastleError> where R: Read {
    let mut identifier_name = String::new();
    loop {
        let c = cursor.peek_char()?.ok_or(CastleError::AbruptEOF)?;
        if let Ok(ch) = char::try_from(c) {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                cursor.next_char()?;
                identifier_name.push(ch);
            } else {
                break;
            }
        }
    }
    return Ok(identifier_name);
}