use core::num;
use std::{io::Read, collections::VecDeque};

use input_cursor::{Cursor, Span, Position};
use shared::CastleError;

use crate::{token::{Token, token::{TokenKind, Punctuator, Numeric}}, ast::syntax_definitions::{expressions::Expression, keyword::Keyword}};
pub struct Tokenizer<R> {
    cursor: Cursor<R>,
    peeked: VecDeque<Token>
}

impl<R> Tokenizer<R>
where
    R: Read,
{
    fn is_whitespace(ch: u32) -> bool {
        matches!(
            ch,
            0x0020 | 0x0009 | 0x000B | 0x000C | 0x00A0 | 0xFEFF |
            // Unicode Space_Seperator category (minus \u{0020} and \u{00A0} which are allready stated above)
            0x1680 | 0x2000..=0x200A | 0x202F | 0x205F | 0x3000
        )
    }

    pub fn new(reader: R) -> Self
    where
        R: Read,
    {
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
            None => self.advance()?,
        };

        match token {
            Some(token) => {
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
        // skip whitespaces
        let (start, next_ch) = loop {
            let start = self.cursor.pos();
            if let Some(next_ch) = self.cursor.peek_char()? {
                // Ignore whitespace
                if !Self::is_whitespace(next_ch) {
                    break (start, next_ch);
                }
                self.cursor.next_char()?; // consume whitespace
            } else {
                return Ok(None);
            }
        };

        if let Ok(c) = char::try_from(next_ch) {
            let token = match c {
                '\r' | '\n' => parse_newline(&mut self.cursor, start)?,
                '"' => parse_string(&mut self.cursor, start)?,
                // spread Operator & dot
                // '.' => parse_spread_or_dot(&mut self.cursor, start)?,

                // Operator & Punctuator
                '=' | '!' | '<' | '>' | '+' | '-' | '*' | '/' | '%' | '&' | '|' | '^' |
                ':' | '{' | '}' | '[' | ']' | ',' | ';' | '@' | '#' | '(' | ')'  => parse_operator(&mut self.cursor, start)?,

                _ if c.is_digit(10) => parse_number(&mut self.cursor, start)?,
                _ if c.is_ascii_alphabetic() => parse_identifier(&mut self.cursor, start)?,
                _ => {
                    return Err(CastleError::syntax(
                        format!(
                            "Unexpected '{}' at line {}, column {}",
                            c,
                            start.line_number(),
                            start.column_number()
                        ),
                        start,
                    ))
                }
            };

            return Ok(Some(token));
        };

        Ok(None) // EOF
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
        let punctuator_token = self.next(skip_line_terminators)?.ok_or(CastleError::EOF)?;
        if punctuator_token.kind != TokenKind::Punctuator(*punctuator) {
            return Err(CastleError::parse(
                format!(
                    "Expected punctuator '{}', but got '{}'",
                    punctuator, punctuator_token.kind
                ),
                punctuator_token.span,
            ));
        }
        Ok(())
    }

    pub fn expect_keyword(
        &mut self,
        keyword: &Keyword,
        skip_line_terminators: bool,
    ) -> Result<(), CastleError> {
        let keyword_token = self.next(skip_line_terminators)?.ok_or(CastleError::EOF)?;
        if keyword_token.kind != TokenKind::Keyword(*keyword) {
            return Err(CastleError::parse(
                format!(
                    "Expected keyword '{}', but got '{}'",
                    keyword, keyword_token.kind
                ),
                keyword_token.span,
            ));
        }
        Ok(())
    }

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

    pub fn expect_identifier(
        &mut self,
        skip_line_terminators: bool,
    ) -> Result<Box<str>, CastleError> {
        expect_identifier(self, skip_line_terminators)
    }

    /// ### Parses an expression
    /// will return an error if the next set of tokens is not a valid expression
    /// otherwise returns a expression
    pub fn expect_expression(&mut self) -> Result<Expression, CastleError> {
        expect_expression(self)
    }

    pub fn parse_primary(&mut self) -> Result<Expression, CastleError> {
        parse_primary(self)
    }

    /// ### Parses an expression
    /// Uses Operator precedence to parse an expression
    pub fn precedence_parser(
        &mut self,
        lhs: Expression,
        min_precedence: Operator,
    ) -> Result<Expression, CastleError> {
        precedence_parser(self, lhs, min_precedence)
    }

    pub fn expect_primitive(&mut self) -> Result<PrimitiveValue, CastleError> {
        let primitive_value = self.next(true)?.ok_or(CastleError::EOF)?;
        let primitive_value = match primitive_value.kind {
            TokenKind::BooleanLiteral(bool) => PrimitiveValue::Boolean(bool),
            TokenKind::StringLiteral(str) => PrimitiveValue::String(str),
            TokenKind::NumericLiteral(numeric) => match numeric {
                Numeric::Integer(i) => PrimitiveValue::Int(i),
                Numeric::Rational(f) => PrimitiveValue::Float(f),
                Numeric::UnsignedInteger(u) => PrimitiveValue::UInt(u),
            },
            _ => {
                return Err(CastleError::parse(
                    format!("Expected primitive, got '{}'", primitive_value.kind),
                    primitive_value.span,
                ))
            }
        };

        Ok(primitive_value)
    }

    pub fn peek_operator(&mut self) -> Result<Option<Operator>, CastleError> {
        let token = self.peek(true)?.or(None);
        match token {
            Some(token) => match token.kind() {
                TokenKind::Punctuator(punc) => match punc.as_operator() {
                    Some(op) => return Ok(Some(op)),
                    _ => Ok(None),
                },
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
}


fn parse_newline<R>(
    cursor: &mut Cursor<R>,
    start: Position,
) -> Result<Token, CastleError> where
    R: Read,
{
    Ok(Token::new(TokenKind::LineTerminator, Span::new(start, cursor.pos())))
}


fn parse_string<R>(
    cursor: &mut Cursor<R>,
    start: Position,
) -> Result<Token, CastleError> where R: Read {
    cursor.next_char(); // skip the first quote

    let mut string = String::new();

    loop {
        let c = cursor.next_char()?.ok_or(CastleError::AbruptEOF)?;

        let ch = char::try_from(c).ok().ok_or(CastleError::lex(
            "invalid character",
            cursor.pos(),
        ))?;



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
            }
        } else if ch == '"' {
            break;
        } else {
            string.push(ch);
        }
    };

    Ok(Token::new(TokenKind::StringLiteral(string.into_boxed_str()), Span::new(start, cursor.pos())))
}


fn parse_identifier<R>(
    cursor: &mut Cursor<R>,
    start: Position,
) -> Result<Token, CastleError> where R: Read {
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
        } else {
            break;
        }
    }

    let token_kind = if let Ok(keyword) = identifier_name.parse() {
        match keyword {
            Keyword::True => TokenKind::BooleanLiteral(true),
            Keyword::False => TokenKind::BooleanLiteral(false),
            _ => TokenKind::Keyword(keyword),
        }
    } else {
        TokenKind::Identifier(identifier_name.into_boxed_str())
    };

    Ok(Token::new(token_kind, Span::new(start_pos, cursor.pos())))
}

fn parse_operator<R>(
    cursor: &mut Cursor<R>,
    start: Position,
) -> Result<Token, CastleError> where R: Read {
    
    let c = cursor.next_char()?.ok_or(CastleError::AbruptEOF)?;
    let ch = char::try_from(c).ok().ok_or(CastleError::lex(
        "invalid character",
        cursor.pos(),
    ))?;
    return match ch {
            '{' => Ok(Token::new(TokenKind::Punctuator(Punctuator::OpenBlock), Span::new(start, cursor.pos()))),
            '}' => Ok(Token::new(TokenKind::Punctuator(Punctuator::CloseBlock), Span::new(start, cursor.pos()))),
            ':' => Ok(Token::new(TokenKind::Punctuator(Punctuator::Colon), Span::new(start, cursor.pos()))),
            ',' => Ok(Token::new(TokenKind::Punctuator(Punctuator::Comma), Span::new(start, cursor.pos()))),
            '(' => Ok(Token::new(TokenKind::Punctuator(Punctuator::OpenParen), Span::new(start, cursor.pos()))),
            ')' => Ok(Token::new(TokenKind::Punctuator(Punctuator::CloseParen), Span::new(start, cursor.pos())))
    }
}

/// peek next character
/// if character is digit or "." push to num_as_string
/// next character 
/// else break
/// convert string to number
/// create token and return
fn parse_number<R>(
    cursor: &mut Cursor<R>,
    start: Position,
) -> Result<Token, CastleError> where R: Read {
    let num_as_string = String::new();
    loop {
        let c = cursor.peek_char()?.ok_or(CastleError::AbruptEOF)?;
        let ch = char::try_from(c).ok().ok_or(CastleError::lex("invalid character",cursor.pos()))?;
        if ch.is_digit(10) || ch == '.' || ch == '-' {
            num_as_string.push(ch);
            cursor.next_char()?;
        } 
        else { break; }
    }
    return convert_num_as_string_to_token(num_as_string, start, cursor)
}

fn convert_num_as_string_to_token<R>(
    num_as_string: String,
    start: Position,
    cursor: &mut Cursor<R>,
) -> Result<Token, CastleError> where R: Read {
    if num_as_string.contains('.') { Ok(Token::new(TokenKind::NumericLiteral(Numeric::Float(num_as_string.parse().unwrap())), Span::new(start, cursor.pos()))) }
    else if num_as_string.contains('-') { Ok(Token::new(TokenKind::NumericLiteral(Numeric::Integer(num_as_string.parse().unwrap())), Span::new(start, cursor.pos()))) }
    else { Ok(Token::new(TokenKind::NumericLiteral(Numeric::UnsignedInteger(num_as_string.parse().unwrap())), Span::new(start, cursor.pos()))) }
}