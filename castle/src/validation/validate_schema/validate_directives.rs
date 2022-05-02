use castle_error::CastleError;
use schema_parser::types::{Directive, DirectiveDefinition, DirectiveLocation, SchemaDefinition};

use crate::validation::{join_paths, validate_inputs::{check_for_unspecified_args, check_for_missing_args}};

pub(crate) fn validate_directive(
    schema: &SchemaDefinition,
    path: &[&str],
    directive: &Directive,
    used_at_location: DirectiveLocation,
) -> Result<(), CastleError> {
    let new_path: &[&str] = &[&format!("{} @{}", join_paths(path), directive.ident)];

    // check that the directive has been defined in the schema
    match schema.directives.get(&directive.ident) {
        None => Err(CastleError::Validation(format!(
                "{} directive has not been defined in the schema",
                join_paths(new_path)
            ).into(),
        ))?,
        Some(directive_def) => {
            check_for_unspecified_args(schema, &new_path, &directive_def.input_definitions, &directive.inputs)?;
            check_for_missing_args(&new_path, &directive_def.input_definitions, &directive.inputs)?;
            check_if_directive_location_allowed(&directive_def, &new_path, used_at_location)?;
        }
    }

    return Ok(());
}


/// check that this directive definition allows the given directive location
fn check_if_directive_location_allowed(
    directive_def: &DirectiveDefinition,
    path: &[&str],
    used_at_location: DirectiveLocation,
) -> Result<(), CastleError> {
    match directive_def.locations.contains(&used_at_location) {
        false => Err(CastleError::Validation(
            format!(
                "{} is not a valid directive location used at {} @{}",
                used_at_location,
                join_paths(path),
                directive_def.ident
            ).into(),
        )),
        true => Ok(()),
    }
}
