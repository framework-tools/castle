#![feature(if_let_guard)]

pub(crate) mod parsers;

use std::collections::HashMap;

use castle_types::{Message, CastleError};
use parsers::parse_projection::{parse_projection};
use castle_tokenizer::{Tokenizer, Tokenizable, TokenKind, Keyword, Punctuator};

pub fn parse_message(msg: &str) -> Result<Message, CastleError> {
    let bytes = msg.as_bytes();
    let mut tokenizer = Tokenizer::new(bytes);
    Ok(Message {
        projection: match tokenizer.next(true)? {
            Some(token) if let TokenKind::Keyword(Keyword::Message) = token.kind => {
                parse_projection(&mut tokenizer, Punctuator::OpenBlock, Punctuator::CloseBlock)?
            }
            Some(token) => return Err(CastleError::Root(
                format!("Expected keyword or EOF, got: {:?}", token.kind).into(),
                token.span
            )),
            None => HashMap::new(),
        },
    })
}

