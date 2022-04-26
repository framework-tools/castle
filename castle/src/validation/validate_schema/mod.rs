use castle_error::CastleError;
use schema_parser::types::{SchemaDefinition, Kind};
use validate_types::validate_types;
use validate_enums::validate_enums;
// use validate_directive_definitions::validate_directive_definitions;

// mod validate_directive_definitions;
mod validate_types;
mod validate_directives;
mod validate_enums;

/// It needs to check every type, enum etc that’s used is defined in the schema.
///
/// Currently Testing:
/// - Enums
///     - All enum directives are defined in the schema and match usage and args
///     - All enum variants have valid kinds defined in the schema or `built-in` types
///     - All enum variant directives are defined in the schema and directive structure
/// - Types
///    - All type directives are defined in the schema and match usage and args
///    - All type fields have valid types defined in the schema or `built-in` types
///    - All type directives are defined in the schema and match directive structure
/// - A root query type has been defined in the schema
pub(crate) fn validate_schema(schema: &SchemaDefinition) -> Result<(), CastleError>{
    // validate_directive_definitions(schema)?;
    validate_types(schema)?;
    validate_enums(schema)?;
    // validate_resolvers(schema)?;
    return Ok(())
}

/// Check if the provided type [Kind] exists in the schema, or is a built-in type
/// If the type is not found, it will return a &str reason why it failed
/// If the type is found, it will return void
pub(crate) fn type_exists(schema: &SchemaDefinition, kind: &Kind) -> Result<(), String> {
    Ok(match &*kind.name {
        "Int" if kind.generics.len() == 0 => (),
        "Float" if kind.generics.len() == 0 => (),
        "String" if kind.generics.len() == 0 => (),
        "Boolean" if kind.generics.len() == 0 => (),
        "Uuid" if kind.generics.len() == 0 => (),
        "Vec" => match &kind.generics {
            generics if generics.len() == 1 => type_exists(schema, &generics[0])?,
            _ => Err("Vec type must have 1 generic type")?,
        },
        "Option" => match &kind.generics {
            generics if generics.len() == 1 => type_exists(schema, &generics[0])?,
            _ => Err("Option type must have 1 generic type")?,
        },
        name if kind.generics.len() == 0 => match name {
            name if schema.types.contains_key(name) => (),
            name if schema.enums.contains_key(name) => (),
            _ => Err(format!("Type {} not defined in schema types or enums", name))?,
        }
        _ => Err(format!("Type {} not defined in schema, maybe there is an incorrect number of generics", kind.name))?,
    })
}