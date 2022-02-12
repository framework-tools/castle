
use std::{io::Read, collections::VecDeque};

use input_cursor::{Cursor, Position};
use shared::CastleError;

use crate::{token::{Token, token::{TokenKind, Punctuator, Numeric}}, ast::syntax_definitions::{expressions::{PrimitiveValue}, keyword::Keyword}, tokenizer::{parse_newline::parse_newline, parse_string::parse_string}};

use super::{parse_operator::parse_operator, parse_numbers::parse_number, parse_identifier_type_or_keyword::{parse_identifier_or_keyword_or_type}};
pub struct Tokenizer<R> {
    pub cursor: Cursor<R>,
    pub peeked: VecDeque<Token>,
}

impl<R> Tokenizer<R>
where
    R: Read,
{
    

    pub fn new(reader: R) -> Self
    where R: Read{
        Self {
            cursor: Cursor::new(reader),
            peeked: VecDeque::new(),
        }
    }

    pub fn next(&mut self, skip_line_terminators: bool) -> Result<Option<Token>, CastleError>
    where
        R: Read,
    {
        let token = match self.peeked.pop_front() {
            Some(token) => Some(token),
            None => {self.advance()?}
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

    pub fn advance(&mut self) -> Result<Option<Token>, CastleError>
    where
        R: Read,
    {
        return advance_and_parse_token(self)
    }

    pub fn peek_n(
        &mut self,
        skip_n: usize,
        skip_line_terminators: bool,
    ) -> Result<Option<&Token>, CastleError>
    where
        R: Read,
    {
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

    pub fn peek(&mut self, skip_line_terminators: bool) -> Result<Option<&Token>, CastleError>
    where
        R: Read,
    {
        self.peek_n(0, skip_line_terminators)
    }

    pub fn expect_punctuator(
        &mut self,
        punctuator: &Punctuator,
        skip_line_terminators: bool,
    ) -> Result<(), CastleError> {
        let punctuator_token = self.next(skip_line_terminators)?.ok_or(CastleError::AbruptEOF("Error found in 'expect_punctuator'".into()))?;
        if punctuator_token.kind != TokenKind::Punctuator(*punctuator) {
            return Err(CastleError::parse(
                format!(
                    "Expected punctuator '{:?}', but got '{:?}'",
                    punctuator, punctuator_token.kind
                ),
                punctuator_token.span,
            ));
        }
        Ok(())
    }

    // pub fn expect_keyword(&mut self, keyword: &Keyword, skip_line_terminators: bool,) -> Result<(), CastleError> {
    //     let keyword_token = self.next(skip_line_terminators)?.ok_or(CastleError::AbruptEOF)?;
    //     if keyword_token.kind != TokenKind::Keyword(keyword) {
    //         return Err(CastleError::parse(
    //             format!(
    //                 "Expected keyword '{:?}', but got '{:?}'",
    //                 keyword, keyword_token.kind
    //             ),
    //             keyword_token.span,
    //         ));
    //     }
    //     Ok(())
    // }

    pub fn peek_keyword(&mut self, skip_line_terminators: bool) -> Result<Option<&Keyword>, CastleError>
    where
        R: Read,
    {
        let token = self.peek(skip_line_terminators)?;
        match token {
            Some(token) => match token.kind() {
                TokenKind::Keyword(keyword) => Ok(Some(keyword)),
                _ => Ok(None),
            },
            None => Ok(None),
        }
    }

    pub fn expect_identifier(&mut self, skip_line_terminators: bool) -> Result<Box<str>, CastleError> {
        let identifier = self
        .next(skip_line_terminators)?
        .ok_or(CastleError::AbruptEOF("Error found in 'expect_identifier'".into()))?;

        match identifier.kind {
            TokenKind::Identifier(str) => Ok(str.name),
            _ => Err(CastleError::parse(
                format!("Expected identifier, got '{:?}'", identifier.kind),
                identifier.span,
            ))
        }
    }

    pub fn expect_primitive(&mut self) -> Result<PrimitiveValue, CastleError> {
        let primitive_value = self.next(true)?.ok_or(CastleError::AbruptEOF("Error found in 'expect_primitive'".into()))?;
        let primitive_value = match primitive_value.kind {
            TokenKind::BooleanLiteral(bool) => PrimitiveValue::Boolean(bool),
            TokenKind::StringLiteral(str) => PrimitiveValue::String(str),
            TokenKind::NumericLiteral(numeric) => match numeric {
                Numeric::Integer(i) => PrimitiveValue::Int(i),
                Numeric::Float(f) => PrimitiveValue::Float(f),
                Numeric::UnsignedInteger(u) => PrimitiveValue::UInt(u),
            },
            _ => {
                return Err(CastleError::parse(
                    format!("Expected primitive, got '{:?}'", primitive_value.kind),
                    primitive_value.span,
                ))
            }
        };

        Ok(primitive_value)
    }
}

pub fn get_character_with_peek<R>(cursor: &mut Cursor<R>, start: Position) -> Result<char, CastleError> 
where R: Read {
    let c = cursor.peek_char()?.ok_or(CastleError::AbruptEOF("Error found in 'get_character_with_peek'".into()))?;
    let ch = char::try_from(c).ok().ok_or(CastleError::lex("invalid character",cursor.pos()))?;
    return Ok(ch)
}



pub fn advance_and_parse_token<R>(tokenizer: &mut Tokenizer<R>) -> Result<Option<Token>, CastleError>
where R: Read {
    let cursor = &mut tokenizer.cursor;

    let option_next_ch = skip_whitespace(cursor)?;
    if option_next_ch.is_none() { return Ok(None) }
    let (start, next_ch) = option_next_ch.unwrap();

    if let Ok(c) = char::try_from(next_ch) {
        let token = parse_token_from_chars(tokenizer, c, start)?;
        return Ok(Some(token))
    } else {
        return Ok(None) // EOF
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

fn skip_whitespace<R>(cursor: &mut Cursor<R>) -> Result<Option<(Position, u32)>, CastleError>
where R: Read {
    loop {
        let start = cursor.pos();
        if let Some(next_ch) = cursor.peek_char()? {
            // Ignore space
            if !is_whitespace(next_ch) {
                return Ok(Some((start, next_ch)))
            }
            cursor.next_char()?; // consume whitespace
        } else {
            return Ok(None);
        }
    }
}

fn parse_token_from_chars<R>(tokenizer: &mut Tokenizer<R>, c: char, start: Position) -> Result<Token, CastleError> 
where R: Read {
    return match c {
        '\r' | '\n' => parse_newline(&mut tokenizer.cursor, start),
        '"' => parse_string(&mut tokenizer.cursor, start),

        // Operator & Punctuator
        '=' | '!' | '<' | '>' | '+' | '-' | '*' | '/' | '%' | '&' | '|' | '^' |
        ':' | '{' | '}' | '[' | ']' | ',' | ';' | '@' | '#' | '(' | ')'  => parse_operator(&mut tokenizer.cursor, start),

        _ if c.is_digit(10) => parse_number(&mut tokenizer.cursor, start),
        _ if c.is_ascii_alphabetic() => parse_identifier_or_keyword_or_type(tokenizer, start),
        _ => {
            return Err(CastleError::Lexer(
                format!(
                    "Unexpected '{}' at line {}, column {}",
                    c,
                    start.line_number(),
                    start.column_number()
                ).into(),
                start,
            ))
        }
    }
}