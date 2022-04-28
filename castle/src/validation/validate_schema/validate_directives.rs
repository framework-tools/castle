use castle_error::CastleError;
use schema_parser::types::{Directive, DirectiveDefinition, DirectiveLocation, SchemaDefinition};

use crate::validation::{join_paths, validate_inputs::type_check_input_against_input_definition};

pub(super) fn validate_directive(
    schema: &SchemaDefinition,
    path: &[&str],
    directive: &Directive,
    used_at_location: DirectiveLocation,
) -> Result<(), CastleError> {
    // check that the directive has been defined in the schema
    match schema.directives.get(&directive.ident) {
        None => Err(CastleError::Validation(
            format!(
                "{} @{} directive has not been defined in the schema",
                join_paths(path),
                directive.ident
            )
            .into(),
        ))?,
        Some(directive_def) => {
            // check that there are no unspecified args being used on the directive
            // that were not defined in the DirectiveDefinition
            // in other words, check each `Directive`s input to check if the inputs
            // were specified in the definition
            for (arg_name, input_value) in directive.inputs.iter() {
                match directive_def.input_definitions.get(arg_name) {
                    None => Err(CastleError::Validation(
                        format!(
                            "{} @{} used arg: {} but was not found in directive definition",
                            join_paths(path),
                            directive.ident,
                            arg_name
                        )
                        .into(),
                    ))?,
                    Some(input_def) => type_check_input_against_input_definition(
                        schema,
                        path,
                        input_def,
                        input_value,
                    )?,
                }
            }

            // TODO: check that each directive definition argument is defined in the directive

            // check that the directive is allowed on the given directive location
            validate_directive_location(directive_def, path, used_at_location)?;
        }
    }

    return Ok(());
}

/// check that this directive definition allows the given directive location
fn validate_directive_location(
    directive_def: &DirectiveDefinition,
    path: &[&str],
    used_at_location: DirectiveLocation,
) -> Result<(), CastleError> {
    if !directive_def.locations.contains(&used_at_location) {
        Err(CastleError::Validation(
            format!(
                "{} is not a valid directive location used at {} @{}",
                used_at_location,
                join_paths(path),
                directive_def.ident
            )
            .into(),
        ))?
    }

    return Ok(());
}
