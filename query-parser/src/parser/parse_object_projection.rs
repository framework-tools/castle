use std::{io::Read};

use crate::{ast::syntax_definitions::want::{Want, ObjectProjection}, token::{token::{TokenKind, Punctuator}}, tokenizer::tokenizer::Tokenizer};
use shared::CastleError;

pub fn parse_object_projection<R>(identifier: Box<str>, tokenizer: &mut Tokenizer<R>) -> Result<Want, CastleError> 
where R: Read{
    
    let identifier = Some(identifier);
    let mut fields = Vec::new();
    let mut err: Option<Result<Want, CastleError>> = None;
    
    loop {
        let token = tokenizer.next(true)?;
        match token {
            Some(token) => match &token.kind {
                TokenKind::Identifier(identifier) => {
                    let arguments = None; //need to implement
                    let field = Want::new_single_field(identifier.clone(), arguments);
                    fields.push(Box::new(field));
                },
                TokenKind::Punctuator(Punctuator::Comma) => {},
                TokenKind::Punctuator(Punctuator::CloseBlock) => { break; },
                _ => {
                    err = Some(Err(CastleError::Parser(
                        format!("unexpected token, expected identifier, close block or comma, got {:?}", token.kind).into(),
                        token.span
                    )));
                    break;
                }
            },
            None => break,
        };
    }
    if let Some(err) = err {
        return err
    } else {
        let object_projection = ObjectProjection {
            identifier,
            fields
        };
        Ok(Want::Projection(object_projection))
    }

}