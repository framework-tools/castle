use std::io::Read;

use castle_error::CastleError;
use castle_input_cursor::{Cursor, Position, Span};

use crate::{Punctuator, Token, TokenKind};

pub fn parse_operator(cursor: &mut Cursor<impl Read>, start_pos: Position ) -> Result<Token, CastleError> {
    let ch = if let Some(ch) = cursor.next_byte()? {
        ch
    } else {
        return Err(CastleError::syntax("unexpected end of file", cursor.pos()));
    };

    let punc = match ch {
        // Equality & Logic
        b'=' => Punctuator::Default,
        b'|' => Punctuator::Or,

        // Generics
        b'<' => Punctuator::GenericOpen,
        b'>' => Punctuator::GenericClose,

        // Symbols, Brackets, Parenthesis, Blocks, etc
        b'{' => Punctuator::OpenBlock,
        b'}' => Punctuator::CloseBlock,
        b'(' => Punctuator::OpenParen,
        b')' => Punctuator::CloseParen,
        b'[' => Punctuator::OpenBracket,
        b']' => Punctuator::CloseBracket,
        b',' => Punctuator::Comma,
        b'@' => Punctuator::At,
        b';' => Punctuator::SemiColon,
        b'.' => Punctuator::Dot,
        b':' => {
            let next = cursor.peek()?.ok_or(CastleError::syntax(
                "unexpected end of file",
                start_pos,
            ))?;

            if next == b':' {
                cursor.next_byte()?;
                Punctuator::DoubleColon
            } else {
                Punctuator::Colon
            }
        }

        // dots are handled by spread literal
        op => Err(CastleError::syntax(format!("unexpected operator: {}", op as char), start_pos))?,
    };

    Ok(Token::new(TokenKind::Punctuator(punc), Span::new(start_pos, cursor.pos())))
}