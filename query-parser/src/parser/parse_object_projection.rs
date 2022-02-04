use std::{io::Read};

use crate::{ast::syntax_definitions::{want::{Want, ObjectProjection, SingleField}, keyword::{Keyword}, expressions::PrimitiveValue}, token::{token::{TokenKind, Punctuator, Identifier}, Token}, tokenizer::{tokenizer::Tokenizer}};
use shared::CastleError;

use super::{parse_inner_object::parse_inner_object, parse_match_statements::parse_match_statements};

pub fn parse_object_projection<R>(identifier: Box<str>, tokenizer: &mut Tokenizer<R>, should_skip_start: bool) -> Result<Want, CastleError> 
where R: Read{
    let mut fields = Vec::new();
    if should_skip_start { skip_ident_colon_and_openblock(tokenizer)?; }

    loop_through_tokens_and_parse(tokenizer, &mut fields)?;
    let parsed_object = create_obj(identifier, fields);
    return parsed_object
}


fn loop_through_tokens_and_parse<R>(tokenizer: &mut Tokenizer<R>, fields: &mut Vec<Box<Want>>) -> Result<(), CastleError> 
where R: Read{
    loop {
        let token = tokenizer.next(true)?;
        let should_break = match_current_token_to_field(tokenizer, token, fields)?;
        if should_break { break; }
    }
    return Ok(())
}

fn match_current_token_to_field(tokenizer: &mut Tokenizer<impl Read>, token: Option<Token>, fields: &mut Vec<Box<Want>>) -> Result<bool, CastleError> {
    match token {
        Some(token) => match token.kind {
            TokenKind::Identifier(Identifier {name, arguments}) => {
                let should_break = parse_field(tokenizer, fields, name, arguments)?;
                if should_break { return Ok(true) }
                else { return Ok(false) }
            },
            TokenKind::Punctuator(Punctuator::Comma) => { return Ok(false )},
            TokenKind::Punctuator(Punctuator::CloseBlock) => { return Ok(true) },
            _ => {
                create_err_for_parser(&token)?;
                return Ok(true)
            }
        },
        None => return Ok(true),
    }
}

fn parse_field<R>(tokenizer: &mut Tokenizer<R>, fields: &mut Vec<Box<Want>>, name: Box<str>, arguments: Option<Vec<PrimitiveValue>>) -> Result<bool, CastleError> 
where R: Read {
    let peeked_token = tokenizer.peek(true)?;
    match peeked_token {
        Some(peeked_token) => match peeked_token.kind {
            TokenKind::Punctuator(Punctuator::Colon) => {
                let should_break = check_match_or_object_then_parse(tokenizer, fields, name)?;
                if should_break { return Ok(true) }
                else { return Ok(false) }
            }
            _ => {
                let field = Want::SingleField(SingleField{ identifier: name, arguments });
                fields.push(field.into());
                return Ok(false)
            }
        },
        None => { return Ok(false) }
    }
}

fn skip_ident_colon_and_openblock<R>(tokenizer: &mut Tokenizer<R>) -> Result<(), CastleError>
where R: Read {
    tokenizer.next(true)?; // consume the identifer
    tokenizer.next(true)?; // consume the colon
    tokenizer.next(true)?; // consume the open block
    return Ok(())
}

fn check_match_or_object_then_parse<R>(tokenizer: &mut Tokenizer<R>, fields: &mut Vec<Box<Want>>, name: Box<str>) -> Result<bool, CastleError> 
where R: Read {
    tokenizer.next(true)?; // consume ':'
    let peeked_token = tokenizer.peek(true)?;
    return match peeked_token {
        Some(peeked_token) => match &peeked_token.kind {
            TokenKind::Keyword(Keyword::Match) => {
                parse_match_statements(tokenizer, fields, name.clone())?;
                return Ok(false)
            },
            TokenKind::Punctuator(Punctuator::OpenBlock) => {
                parse_inner_object(tokenizer, fields, name.clone())?;
                return Ok(false)
            },
            _ => Ok(true) // end of object projection
        },
        None => Ok(true) // end of object projection
    }
}

fn create_obj(identifier: Box<str>, fields: Vec<Box<Want>>) -> Result<Want, CastleError> {
        let object_projection = ObjectProjection {
            identifier,
            fields: Some(fields),
            match_statements: None
        };
        return Ok(Want::Projection(object_projection))
}

fn create_err_for_parser(token: &Token) -> Result<Option<Result<Want, CastleError>>, CastleError> {
    let err = Some(Err(CastleError::Parser(
        format!("unexpected token, expected identifier, close block or comma, got {:?}", token.kind).into(),
        token.span
    )));
    return Ok(err)
}