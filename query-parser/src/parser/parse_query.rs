use std::{io::Read, collections::HashSet};

use shared::CastleError;
use crate::{ tokenizer::{tokenizer::Tokenizer, self}, ast::syntax_definitions::want::Want};


pub fn parse_query(query: &str) -> Result<HashSet<Want>, CastleError> {
    let bytes = query.as_bytes();
    let mut tokenizer = Tokenizer::new(bytes);

    let statements = parse_statements(&mut tokenizer)?;

    let token = tokenizer.peek(false)?;
    if token.is_some() {
        return Err(CastleError::Parser(
            format!("Expected EOF, found: {:?}", token.unwrap().kind()).into(),
            *token.unwrap().span(),
        ));
    }

    Ok(statements)
}


fn parse_statements<R>(tokenizer: &mut Tokenizer<R>) -> Result<HashSet<Want>, CastleError> 
where R: Read 
{
    let mut statements = Vec::new();

    Ok(statements)
}
