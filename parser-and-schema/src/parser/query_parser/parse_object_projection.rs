use std::{io::Read, collections::HashMap};

use crate::{ast::syntax_definitions::{want::{Want, ObjectProjection, SingleField}, keyword::{Keyword}}, token::{token::{TokenKind, Punctuator, Identifier}, Token}, tokenizer::{tokenizer::Tokenizer, tokenizer_utils::get_next_token_and_unwrap}};
use shared::CastleError;

use crate::ast::syntax_definitions::argument::Argument;

use super::{parse_inner_object::parse_inner_object, parse_match_statements::parse_match_statements};

pub fn parse_object_projection<R>(identifier: Box<str>, tokenizer: &mut Tokenizer<R>, should_skip_start: bool) -> Result<Want, CastleError> 
where R: Read{
    if should_skip_start { skip_ident_colon_and_openblock(tokenizer)?; }

    let fields = loop_through_tokens_and_parse_fields(tokenizer)?;
    let parsed_object = create_obj(identifier, fields);

    return parsed_object
}


fn loop_through_tokens_and_parse_fields<R>(tokenizer: &mut Tokenizer<R>) -> Result<HashMap<Box<str>, Want>, CastleError> 
where R: Read {
    let mut fields: HashMap<Box<str>, Want> = HashMap::new();
    let err = None;
    loop {
        let token = get_next_token_and_unwrap(tokenizer)?;
        let end_of_fields = match_current_token_to_field_and_parse_Field(tokenizer, token, &mut fields)?;
        if end_of_fields { break; }
    }
    handle_errors_for_fields(err, &mut fields)?;
    return Ok(fields)
}

fn handle_errors_for_fields(err: Option<CastleError>, fields: &mut HashMap<Box<str>, Want> ) -> Result<&mut HashMap<Box<str>, Want>, CastleError> {
    if err.is_some() { return Err(err.unwrap()); }
    else if fields.is_empty() { return Err(CastleError::EmptyObject("can't create empty object".into())) }
    else { return Ok(fields) }
}

fn match_current_token_to_field_and_parse_Field(tokenizer: &mut Tokenizer<impl Read>, token: Token, fields: &mut HashMap<Box<str>, Want>) 
-> Result<bool, CastleError> {
    match token.kind {
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
    }
}

fn parse_field<R>(tokenizer: &mut Tokenizer<R>, fields: &mut HashMap<Box<str>, Want>, name: Box<str>, arguments: Option<Vec<Argument>>) 
-> Result<bool, CastleError> where R: Read {
    let peeked_token = tokenizer.peek(true)?;
    match peeked_token {
        Some(peeked_token) => match peeked_token.kind {
            TokenKind::Punctuator(Punctuator::Colon) => {
                let should_break = check_match_or_object_then_parse(tokenizer, fields, name)?;
                if should_break { return Ok(true) }
                else { return Ok(false) }
            }
            _ => {
                let field = Want::SingleField(SingleField{ identifier: name.clone(), arguments, match_statement: None });
                fields.insert(name, field);
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

fn check_match_or_object_then_parse<R>(tokenizer: &mut Tokenizer<R>, fields: &mut HashMap<Box<str>, Want>, name: Box<str>) -> Result<bool, CastleError> 
where R: Read {
    tokenizer.next(true)?; // consume ':'
    let peeked_token = tokenizer.peek(true)?;
    return match peeked_token {
        Some(peeked_token) => match &peeked_token.kind {
            TokenKind::Keyword(Keyword::Match) => {
                tokenizer.next(true)?; // consume the match keyword
                parse_match_statements(tokenizer, name.clone())?;
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

fn create_obj(identifier: Box<str>, fields: HashMap<Box<str>, Want>) -> Result<Want, CastleError> {
        let object_projection = ObjectProjection {
            identifier: Some(identifier),
            fields: Some(fields),
            match_statement: None
        };
        return Ok(Want::ObjectProjection(object_projection))
}

fn create_err_for_parser(token: &Token) -> Result<Option<Result<Want, CastleError>>, CastleError> {
    let err = Some(Err(CastleError::Parser(
        format!("unexpected token, expected identifier, close block or comma, got {:?}", token.kind).into(),
        token.span
    )));
    return Ok(err)
}