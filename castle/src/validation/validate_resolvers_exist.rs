use std::collections::HashMap;

use castle_error::CastleError;
use schema_parser::types::SchemaDefinition;

use crate::Resolver;

pub(crate) fn validate_resolvers_exist<Ctx>(
    parsed_schema: &SchemaDefinition,
    field_resolvers: &HashMap<Box<str>, Resolver<Ctx>>,
) -> Result<(), CastleError> {
    match parsed_schema.types.get("Root") {
        Some(query_type) => {
            for field_name in query_type.fields.keys() {
                if !field_resolvers.contains_key(field_name) {
                    Err(CastleError::MissingResolver(
                        format!("Missing resolver for Root.{}", field_name).into(),
                    ))?;
                }
            }
            Ok(())
        },
        None => Err(CastleError::MissingResolver("Missing `type Root` root type".into()))?,
    }
}
