pub(crate) mod parsers;
pub(crate) mod types;

use std::collections::HashMap;

use castle_error::CastleError;
use parsers::parse_projection::parse_projection_inner;
use tokenizer::Tokenizer;
pub use types::{Field, FieldKind};

pub fn parse_query(query: &str) -> Result<HashMap<Box<str>, Field>, CastleError> {
    let bytes = query.as_bytes();
    let mut tokenizer = Tokenizer::new(bytes);
    let map = parse_projection_inner(&mut tokenizer)?;
    Ok(map)
}