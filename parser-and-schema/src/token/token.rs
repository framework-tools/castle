use std::{fmt::{Formatter, self, Display}};

use input_cursor::{Span, Position};
use shared::CastleError;

use crate::{ast::syntax_definitions::{keyword::Keyword, expressions::PrimitiveValue, want::Argument}, parser::schema_parser::types::schema_field::{PrimitiveType, Type}, tokenizer::parse_identifier_or_keyword::parse_identifier_or_keyword_or_type};

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    BooleanLiteral(bool),
    Identifier(Identifier),
    NumericLiteral(Numeric),
    Punctuator(Punctuator),
    StringLiteral(Box<str>),
    LineTerminator,
    Comment,
    Keyword(Keyword),
    Arguments(Vec<Box<Token>>),
    PrimitiveType(PrimitiveType),
    VecType(VecType),
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: Box<str>,
    pub arguments: Option<Vec<Argument>>
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

impl From<Keyword> for TokenKind {
    fn from(keyword: Keyword) -> Self {
        return match keyword {
            Keyword::True => TokenKind::BooleanLiteral(true),
            Keyword::False => TokenKind::BooleanLiteral(false),
            Keyword::As => TokenKind::Keyword(Keyword::As),
            Keyword::Some => TokenKind::Keyword(Keyword::Some),
            Keyword::None => TokenKind::Keyword(Keyword::None),
            Keyword::Match => TokenKind::Keyword(Keyword::Match),
            Keyword::Type => TokenKind::Keyword(Keyword::Type),
            Keyword::Enum => TokenKind::Keyword(Keyword::Enum),
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct VecType {
    pub inner_type: Box<Type>,
}

impl VecType {
    pub fn new(type_: &str) -> Option<Type> {
        let mut type_inside_vec_as_str = String::new();
        let mut parsing_inside_type = false;
        let mut i = 0;
        let mut not_a_vec = false;
        loop {
            let c = type_.chars().nth(i);
            let c = c.unwrap();
            //check that first 3 characters of type_ is "Vec"
            if i == 0 { if c != 'V' { not_a_vec = true; break; } }
            if i == 1 { if c != 'e' { not_a_vec = true; break; } }
            if i == 2 { if c != 'c' { not_a_vec = true; break; } }

            //parse type inside vec
            if c == '<' {
                parsing_inside_type = true;
            } else if c == '>' { }
            else if parsing_inside_type {
                type_inside_vec_as_str.push(c);
            }

            i += 1;

            if i == type_.len() {
                break;
            }
        };

        if not_a_vec { return None } else {
            let type_inside_vec = Type::new(type_inside_vec_as_str);
            return Some(Type::VecType( VecType {
                inner_type: type_inside_vec.into()
            }))
        }
    }

    pub fn get_vec_type_struct(type_: Type) -> VecType {
        match type_ {
            Type::VecType(vec_type) => vec_type,
            _ => panic!("Type is not a VecType")
        }
    }
}