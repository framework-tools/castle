


use std::io::Read;

use shared::castle_error::CastleError;

use crate::{token::{token::{TokenKind, Punctuator}, Token}, tokenizer::tokenizer::Tokenizer};

#[derive(Debug, PartialEq)]
pub struct Type {
    pub name: Box<str>,
    pub generics: Vec<Type>,
}

pub fn parse_type<R: Read>(tokenizer: &mut Tokenizer<R>) -> Result<Type, CastleError> {
    Ok(Type {
        name: tokenizer.expect_identifier(true)?, 
        generics:parse_generics(tokenizer)?
    })
}

fn parse_generics<R: Read>(tokenizer: &mut Tokenizer<R>) -> Result<Vec<Type>, CastleError> {
    let mut generics = Vec::new();
    match tokenizer.peek(true)? {
        Some(Token { kind: TokenKind::Punctuator(Punctuator::LessThan), ..}) => {
            tokenizer.expect_punctuator(Punctuator::LessThan, true)?; // skip <
            loop {
                generics.push(parse_type(tokenizer)?);
                if let Some(Token { kind: TokenKind::Punctuator(Punctuator::GreaterThan), ..}) = tokenizer.peek(true)? {
                    tokenizer.expect_punctuator(Punctuator::GreaterThan, true)?; // skip >
                    break Ok(generics);
                }
            }
        }
        _ => Ok(generics)
    }
}