use castle_error::CastleError;
use schema_parser::types::SchemaDefinition;

use crate::Projection;

pub(crate) fn validate_query(schema: &SchemaDefinition, query: Projection) -> Result<(), CastleError> {
    unimplemented!()
}