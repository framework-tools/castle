use parser_and_schema::{ast::syntax_definitions::{enum_definition::{EnumDataType, EnumDefinition, EnumVariant}, schema_definition::SchemaDefinition, directive_definition::DirectiveOnValue}};
use shared::castle_error::CastleError;
use super::{check_directives::validate_directives_with_definitions::check_directives_are_valid};

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
    let enums = &schema.enums;
    let result: Result<Vec<()>, CastleError> = enums.into_iter()
        .map(|(_, enum_)| validate_enum(schema, enum_))
        .collect();
    result?;
    return Ok(())
}

fn validate_enum(
    schema: &SchemaDefinition, 
    enum_: &EnumDefinition
) -> Result<(), CastleError> {
    let variants = &enum_.variants;
    let result: Result<Vec<()>, CastleError> = variants.into_iter()
        .map(|(_, variant)| validate_enum_variant(schema, &variant))
        .collect();
    result?;
    return Ok(())
}

fn validate_enum_variant(
    schema: &SchemaDefinition,
    variant: &EnumVariant
) -> Result<(), CastleError> {
    check_directives_are_valid(schema, &variant.directives, &DirectiveOnValue::EnumVariant)?;
    check_enum_data_type(schema, &variant)?;
    return Ok(())
}

fn check_enum_data_type(
    schema: &SchemaDefinition, 
    variant: &EnumVariant
) -> Result<(), CastleError> {
    match &variant.enum_data_type {
        EnumDataType::EnumUnit => {
            if !schema.schema_types.contains_key(&*variant.name){
                return Err(CastleError::EnumVariantTypeUndefinedInSchema(format!("Enum variant type is undefined in schema: {}", variant.name).into()))
            } else { return Ok(()) }
        },
        _ => {
            return Err(CastleError::EnumDataTypeNotSupported(format!("Enum data type not supported: {}", variant.name).into()))
        }
    }
}