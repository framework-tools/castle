
use castle_types::{SchemaDefinition, AppliedDirective, CastleError, DirectiveDefinition};

use crate::validation::{join_paths, validate_inputs::{check_for_unspecified_args, check_for_missing_args}, };

pub(crate) fn validate_directive(
    schema: &SchemaDefinition,
    path: &[&str],
    directive: &AppliedDirective,
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
        }
    }

    return Ok(());
}

