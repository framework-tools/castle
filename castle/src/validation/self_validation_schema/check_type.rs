use parser_and_schema::{ast::syntax_definitions::{schema_definition::SchemaDefinition, directive_definition::DirectiveOnValue}, parsers::schema_parser::types::{type_system::Type, vec_type::VecType, option_type::OptionType, schema_type::SchemaType, schema_field::SchemaField}};
use shared::castle_error::CastleError;
use super::check_directives::{validate_directives_with_definitions::check_directives_are_valid};

pub(crate) fn validate_types(schema: &SchemaDefinition) -> Result<(), CastleError> {
    let types = &schema.schema_types;
    let result: Result<Vec<()>, CastleError> = types.into_iter()
        .map(|(_, type_)| validate_schema_type(schema, type_))
        .collect();
    result?;
    return Ok(())
}

fn validate_schema_type(schema: &SchemaDefinition, type_: &SchemaType) -> Result<(), CastleError> {
    let fields = &type_.fields;
    let result: Result<Vec<()>, CastleError> = fields.into_iter()
        .map(|(_, field)| validate_types_field(schema, &field))
        .collect();
    result?;
    return Ok(())
}

fn validate_types_field(schema: &SchemaDefinition, field: &SchemaField) -> Result<(), CastleError> {
    check_type_is_valid(schema, &field.type_)?;
    check_directives_are_valid(schema, &field.directives, &DirectiveOnValue::Field)?;
    return Ok(())
}


/// Takes in parsed schema and checks each field for any undefined types
///     - For each Type loop
///     - For each Field in Type Loop
///     - If Field Type is a SchemaOrEnumType, 
///     - check this type is defined in the schema_types (Hashmap)
///     - if it's not, check this type is defined in the enums (Hashmap)
///     - If Type is not defined in either, return Some(error)
///     - Else if no errors found, return None
pub(crate) fn check_type_is_valid(schema: &SchemaDefinition, type_: &Type) -> Result<(), CastleError> {
    match type_ {
        Type::SchemaTypeOrEnum(schema_type_or_enum_name) => check_schema_type_or_enum_exists(&schema_type_or_enum_name, schema)?,
        Type::VecType(VecType { inner_type }) => check_type_is_valid(schema, inner_type)?,
        Type::OptionType(OptionType { inner_type }) => check_type_is_valid(schema, inner_type)?,
        Type::HashMapType(value_type) => check_type_is_valid(schema, &value_type)?,
        _ => {}
    }
    return Ok(())
}

/// If the schema_type_or_enum does not exist in enums or schema types throw error
/// Else return Ok(())
fn check_schema_type_or_enum_exists(schema_type_or_enum_name: &Box<str>, schema: &SchemaDefinition) -> Result<(), CastleError> {
    if !schema.schema_types.contains_key(&*schema_type_or_enum_name) &&
    !schema.enums.contains_key(&*schema_type_or_enum_name) {
        return Err(CastleError::UndefinedTypeOrEnumInSchema(format!("Undefined schema type used: {}", schema_type_or_enum_name).into()));
    }
    return Ok(())
}