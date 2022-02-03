use std::{io::Read};

use crate::{ast::syntax_definitions::want::{Want, ObjectProjection}, token::{token::{TokenKind, Punctuator}}, tokenizer::tokenizer::Tokenizer};
use shared::CastleError;

pub fn create_object_projection<R>(identifier: Box<str>, tokenizer: &mut Tokenizer<R>) -> Result<Want, CastleError> 
where R: Read{
    
    let identifier = Some(identifier);
    let mut fields = Vec::new();
    let mut err: Option<Result<Want, CastleError>> = None;
    
    tokenizer.next(true)?; // skip open block
    println!("yes");
    loop {
        let token = tokenizer.peek(true)?;
        println!("Token inside object projection: {:#?}", &token);
        match token {
            Some(token) => match &token.kind {
                TokenKind::Identifier(identifier) => {
                    let field = Want::new_single_field(identifier.clone());
                    fields.push(Box::new(field));
                },
                TokenKind::Punctuator(Punctuator::Comma) => { tokenizer.next(true)?; },
                TokenKind::Punctuator(Punctuator::CloseBlock) => {
                    tokenizer.next(true)?;
                    break;
                },
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