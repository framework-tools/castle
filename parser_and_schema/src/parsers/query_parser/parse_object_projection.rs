use std::{io::Read, collections::HashMap};

use crate::{ast::syntax_definitions::{want::{Want, ObjectProjection, SingleField}, keyword::{Keyword}}, token::{token::{TokenKind, Punctuator, Identifier}, Token}, tokenizer::{tokenizer::Tokenizer, tokenizer_utils::get_next_token_and_unwrap}};
use shared::CastleError;

use crate::ast::syntax_definitions::argument::Argument;

use super::{parse_inner_object::parse_inner_object, parse_match_statements::parse_match_statements};

pub fn parse_object_projection<R>(identifier:Identifier, tokenizer: &mut Tokenizer<R>, should_skip_start: bool) -> Result<Want, CastleError> 
where R: Read{

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
        let end_of_fields = match_current_token_to_field_and_parse_field(tokenizer, token, &mut fields)?;
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

fn match_current_token_to_field_and_parse_field(tokenizer: &mut Tokenizer<impl Read>, token: Token, fields: &mut HashMap<Box<str>, Want>) 
-> Result<bool, CastleError> {
    match token.kind {
        TokenKind::Identifier(identifier) => {
            let should_break = parse_field(tokenizer, fields, identifier)?;
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

pub fn parse_field<R>(tokenizer: &mut Tokenizer<R>, fields: &mut HashMap<Box<str>, Want>, identifier: Identifier) 
-> Result<bool, CastleError> where R: Read {
    let peeked_token = tokenizer.peek(true)?;
    match peeked_token {
        Some(peeked_token) => match peeked_token.kind {
            TokenKind::Punctuator(Punctuator::Colon) => {
                let should_break = check_match_or_object_then_parse(tokenizer, fields, identifier)?;
                if should_break { return Ok(true) }
                else { return Ok(false) }
            }
            _ => {
                let field = Want::SingleField(SingleField{ identifier: identifier.name.clone(), arguments: identifier.arguments, match_statement: None });
                fields.insert(identifier.name, field);
                return Ok(false)
            }
        },
        None => { return Ok(false) }
    }
}

fn check_match_or_object_then_parse<R>(tokenizer: &mut Tokenizer<R>, fields: &mut HashMap<Box<str>, Want>, identifier: Identifier) -> Result<bool, CastleError> 
where R: Read {
    tokenizer.next(true)?; // consume ':'
    let peeked_token = tokenizer.peek(true)?;
    return match peeked_token {
        Some(peeked_token) => match &peeked_token.kind {
            TokenKind::Keyword(Keyword::Match) => {
                tokenizer.next(true)?; // consume the match keyword
                let match_statements = parse_match_statements(tokenizer)?;
                fields.insert(identifier.name.clone(), Want::new_object_projection(Some(identifier.name), None, Some(match_statements), None));
                return Ok(false)
            },
            TokenKind::Punctuator(Punctuator::OpenBlock) => {
                parse_inner_object(tokenizer, fields, identifier)?;
                return Ok(false)
            },
            _ => Ok(true) // end of object projection
        },
        None => Ok(true) // end of object projection
    }
}

fn create_obj(identifier: Identifier, fields: HashMap<Box<str>, Want>) -> Result<Want, CastleError> {
    let arguments;
    if identifier.arguments.is_some() {
        let ident_args= identifier.arguments.unwrap();
        if ident_args.len() == 0 { arguments = None }
        else { arguments = Some(ident_args) }
    } else {
        arguments = None;
    }
    let object_projection = ObjectProjection {
        identifier: Some(identifier.name),
        arguments,
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