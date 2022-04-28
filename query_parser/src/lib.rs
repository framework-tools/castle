pub(crate) mod parsers;
pub(crate) mod types;

use castle_error::CastleError;
use parsers::parse_projection::parse_projection_inner;
use tokenizer::{Tokenizer, Tokenizable};
pub use types::{Field, FieldKind, Projection, Inputs, Input};

pub fn parse_query(query: &str) -> Result<Projection, CastleError> {
    let bytes = query.as_bytes();
    let mut tokenizer = Tokenizer::new(bytes);
    let map = parse_projection_inner(&mut tokenizer)?;
    // check that the tokenizer is at EOF
    match tokenizer.next(true)? {
        Some(token) => Err(CastleError::Query(
            format!("Expected identifier or EOF, got: {:?}", token.kind).into(),
            token.span
        )),
        None => Ok(map),
    }
}