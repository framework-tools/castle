use std::collections::HashMap;

use castle_types::{SchemaDefinition, CastleError};

pub(crate) fn validate_resolvers_exist(
    parsed_schema: &SchemaDefinition,
    // field_resolvers: &HashMap<Box<str>, Box<dyn Resolver>>,
) -> Result<(), CastleError> {
    // match parsed_schema.types.get("Root") {
    //     Some(query_type) => {
    //         for field_name in query_type.fields.keys() {
    //             if !field_resolvers.contains_key(field_name) {
    //                 Err(CastleError::MissingResolver(
    //                     format!("Missing resolver for Root.{}", field_name).into(),
    //                 ))?;
    //             }
    //         }
    //         Ok(())
    //     },
    //     None => Err(CastleError::MissingResolver("Missing `type Root` root type".into()))?,
    // }
    Ok(())
}
