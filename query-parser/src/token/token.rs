use std::fmt::{Formatter, self, Display};

use input_cursor::Span;

use crate::ast::syntax_definitions::keyword::Keyword;


pub struct Token {
    pub kind: TokenKind,
    pub span: Span
}

#[derive(Debug, PartialEq,)]
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

    #[inline]
    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    #[inline]
    pub fn span(&self) -> &Span {
        &self.span
    }
}


impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}


// impl Display for TokenKind {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         match *self {

//         }
//     }
// }


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
