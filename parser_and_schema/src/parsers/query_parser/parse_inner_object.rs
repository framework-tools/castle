use std::{io::Read, collections::HashMap};

use shared::CastleError;

use crate::{tokenizer::tokenizer::Tokenizer, ast::syntax_definitions::want::Want, token::token::Identifier};

use super::parse_object_projection::parse_object_projection;

pub fn parse_inner_object<R>(tokenizer: &mut Tokenizer<R>, fields: &mut HashMap<Box<str>, Want>, identifier: Identifier) -> Result<(), CastleError> 
where R: Read {
    tokenizer.next(true)?; // consume the open block
    let name = identifier.name.clone();
    let object_projection = parse_object_projection(identifier, tokenizer, false)?;
    fields.insert(name, object_projection);
    return Ok(())
}