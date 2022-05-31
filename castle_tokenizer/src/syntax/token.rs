use std::{fmt::{Formatter, self, Display}};

use castle_input_cursor::{Span};
use castle_types::Primitive;

use crate::{Punctuator, Keyword};


#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Primitive(Primitive),
    Identifier(Box<str>),
    Punctuator(Punctuator),
    LineTerminator,
    Keyword(Keyword),
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