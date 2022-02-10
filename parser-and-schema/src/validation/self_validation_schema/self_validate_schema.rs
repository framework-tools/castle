use std::collections::HashMap;

use shared::CastleError;

use crate::{parser::schema_parser::types::{type_system::Type, schema_type::SchemaType}, ast::syntax_definitions::{schema_definition::SchemaDefinition, enum_definition::EnumDefinition}};


/// It needs to check every type, enum etc that’s used is defined in the schema.

/// Takes in parsed schema and checks each field for any undefined types
///     - For each Type loop
///     - For each Field in Type Loop
///     - If Field Type is a SchemaOrEnumType, 
///     - check this type is defined in the schema_types (Hashmap)
///     - if it's not, check this type is defined in the enums (Hashmap)
///     - If Type is not defined in either, return Some(error)
///     - Else if no errors found, return None
pub fn self_validate_schema(schema: &SchemaDefinition) -> Result<(), CastleError>{
    for (_schema_type_name, schema_type) in &schema.schema_types {
        for (_field_name, field) in &schema_type.fields {
            match &field.type_ {
                Type::SchemaTypeOrEnum(schema_type_name) => {
                    if !schema.schema_types.contains_key(&*schema_type_name) &&
                    !schema.enums.contains_key(&*schema_type_name) {
                        return Err(CastleError::UndefinedSchemaType(format!("Undefined schema type: {}", schema_type_name).into()));
                    }
                },
                _ => { } 
            }
        }
    }
    return Ok(())
}