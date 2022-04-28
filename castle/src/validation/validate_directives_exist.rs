use std::collections::HashMap;

use castle_error::CastleError;
use schema_parser::types::SchemaDefinition;

use crate::Directive;


pub(crate) fn validate_directives_exist<Ctx>(
    parsed_schema: &SchemaDefinition,
    directives: &HashMap<Box<str>, Box<dyn Directive<Ctx>>>,
) -> Result<(), CastleError> {
    for name in parsed_schema.directives.keys() {
        if !directives.contains_key(name) {
            return Err(CastleError::MissingDirective(name.clone()));
        }
    }
    Ok(())
}