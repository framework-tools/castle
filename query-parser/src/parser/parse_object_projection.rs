use std::{io::Read};

use crate::{ast::syntax_definitions::{want::{Want, ObjectProjection, SingleField}, keyword::{Keyword}}, token::{token::{TokenKind, Punctuator, Identifier}}, tokenizer::{tokenizer::Tokenizer, self}};
use shared::CastleError;

pub fn parse_object_projection<R>(identifier: Box<str>, tokenizer: &mut Tokenizer<R>, should_skip_start: bool) -> Result<Want, CastleError> 
where R: Read{
    
    let mut fields = Vec::new();
    let mut err: Option<Result<Want, CastleError>> = None;
    
    if should_skip_start { skip_ident_colon_and_openblock(tokenizer)?; }
    loop {
        let token = tokenizer.next(true)?;
        match token {
            Some(token) => match token.kind {
                TokenKind::Identifier(Identifier {name, arguments}) => {
                    let peeked_token = tokenizer.peek(false)?;
                    match peeked_token {
                        Some(peeked_token) => match peeked_token.kind {
                            TokenKind::Punctuator(Punctuator::Colon) => {
                                tokenizer.next(true)?; // consume ':'
                                let peeked_token = tokenizer.peek(true)?;
                                match peeked_token {
                                    Some(peeked_token) => match &peeked_token.kind {
                                        TokenKind::Keyword(Keyword::Match) => {
                                            tokenizer.next(true)?; // consume the match keyword
                                            tokenizer.next(true)?; // consume the open block
                                            let match_statements = parse_match_statement(tokenizer)?;
                                            let field = Want::new_projection(name.clone(), None, match_statements);
                                            fields.push(field.into());
                                        },
                                        TokenKind::Punctuator(Punctuator::OpenBlock) => {
                                            tokenizer.next(true)?; // consume the open block
                                            let object_projection = parse_object_projection(name.clone(), tokenizer, false)?;
                                            fields.push(object_projection.into());
                                        },
                                        _ => break // end of object projection
                                    },
                                    None => break
                                };
                            }
                            _ => {
                                let field = Want::SingleField(SingleField{ identifier: name,arguments });
                                fields.push(field.into());
                            }
                        },
                        None => {}
                    }
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
            fields: Some(fields),
            match_statements: None
        };
        Ok(Want::Projection(object_projection))
    }

}

/// Parses a match statement
/// loop through tokens
/// parse object projection for each possible match statement
/// 
/// 
fn parse_match_statement<R>(tokenizer: &mut Tokenizer<R>) -> Result<Option<Vec<Box<Want>>>, CastleError>
where R: Read{
    let mut match_statements = Vec::new();
    loop {
        let token = tokenizer.peek(true)?;
        match token {
            Some(token) => match &token.kind {
                TokenKind::Punctuator(Punctuator::CloseBlock) => {
                    tokenizer.next(true)?; // consume the close block
                    break;
                },
                TokenKind::Identifier(Identifier { name, .. }) => {
                    let statement = parse_object_projection(name.clone(), tokenizer, true)?;
                        match_statements.push(statement.into());
                }
                _ => break
            }
            None => break
        };
    }
    return Ok(Some(match_statements))
}

fn skip_ident_colon_and_openblock<R>(tokenizer: &mut Tokenizer<R>) -> Result<(), CastleError>
where R: Read {
    tokenizer.next(true)?; // consume the identifer
    tokenizer.next(true)?; // consume the colon
    tokenizer.next(true)?; // consume the open block
    return Ok(())
}