use std::fmt::{Formatter, self, Display};

use input_cursor::Span;


pub struct Token {
    pub kind: TokenKind,
    pub span: Span
}

pub enum TokenKind {

}


impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}


impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {

        }
    }
}