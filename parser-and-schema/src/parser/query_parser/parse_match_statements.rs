use std::{io::Read, collections::HashMap};

use shared::CastleError;

use crate::{tokenizer::tokenizer::Tokenizer, ast::syntax_definitions::want::Want, token::token::{TokenKind, Punctuator, Identifier}};

use super::parse_object_projection::parse_object_projection;

pub fn parse_match_statements<R>(tokenizer: &mut Tokenizer<R>, fields: &mut HashMap<Box<str>, Want>, name: Box<str>) -> Result<(), CastleError> 
where R: Read {
    tokenizer.next(true)?; // consume the match keyword
    tokenizer.next(true)?; // consume the open block
    let match_statements = get_match_arms(tokenizer)?;
    let field = Want::new_projection(name.clone(), None, Some(match_statements));
    fields.insert(name.clone(), field);
    return Ok(())
}

/// Parses a match statement
/// loop through tokens
/// parse object projection for each possible match statement
fn get_match_arms<R>(tokenizer: &mut Tokenizer<R>) -> Result<HashMap<Box<str>, Want>, CastleError>
where R: Read{
    let mut match_statements: HashMap<Box<str>, Want> = HashMap::new();
    loop {
        let token = tokenizer.peek(true)?;
        match token {
            Some(token) => match &token.kind {
                TokenKind::Punctuator(Punctuator::CloseBlock) => {
                    tokenizer.next(true)?; // consume the close block
                    break;
                },
                TokenKind::Identifier(Identifier { name, .. }) => {
                    let name = name.clone();
                    let statement = parse_object_projection(name.clone(), tokenizer, true)?;
                    match_statements.insert(name, statement);
                },
                _ => break
            }
            None => break
        };
    }
    return Ok(match_statements)
}