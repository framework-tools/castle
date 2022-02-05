use std::{io::Read, collections::HashMap};

use shared::CastleError;

use crate::{tokenizer::tokenizer::Tokenizer, ast::syntax_definitions::want::Want};

use super::parse_object_projection::parse_object_projection;

pub fn parse_inner_object<R>(tokenizer: &mut Tokenizer<R>, fields: &mut HashMap<Box<str>, Want>, name: Box<str>) -> Result<(), CastleError> 
where R: Read {
    tokenizer.next(true)?; // consume the open block
    let object_projection = parse_object_projection(name.clone(), tokenizer, false)?;
    fields.insert(name, object_projection);
    return Ok(())
}