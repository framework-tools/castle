use castle_error::CastleError;
use schema_parser::types::SchemaDefinition;

use crate::Projection;

pub(crate) fn validate_projection(schema: &SchemaDefinition, projection: &Projection) -> Result<(), CastleError> {
    unimplemented!()
}