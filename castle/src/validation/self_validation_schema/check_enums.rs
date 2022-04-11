use std::collections::HashMap;

use parser_and_schema::{ast::syntax_definitions::{enum_definition::EnumDataType, schema_definition::SchemaDefinition, directive_definition::DirectiveOnValue}, parsers::schema_parser::types::schema_field::SchemaField};
use shared::castle_error::CastleError;

use super::{check_type::check_type_used_has_been_defined, check_directives::validate_directives_with_definitions::check_directives_are_valid};

/// Takes in parsed schema and checks each field for any enum tuple types that are not defined 
///     - For each Enum Defined ,loop
///     - For each Variant in Enum, Loop
///     - If Variant Data Type is a Tuple
///         - For each type in the tuple
///         - If the type is a SchemaOrEnumType => check this type is defined in the schema_types or enums
///             - If it's not, return Error
///     - Else If Variant Data Type is an Object
///         - For each field in the tuple check the type
///         - If the type is a SchemaOrEnumType => check this type is defined in the schema_types or enums
///             - If it's not, return Error
///     - If no error found return Ok(())
pub(crate) fn validate_enums(schema: &SchemaDefinition) -> Result<(), CastleError> {
    for (_enum_name, enum_definition) in &schema.enums {
        for (_variant_name, variant) in &enum_definition.variants {
            check_directives_are_valid(
                schema, 
                &variant.directives, 
                &DirectiveOnValue::EnumVariant
            )?;

            match &variant.enum_data_type {
                EnumDataType::EnumUnit => {
                    if !schema.schema_types.contains_key(&*variant.name){
                        return Err(CastleError::EnumVariantTypeUndefinedInSchema(format!("Enum variant type is undefined in schema: {}", variant.name).into()))
                    }
                },
                _ => {
                    return Err(CastleError::EnumDataTypeNotSupported(format!("Enum data type not supported: {}", variant.name).into()))
                }
            }
        }
    }
    return Ok(())
}

///TODO: Try to reuse code for this
fn check_enum_object_field_types_are_defined(
    schema: &SchemaDefinition, 
    fields: &HashMap<Box<str>, SchemaField>
) -> Result<(), CastleError> {
    for (_field_name, field) in fields {
        check_type_used_has_been_defined(schema, &field.type_)?;
        check_directives_are_valid(schema, &field.directives, &DirectiveOnValue::Field)?;
    }
    return Ok(())
}