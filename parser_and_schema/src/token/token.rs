use std::{fmt::{Formatter, self, Display}};

use input_cursor::{Span, Position};
use shared::castle_error::CastleError;


use crate::{ast::{keyword::Keyword,expressions::PrimitiveValue}};

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    BooleanLiteral(bool),
    Identifier(Box<str>),
    NumericLiteral(Numeric),
    Punctuator(Punctuator),
    StringLiteral(Box<str>),
    LineTerminator,
    Comment,
    Keyword(Keyword),
}

#[derive(Debug, PartialEq)]
pub enum Numeric {
    Float(f64),
    Integer(i64),
    UnsignedInteger(u64)
}

impl Token {

    #[inline]
    pub fn new (kind: TokenKind, span: Span) -> Self {
        Self {
            kind,
            span
        }
    }

    pub fn operator_as_str_to_token(ch: &char, start:Position, end: Position) -> Result<Self, CastleError>{
        match ch {
            '{' => Ok(Token::new(TokenKind::Punctuator(Punctuator::OpenBlock), Span::new(start, end))),
            '}' => Ok(Token::new(TokenKind::Punctuator(Punctuator::CloseBlock), Span::new(start, end))),
            ':' => Ok(Token::new(TokenKind::Punctuator(Punctuator::Colon), Span::new(start, end))),
            ',' => Ok(Token::new(TokenKind::Punctuator(Punctuator::Comma), Span::new(start, end))),
            '(' => Ok(Token::new(TokenKind::Punctuator(Punctuator::OpenParen), Span::new(start, end))),
            ')' => Ok(Token::new(TokenKind::Punctuator(Punctuator::CloseParen), Span::new(start, end))),
            '-' => Ok(Token::new(TokenKind::Punctuator(Punctuator::Sub), Span::new(start, end))),
            '>' => Ok(Token::new(TokenKind::Punctuator(Punctuator::GreaterThan), Span::new(start, end))),
            '@' => Ok(Token::new(TokenKind::Punctuator(Punctuator::At), Span::new(start, end))),
            '=' => Ok(Token::new(TokenKind::Punctuator(Punctuator::Assign), Span::new(start, end))),
            _ => Err(CastleError::Unimplemented("Unimplemented Operator".into()))
        }
    }

    #[inline]
    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    #[inline]
    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn convert_token_to_primitive(self) -> Result<PrimitiveValue, CastleError> {
        match self.kind {
            TokenKind::StringLiteral(s) => Ok(PrimitiveValue::String(s)),
            TokenKind::NumericLiteral(numeric) => Ok(match_numeric_token_to_primitive(numeric)?),
            TokenKind::BooleanLiteral(b) => Ok(PrimitiveValue::Boolean(b)),
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::True => Ok(PrimitiveValue::Boolean(true)),
                Keyword::False => Ok(PrimitiveValue::Boolean(false)),
                _ => Err(CastleError::Schema(format!("Expected primitive value, found keyword: {:?}", &keyword).into(), self.span)),
            },
            _ => Err(CastleError::Schema(format!("Expected primitive value, found: {:?}", self.kind).into(), self.span))
        }
    }
}


impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Punctuator {
    // Operator
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %
    Pow, // ^

    // Assignment Operator
    Assign, // =
    AssignAdd, // +=
    AssignSub, // -=
    AssignMul, // *=
    AssignDiv, // /=
    AssignMod, // %=
    AssignPow, // ^=

    // Equality
    Eq, // ==
    NotEq, // !=
    LessThan, // < - Also used for Generic Type Parameters
    LessThanOrEq, // <=
    GreaterThan, // > - Also used for Generic Type Parameters
    GreaterThanOrEq, // >=

    // Logic
    And, // &
    Or, // |
    Not, // !

    // Symbols
    Colon, // :
    SemiColon, // ;
    Dot, // .
    Comma, // ,
    Spread, // ...
    At, // @
    DoubleColon, // ::

    // Brackets, Parenthesis, Blocks
    OpenBlock, // {
    CloseBlock, // }
    OpenParen, // (
    CloseParen, // )
    OpenBracket, // [
    CloseBracket, // ]
}

impl From<Keyword> for TokenKind {
    fn from(keyword: Keyword) -> Self {
        return match keyword {
            Keyword::True => TokenKind::BooleanLiteral(true),
            Keyword::False => TokenKind::BooleanLiteral(false),
            Keyword::As => TokenKind::Keyword(Keyword::As),
            Keyword::Match => TokenKind::Keyword(Keyword::Match),
            Keyword::Type => TokenKind::Keyword(Keyword::Type),
            Keyword::Enum => TokenKind::Keyword(Keyword::Enum),
            Keyword::Fn => TokenKind::Keyword(Keyword::Fn),
            Keyword::Into => TokenKind::Keyword(Keyword::Into),
            Keyword::Directive => TokenKind::Keyword(Keyword::Directive),
            Keyword::On => TokenKind::Keyword(Keyword::On),
        }
    }
}

fn match_numeric_token_to_primitive(numeric:Numeric) -> Result<PrimitiveValue, CastleError> {
    match numeric {
        Numeric::Float(f) => Ok(PrimitiveValue::Float(f)),
        Numeric::Integer(i) => Ok(PrimitiveValue::Int(i)),
        Numeric::UnsignedInteger(u) => Ok(PrimitiveValue::UInt(u)),
    }
}