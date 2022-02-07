use std::collections::HashMap;

use shared::CastleError;

use super::types::{schema_type::SchemaType, schema_field::Type};

/// Takes in parsed schema and checks each field for any undefined types
///     - For each Type loop
///     - For each Field in Type Loop
///     - If Field Type is a SchemaType, check this type is defined in the schema (Hashmap)
///     - If FieldType is not defined, return Some(error)
///     - Else if no errors found, return None
pub fn check_for_undefined_schema_types(schema: &HashMap<Box<str>, SchemaType>) -> Result<(), CastleError> {
    for (_schema_type_name, schema_type) in schema {
        for (_field_name, field) in &schema_type.fields {
            match &field.type_ {
                Type::SchemaTypeOrEnum(schema_type_name) => {
                    if !schema.contains_key(schema_type_name) {
                        return Err(CastleError::UndefinedSchemaType(format!("Undefined schema type: {}", schema_type_name).into()));
                    }
                },
                _ => { }
            }
        }
    }
    return Ok(())
}