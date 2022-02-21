use std::{io::Read, collections::HashMap};

use shared::castle_error::CastleError;

use crate::{ast::syntax_definitions::{want::{Want, WantArguments, Wants}, keyword::{Keyword}, match_statement::MatchArm}, token::{token::{TokenKind, Punctuator, Identifier}, Token}, tokenizer::{tokenizer::Tokenizer, tokenizer_utils::get_next_token_and_unwrap}};


use crate::ast::syntax_definitions::argument::ArgumentOrTuple;

use super::{parse_inner_object::parse_inner_object, parse_match_statements::parse_match_statements};

pub fn parse_object_projection<R>(identifier:Identifier, tokenizer: &mut Tokenizer<R>) -> Result<Want, CastleError> 
where R: Read{

    let fields = loop_through_tokens_and_parse_fields(tokenizer)?;
    let parsed_object = create_obj(identifier, fields);

    return parsed_object
}


pub fn loop_through_tokens_and_parse_fields<R>(tokenizer: &mut Tokenizer<R>) -> Result<HashMap<Box<str>, Want>, CastleError> 
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
    else { return Ok(fields) }
}

fn match_current_token_to_field_and_parse_field(tokenizer: &mut Tokenizer<impl Read>, token: Token, fields: &mut HashMap<Box<str>, Want>) 
-> Result<bool, CastleError> {
    match token.kind {
        TokenKind::Identifier(identifier) => {
            let should_break = parse_query_field(tokenizer, fields, identifier)?;
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

pub fn parse_query_field<R>(tokenizer: &mut Tokenizer<R>, fields: &mut HashMap<Box<str>, Want>, identifier: Identifier) 
-> Result<bool, CastleError> where R: Read {
    let peeked_token = tokenizer.peek(true)?;
    match peeked_token {
        Some(peeked_token) => match peeked_token.kind {
            TokenKind::Keyword(Keyword::Match) => {
                let name = identifier.name.clone();
                let match_statements = parse_match(tokenizer, identifier)?;
                fields.insert(name, Want::new_match(match_statements)); 
                return Ok(false)
            },
            TokenKind::Punctuator(Punctuator::OpenBlock) => {
                parse_inner_object(tokenizer, fields, identifier)?;
                return Ok(false)
            },
            _ => {
                let arguments = ArgumentOrTuple::convert_arguments_to_identifier_and_value_arguments(identifier.arguments)?;
                let field = Want::SingleField(arguments);
                fields.insert(identifier.name, field);
                return Ok(false)
            }
        },
        None => { return Ok(false) }
    }
}

pub fn parse_match<R>(tokenizer: &mut Tokenizer<R>, identifier: Identifier, ) -> Result<Vec<MatchArm>, CastleError> 
where R: Read {
    tokenizer.next(true)?; // consume the match keyword
    let match_statements = parse_match_statements(tokenizer)?;
    return Ok(match_statements)
}

fn create_obj(identifier: Identifier, fields: HashMap<Box<str>, Want>) -> Result<Want, CastleError> {
    let arguments;
    if identifier.arguments.is_some() {

        arguments = ArgumentOrTuple::convert_arguments_to_identifier_and_value_arguments(identifier.arguments)?;
    } else {
        arguments = HashMap::new();
    }
    let object_projection = Want::new_object_projection(fields, arguments);
    return Ok(object_projection)
}

fn create_err_for_parser(token: &Token) -> Result<Option<Result<Want, CastleError>>, CastleError> {
    let err = Some(Err(CastleError::Parser(
        format!("unexpected token, expected identifier, close block or comma, got {:?}", token.kind).into(),
        token.span
    )));
    return Ok(err)
}