use std::{io::Read, collections::HashSet};

use shared::CastleError;

use crate::{ast::syntax_definitions::want::Want, tokenizer::{tokenizer::Tokenizer}, token::{token::{TokenKind, Punctuator}, Token}};

use super::{create_object_projection::create_object_projection};


/// Parses a query into a set of wants.
/// - get bytes from query string
/// - convert bytes into tokens
/// - convert tokens into a hashset of wants
/// - return hashset of wants(parsed query)
pub fn parse_query(query: &str) -> Result<HashSet<Want>, CastleError> {
    let bytes = query.as_bytes();
    let mut tokenizer = Tokenizer::new(bytes);

    let statements = parse_tokens(&mut tokenizer)?;
    check_end_of_file(&mut tokenizer)?;
    Ok(statements)
}

/// takes in tokens and returns a hashset of wants (parsed query)
/// - create a empty hashset of wants
/// - loop through tokens
///     - if token is identifier & next token excluding /n is not open block -> add single field want to hashset
///     - if empty break
/// - return hashset of wants
fn parse_tokens<R>(tokenizer: &mut Tokenizer<R>) -> Result<HashSet<Want>, CastleError> 
where R: Read 
{
    let mut err: Option<Result<HashSet<Want>, CastleError>> = None;
    let mut wants = HashSet::new();
    loop {
        let token = tokenizer.next(true)?;
        println!("Token: {:#?}", &token);
        match token {
            Some(token) => {
                match token.kind {
                    TokenKind::Identifier(identifier) => {wants.insert(match_peeked_token_to_want(identifier, tokenizer)?);}
                    _ => {
                        err = Some(Err(CastleError::Parser( format!("unexpected token, expected identifier, close block or comma, got {:?}", token.kind).into(), token.span)));
                        break;
                    }
                };
            },
            None => break
        }
    };
    
    return get_parse_result(wants, err)
}

fn check_end_of_file<R>(tokenizer: &mut Tokenizer<R>) -> Result<(), CastleError> 
where R: Read {
    let token = tokenizer.peek(false)?;
    if token.is_some() {
        return Err(CastleError::Parser(
            format!("Expected EOF, found: {:?}", token.unwrap().kind()).into(),
            *token.unwrap().span(),
        ));
    }
    else{ return Ok(()) }
}

fn match_peeked_token_to_want<R> (
    identifier: Box<str>,  
    tokenizer: &mut Tokenizer<R>,
) -> Result<Want, CastleError>
where R: Read {
    let next_token = tokenizer.peek(true)?;
    return match next_token {
        Some(next_token) => {
            match &next_token.kind {
                TokenKind::Punctuator(Punctuator::OpenBlock) => {
                tokenizer.next(true)?;
                create_object_projection(identifier, tokenizer)
            },
            _ => Ok(Want::SingleField(identifier.clone()))
            }
        },
        None => { 
            Ok(Want::SingleField(identifier.clone()))
        }
    }
}

fn get_parse_result(wants: HashSet<Want>, err:Option<Result<HashSet<Want>, CastleError>>) -> Result<HashSet<Want>, CastleError> {
    if let Some(err) = err {return err} 
    else {return Ok(wants)}
}