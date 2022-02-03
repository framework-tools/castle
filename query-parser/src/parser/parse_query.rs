use std::{io::Read, collections::HashSet};

use shared::CastleError;

use crate::{ast::syntax_definitions::want::Want, tokenizer::tokenizer::Tokenizer};

use super::parse_single_field_want::parse_single_field_want;


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
///     - if token is identifier without object before semi-colon -> add single field want to hashset
///     - if empty break
/// - return hashset of wants
fn parse_tokens<R>(tokenizer: &mut Tokenizer<R>) -> Result<HashSet<Want>, CastleError> 
where R: Read 
{
    let mut wants = HashSet::new();
    loop {
        let token = tokenizer.next(true)?;
        match token {
            Some(token) => {
                let single_field_want = parse_single_field_want(tokenizer, token)?;
                wants.insert(single_field_want);
            },
            None => break
        }
        // let want = parse_token(tokenizer, token)?;
        // wants.insert(want);
    }
    return Ok(wants)
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

