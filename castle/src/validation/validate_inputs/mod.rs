use std::collections::HashMap;

use castle_error::CastleError;
use query_parser::{Input, Inputs};
use schema_parser::types::{SchemaDefinition, InputDefinition, Kind, InputTypeDefinition, InputDefinitions};
use shared_parser::Primitive;

use crate::{Projection};

use super::join_paths;

pub(crate) fn validate_inputs(schema: &SchemaDefinition, query: Projection) -> Result<(), CastleError> {
    unimplemented!()
}

/// we have an [InputDefinition], which has a [input_kind](Kind).
/// we want to validate that the type the user provided as [Input]
/// matches the [input_kind](Kind) of the [InputDefinition].
pub(crate) fn type_check_input_against_input_definition(
    schema: &SchemaDefinition,
    path: &[&str], // used to build error message
    input_def: &InputDefinition,
    input_value: &Input,
) -> Result<(), CastleError> {
    match &input_def.input_kind {
        Kind { ident, .. } if &**ident == "String" =>  match input_value {
            Input::Primitive(Primitive::String(..)) => Ok(()),
            _ => Err(CastleError::Validation(format!(
                "{} expected input of type String but got {:#?}",
                join_paths(path),
                input_value
            ).into()))
        }
        Kind { ident, .. } if &**ident == "number" =>  match input_value {
            Input::Primitive(Primitive::Number(..)) => Ok(()),
            _ => Err(CastleError::Validation(format!(
                "{} expected input of type Int but got {:#?}",
                join_paths(path),
                input_value
            ).into()))
        }
        // TODO: bool
        // TODO: custom types
        // TODO: Uuid
        // TODO: Option
        // TODO: Vec
        // TODO: Enum
        //# eg:
        //# ```text
        //# type CreateUser {
        //#     name: String
        //#     age: number
        //# }
        //# type Query {
        //#     createUser(input: CreateUser): User
        //# }
        //# ```
        Kind { ident, .. } => match schema.input_types.get(ident) {
            Some(input_def) => match input_value {
                Input::Map(map) => {
                    type_check_input_against_input_type_definition(
                        schema,
                        path,
                        &input_def.input_definitions,
                        map,
                    )
                },
                other => Err(CastleError::Validation(format!(
                    "{} expected input of type {:#?} but got {:#?}",
                    join_paths(path),
                    &input_def.input_definitions,
                    other
                ).into()))
            },
            // This case should not happen if we properly validate the schema.
            None => Err(CastleError::Validation(format!("InputTypeDefinition for kind {} not found in schema", input_def.input_kind).into()))
        }
    }
}

/// We want to validate that the user provided all of the required inputs and no
/// additional inputs were provided.
pub(crate) fn type_check_input_against_input_type_definition(
    schema: &SchemaDefinition,
    path: &[&str],
    input_defs: &InputDefinitions,
    map: &Inputs,
) -> Result<(), CastleError> {


    unimplemented!()

}