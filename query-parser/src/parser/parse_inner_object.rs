use std::io::Read;

use shared::CastleError;

use crate::{tokenizer::tokenizer::Tokenizer, ast::syntax_definitions::want::Want};

use super::parse_object_projection::parse_object_projection;

pub fn parse_inner_object<R>(tokenizer: &mut Tokenizer<R>, fields: &mut Vec<Box<Want>>, name: Box<str>) -> Result<(), CastleError> 
where R: Read {
    tokenizer.next(true)?; // consume the open block
    let object_projection = parse_object_projection(name, tokenizer, false)?;
    fields.push(object_projection.into());
    return Ok(())
}