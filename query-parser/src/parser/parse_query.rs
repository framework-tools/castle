use std::{io::Read, collections::HashSet};

use shared::CastleError;

use crate::{ast::syntax_definitions::want::Want, tokenizer::tokenizer::Tokenizer};



pub fn parse_query(query: &str) -> Result<HashSet<Want>, CastleError> {
    let bytes = query.as_bytes();
    let mut tokenizer = Tokenizer::new(bytes);

    let statements = parse_tokens(&mut tokenizer)?;
    check_end_of_file(&mut tokenizer)?;
    Ok(statements)
}


fn parse_tokens<R>(tokenizer: &mut Tokenizer<R>) -> Result<HashSet<Want>, CastleError> 
where R: Read 
{
    let mut wants = Vec::new();

    Ok(wants)
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