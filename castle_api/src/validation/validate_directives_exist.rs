use std::collections::HashMap;

use castle_types::{Directive, SchemaDefinition, CastleError};




pub(crate) fn validate_directives_exist(
    parsed_schema: &SchemaDefinition,
    directives: &HashMap<Box<str>, Box<dyn Directive>>,
) -> Result<(), CastleError> {
    for name in parsed_schema.directives.keys() {
        if !directives.contains_key(name) {
            return Err(CastleError::MissingDirective(name.clone()));
        }
    }
    Ok(())
}