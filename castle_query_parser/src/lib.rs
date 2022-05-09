#![feature(if_let_guard)]

pub(crate) mod parsers;
pub(crate) mod types;

use std::collections::HashMap;

use castle_error::CastleError;
use parsers::parse_projection::{parse_projection};
use castle_tokenizer::{Tokenizer, Tokenizable, TokenKind, Keyword, Punctuator};
pub use types::{Field, FieldKind, Projection, Inputs, Input, Message};

pub fn parse_message(msg: &str) -> Result<Message, CastleError> {
    let bytes = msg.as_bytes();
    let mut tokenizer = Tokenizer::new(bytes);
    Ok(Message {
        projection: match tokenizer.next(true)? {
            Some(token) if let TokenKind::Keyword(Keyword::Message) = token.kind => {
                parse_projection(&mut tokenizer, Punctuator::OpenBlock, Punctuator::CloseBlock)?
            }
            Some(token) => return Err(CastleError::Root(
                format!("Expected query keyword or EOF, got: {:?}", token.kind).into(),
                token.span
            )),
            None => HashMap::new(),
        },
    })
}

