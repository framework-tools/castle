use std::collections::HashMap;

use castle_error::CastleError;
use tokenizer::Tokenizable;

use crate::Projection;







/// Parses a object projection, except without the {} brackets (so just the fields)
/// ```text
/// {
///     first_name,
///     last_name,
/// }
/// ```
fn parse_projection_inner(tokenizer: &mut impl Tokenizable) -> Result<HashMap<Box<str>, Projection>, CastleError> {
    let mut projections = HashMap::new();

    loop {
        let name = tokenizer.expect_identifier(true)?;
        projections.insert(name.clone(), Projection::Include);
    }
}