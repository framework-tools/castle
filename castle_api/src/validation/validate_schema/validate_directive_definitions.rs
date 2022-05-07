use castle_error::CastleError;
use castle_schema_parser::types::{SchemaDefinition, DirectiveDefinition};

use super::input_type_exists;


pub(crate) fn validate_directive_definitions(schema: &SchemaDefinition) -> Result<(), CastleError> {
    for directive_definition in schema.directives.values() {
        validate_directive_definition(schema, directive_definition)?;
    }
    return Ok(());
}

fn validate_directive_definition(schema: &SchemaDefinition, directive_definition: &DirectiveDefinition) -> Result<(), CastleError> {
    // for each input on directive we will validate the input_type_exists
    for input in directive_definition.input_definitions.values() {
        match input_type_exists(schema, &input.input_kind) {
            Ok(()) => {}
            Err(e) => Err(CastleError::Validation(format!("directive @{} with {}'s input with type {} does not exist in schema: {}", directive_definition.ident, input.ident, input.input_kind, e).into()))?,
        }
    }
    return Ok(());
}