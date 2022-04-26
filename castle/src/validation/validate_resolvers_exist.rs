use std::collections::HashMap;

use castle_error::CastleError;
use schema_parser::types::SchemaDefinition;

use crate::Resolver;

pub(crate) fn validate_resolvers_exist<C, R>(
    parsed_schema: &SchemaDefinition,
    field_resolvers: &HashMap<Box<str>, Resolver<C, R>>,
) -> Result<(), CastleError> {
    for name in parsed_schema.types.keys() {
        if !field_resolvers.contains_key(name) {
            return Err(CastleError::MissingResolver(name.clone()));
        }
    }
    Ok(())
}
