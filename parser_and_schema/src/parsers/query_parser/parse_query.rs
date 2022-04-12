use std::{io::Read, collections::{HashMap}};



use shared::castle_error::CastleError;

use crate::{ast::syntax_definitions::{want::Want, argument::ArgumentOrTuple, keyword::Keyword}, tokenizer::{tokenizer::Tokenizer}, token::{token::{TokenKind, Punctuator, Identifier}, Token}};

use super::{parse_object_projection::{parse_object_projection, parse_match}};

#[derive(Debug, PartialEq)]
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
    let wants = parse_query_tokens(&mut tokenizer)?;
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
fn parse_query_tokens<R: Read>(tokenizer: &mut Tokenizer<R>) -> Result<HashMap<Box<str>, Want>, CastleError> {
    let mut wants = HashMap::new();
    let err = None;
    loop {
        let token = tokenizer.next(true)?;
        match token {
            Some(token) => { 
                let (identifier, want) = match_token_to_want(token, tokenizer)?;
                wants.insert(identifier, want);
            },
            None => { break; }
        };
    }
    if err.is_some() { return Err(err.unwrap()) }
    else { return Ok(wants) }
}

fn match_token_to_want<R>(token: Token, tokenizer: &mut Tokenizer<R>) -> Result<(Box<str>, Want), CastleError>
where R: Read{
    return match token.kind {
        TokenKind::Identifier(identifier) => {
            let name = identifier.name.clone();
            Ok((name.clone(), match_peeked_token_to_want(identifier, tokenizer)?))
        },
        TokenKind::EnumValue(enum_value) => {
            let name = enum_value.identifier;
            Ok((name.clone(), match_peeked_token_to_want(Identifier { name, arguments: None }, tokenizer)?))
        }
        _ => Err(CastleError::Parser( format!("2. unexpected token, expected identifier, enumvalue: {:?}", token.kind).into(), token.span))
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
                    parse_object_projection(identifier, tokenizer)
                },
                TokenKind::Keyword(Keyword::Match) => {
                    let match_statement = parse_match(tokenizer)?;
                    return Ok(Want::Match(match_statement))
                },
                _ => {
                    let arguments = ArgumentOrTuple::convert_arguments_to_identifier_and_value_arguments(identifier.arguments)?;
                    Ok(Want::new_single_field(arguments))
                }
            }
        },
        None => {
            let arguments = ArgumentOrTuple::convert_arguments_to_identifier_and_value_arguments(identifier.arguments)?;
            Ok(Want::new_single_field(arguments))
        }
    }
}