use castle_error::CastleError;
use schema_parser::types::{SchemaDefinition, Directive, DirectiveLocation};

use crate::validation::join_paths;

pub(super) fn validate_directive(schema: &SchemaDefinition, path: &[&str], directive: &Directive, directive_location: DirectiveLocation) -> Result<(), CastleError> {
    // check that the directive has been defined in the schema
    match schema.directives.get(&directive.name) {
        None => Err(CastleError::SchemaValidation(format!("{} @{} directive has not been defined in the schema", join_paths(path), directive.name).into()))?,
        Some(directive_def) => {
            // check that each directive argument is defined in the schema
            // check that each directive definition argument is defined in the directive
            // check that the directive is allowed on the given directive location
        }
    }

    return Ok(());
}