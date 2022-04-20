use std::{collections::VecDeque, io::Read};

use castle_error::CastleError;
use input_cursor::{Cursor};

use crate::{
    token_parsers::{
        parse_ident_or_keyword::parse_ident_or_keyword, parse_newline::parse_newline,
        parse_numbers::parse_number, parse_operator::parse_operator, parse_string::parse_string,
    },
    Tokenizable, Token, TokenKind,
};

pub struct Tokenizer<R> {
    pub cursor: Cursor<R>,
    pub peeked: VecDeque<Token>,
}

impl<R: Read> Tokenizable for Tokenizer<R> {
    fn next(&mut self, skip_line_terminators: bool) -> Result<Option<Token>, CastleError> {
        let token = match self.peeked.pop_front() {
            Some(token) => Some(token),
            None => self.advance()?,
        };

        match token {
            Some(token) => {
                if token.kind == TokenKind::Comment {
                    self.next(skip_line_terminators)?;
                }
                if skip_line_terminators && token.kind == TokenKind::LineTerminator {
                    // this will consume all line terminators recursively
                    return self.next(skip_line_terminators);
                }
                Ok(Some(token))
            }
            None => Ok(None),
        }
    }

    fn peek_n(
        &mut self,
        skip_n: usize,
        skip_line_terminators: bool,
    ) -> Result<Option<&Token>, CastleError> {
        // if the number of tokens to skip is greater than or equal to the number of peeked tokens,
        // we need to read more tokens
        while skip_n >= self.peeked.len() {
            if let Some(token) = self.advance()? {
                if token.kind == TokenKind::Comment {
                    self.next(skip_line_terminators)?;
                }
                if skip_line_terminators && token.kind() == &TokenKind::LineTerminator {
                    continue;
                }

                self.peeked.push_back(token);

                // skip consecutive line terminators
                // this will add anything that is not a line terminator to the
                // peeked queue as well
                if self.peeked.back().unwrap().kind() == &TokenKind::LineTerminator {
                    while let Some(token) = self.advance()? {
                        if token.kind() == &TokenKind::LineTerminator {
                            continue;
                        } else {
                            self.peeked.push_back(token);
                            break;
                        }
                    }
                }
            } else {
                break;
            }
        }

        Ok(self.peeked.get(skip_n))
    }
}

impl<R> Tokenizer<R>
where
    R: Read,
{
    pub fn new(reader: R) -> Self
    where
        R: Read,
    {
        Self {
            cursor: Cursor::new(reader),
            peeked: VecDeque::new(),
        }
    }

    pub fn advance(&mut self) -> Result<Option<Token>, CastleError>
    where
        R: Read,
    {
        // skip whitespaces
        let (start, next_ch) = loop {
            let start = self.cursor.pos();
            if let Some(next_ch) = self.cursor.peek_char()? {
                // Ignore whitespace
                if !is_whitespace(next_ch) {
                    break (start, next_ch);
                }
                self.cursor.next_char()?;
            } else {
                return Ok(None);
            }
        };

        if let Ok(c) = char::try_from(next_ch) {
            let token = match c {
                '\r' | '\n' => parse_newline(&mut self.cursor, start)?,
                '"' => parse_string(&mut self.cursor, start)?,
                // Operator & Punctuator
                '=' | '<' | '>' | '*' | '/' | '%' | '&' | '|' | '^' | ':'
                | '{' | '}' | '[' | ']' | ',' | ';' | '@' | '#' | '(' | ')' =>  parse_operator(&mut self.cursor, start)?,
                '-' => parse_number(&mut self.cursor, start)?,
                _ if c.is_digit(10) => parse_number(&mut self.cursor, start)?,
                _ if c.is_ascii_alphabetic() => parse_ident_or_keyword(&mut self.cursor, start)?,
                _ => Err(CastleError::syntax(
                    format!(
                        "Unexpected '{}' at line {}, column {}",
                        c,
                        start.line_number(),
                        start.column_number()
                    ),
                    start,
                ))?
            };

            Ok(Some(token))
        } else {
            Ok(None) // EOF
        }
    }
}

fn is_whitespace(ch: u32) -> bool {
    matches!(
        ch,
        0x0020 | 0x0009 | 0x000B | 0x000C | 0x00A0 | 0xFEFF |
        // Unicode Space_Seperator category (minus \u{0020} and \u{00A0} which are allready stated above)
        0x1680 | 0x2000..=0x200A | 0x202F | 0x205F | 0x3000
    )
}
