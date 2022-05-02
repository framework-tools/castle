use castle_error::CastleError;
use query_parser::{FieldKind};
use schema_parser::types::{SchemaDefinition};

use crate::Projection;

use super::{validate_inputs::{type_check_inputs_against_input_definitions}, validate_schema::input_type_exists};


// check name of field in message exists in schema
pub(crate) fn validate_projection(schema: &SchemaDefinition, projection: &Projection) -> Result<(), CastleError> {
    let query = schema.types.get("Query".into()).ok_or(CastleError::Validation("Schema is missing Query type".into()))?;

    for (name, value) in projection {
        let field_def = match query.fields.get(name) {
            Some(definition) => definition,
            None => return Err(CastleError::Validation(format!("Query has no field named: {}", name).into())),
        };

        type_check_inputs_against_input_definitions(schema, &[&*name], &field_def.input_definitions, &value.inputs);

        match input_type_exists(schema, &field_def.return_kind){
            Ok(_) => (),
            Err(e) => return Err(CastleError::Validation(e.into())),
        }
        match &value.kind {
            FieldKind::Field => match field_def.return_kind {
                
            }
            FieldKind::Object(_) => todo!(),
            FieldKind::List(_) => todo!(),
        }

        // type check the inputs of the field to the field_def.input_defs

        // check_field_definition(definition, projection)?;

    }

    
    return Ok(())
}

// fn check_field_definition(definition: &FieldDefinition, projection: &Projection) -> Result<(), CastleError> {
//     input_type_exists()
// }