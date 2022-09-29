
use castle_types::{SchemaDefinition, InputDefinition, Input, CastleError, Kind, Primitive, InputDefinitions, Inputs};

use super::{join_paths, validate_schema::validate_directives::validate_directive};

/// we have an [InputDefinition], which has a [input_kind](Kind).
/// we want to validate that the type the user provided as [Input]
/// matches the [input_kind](Kind) of the [InputDefinition].
pub(crate) fn type_check_input_against_input_definition(
    schema: &SchemaDefinition,
    path: &[&str], // used to build error message
    input_def: &InputDefinition,
    input_value: &Input,
) -> Result<(), CastleError> {
    // TODO: maybe mutate the input to include the default value?

    // we will first check the input kind matches the expected type
    type_check_input_against_expected_type(schema, path, &input_def.input_kind, input_value)?;

    // typecheck each of the input directives
    for input_directive in input_def.directives.iter() {
        validate_directive(
            schema,
            path,
            input_directive,
        )?;
    }

    Ok(())
}
pub(crate) fn type_check_input_against_expected_type(
    schema: &SchemaDefinition,
    path: &[&str], // used to build error message
    expected_kind: &Kind,
    input_value: &Input,
) -> Result<(), CastleError> {
    match input_value {
        // TODO: maybe implement scalar types?
        // TODO: Enum

        Input::Variant(_) if &*expected_kind.ident == "Option" => {} // this should check other enums
        Input::Primitive(Primitive::String(..)) if &*expected_kind.ident == "String" => {}
        Input::Primitive(Primitive::Number(..)) if &*expected_kind.ident == "number" => {}
        Input::Primitive(Primitive::Boolean(..)) if &*expected_kind.ident == "bool" => {}
        Input::List(list) if &*expected_kind.ident == "Vec" => for (index, item) in list.iter().enumerate() {
            type_check_input_against_expected_type(schema, &[&format!("{}[{}]", join_paths(path), index)], &expected_kind.generics[0], item)?;
        },
        Input::Map(map) if let Some(input_def) = schema.input_types.get(&expected_kind.ident) =>
            type_check_inputs_against_input_definitions(
                schema,
                path,
                &input_def.input_definitions,
                map,
            )?,
        input_value => Err(CastleError::Validation(format!(
            "{} expected input of type {} but got {}",
            join_paths(path),
            expected_kind,
            input_value
        ).into()))?,
    }

    Ok(())
}

/// We want to validate that the user provided all of the required inputs and no
/// additional inputs were provided.
/// - check for missing inputs
/// - for each input
///     - check for extra inputs that were not specified
///     - [type_check_input_against_input_definition]
pub(crate) fn type_check_inputs_against_input_definitions(
    schema: &SchemaDefinition,
    path: &[&str],
    input_defs: &InputDefinitions,
    map: &Inputs,
) -> Result<(), CastleError> {
    check_for_unspecified_args(schema, path, input_defs, map)?;
    check_for_missing_args(path, input_defs, map)?;
    Ok(())
}

/// check that there are no unspecified args being used in the inputs
/// that were not defined in the [InputDefinitions]
/// in other words, check that each [Input] was defined in the [InputDefinition]
pub(crate) fn check_for_unspecified_args(
    schema: &SchemaDefinition,
    path: &[&str],
    input_defs: &InputDefinitions,
    inputs_map: &Inputs,
) -> Result<(), CastleError> {
    for (arg_ident, input_value) in inputs_map.iter() {
        match input_defs.get(arg_ident) {
            None => Err(CastleError::Validation(
                format!(
                    "{} was not specified in input definition",
                    join_paths(&[path, &[arg_ident]].concat()),
                )
                .into(),
            ))?,
            Some(input_def) => type_check_input_against_input_definition(
                schema,
                &[path, &[&**arg_ident]].concat(),
                input_def,
                input_value,
            )?,
        }
    }

    return Ok(());
}


/// check that all args defined in the [InputDefinitions] were used in the [Inputs]
/// and that none were missing
pub(crate) fn check_for_missing_args(
    path: &[&str],
    input_defs: &InputDefinitions,
    inputs_map: &Inputs,
) -> Result<(), CastleError> {
    for (arg_ident, _) in input_defs.iter() {
        match inputs_map.get(&**arg_ident) {
            None => Err(CastleError::Validation(
                format!(
                    "{} was not specified in input, but is required",
                    join_paths(&[path, &[arg_ident]].concat()),
                )
                .into(),
            ))?,
            Some(..) => {}
        }
    }

    return Ok(());
}