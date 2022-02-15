use std::{io::Read, collections::{HashMap}};

use shared::CastleError;

use crate::{ast::syntax_definitions::{want::Want, argument::ArgumentOrTuple}, tokenizer::{tokenizer::Tokenizer}, token::{token::{TokenKind, Punctuator, Identifier}, Token}};

use super::{parse_object_projection::parse_object_projection};

#[derive(Debug)]
pub struct ParsedQuery {
    pub wants: HashMap<Box<str>, Want>
}

/// Parses a query into a set of wants.
/// - get bytes from query string
/// - convert bytes into tokens
/// - convert tokens into a hashset of wants
/// - return hashset of wants(parsed query)
pub fn parse_query(query: &str) -> Result<ParsedQuery, CastleError> {
    let bytes = query.as_bytes();
    let mut tokenizer = Tokenizer::new(bytes);
    let wants = parse_query_tokens (&mut tokenizer)?;
    // check_end_of_file(&mut tokenizer)?; - need to re-implement this correcly
    let parsed_query = ParsedQuery { wants };
    return Ok(parsed_query)
}

/// takes in tokens and returns a hashset of wants (parsed query)
/// - create a empty hashset of wants
/// - loop through tokens
///     - if token is identifier & next token excluding /n is not open block -> add single field want to hashset
///     - if empty break
/// - return hashset of wants
fn parse_query_tokens<R>(tokenizer: &mut Tokenizer<R>) -> Result<HashMap<Box<str>, Want>, CastleError> 
where R: Read {
    let mut wants = HashMap::new();
    let err = None;
    loop {
        let token = tokenizer.next(true)?;
        match token {
            Some(token) => { 
                let want = match_token_to_want(token, tokenizer)?;
                let identifier = want.get_identifier()?;
                wants.insert(identifier, want);
            },
            None => { break; }
        };
    }
    if err.is_some() { return Err(err.unwrap()) }
    else { return Ok(wants) }
}

fn match_token_to_want<R>(token: Token, tokenizer: &mut Tokenizer<R>) -> Result<Want, CastleError>
where R: Read{
    return match token.kind {
        TokenKind::Identifier(identifier) => Ok(match_peeked_token_to_want(identifier, tokenizer)?),
        TokenKind::EnumValue(enum_value) => {
            let name = enum_value.identifier;
            Ok(match_peeked_token_to_want(Identifier { name, arguments: None }, tokenizer)?)
        }
        _ => Err(CastleError::Parser( format!("2. unexpected token, expected identifier, got: {:?}", token.kind).into(), token.span))
    }
}

pub fn match_peeked_token_to_want<R> (identifier: Identifier, tokenizer: &mut Tokenizer<R>) -> Result<Want, CastleError>
where R: Read {
    let next_token = tokenizer.peek(true)?;
    return match next_token {
        Some(next_token) => {
            match &next_token.kind {
                TokenKind::Punctuator(Punctuator::OpenBlock) => {
                    tokenizer.next(true)?;
                    parse_object_projection(identifier, tokenizer, false)
                },
                _ => {
                    let arguments = ArgumentOrTuple::convert_arguments_to_identifier_and_value_arguments(identifier.arguments)?;
                    Ok(Want::new_single_field(identifier.name, arguments))
                }
            }
        },
        None => {
            let arguments = ArgumentOrTuple::convert_arguments_to_identifier_and_value_arguments(identifier.arguments)?;
            Ok(Want::new_single_field(identifier.name, arguments))
        }
    }
}