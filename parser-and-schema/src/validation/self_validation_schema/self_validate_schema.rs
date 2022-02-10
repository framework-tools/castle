use std::collections::HashMap;

use shared::CastleError;

use crate::{parser::schema_parser::types::{type_system::Type, schema_type::SchemaType, schema_field::SchemaField}, ast::syntax_definitions::{schema_definition::SchemaDefinition, enum_definition::{EnumDefinition, EnumVariant, EnumDataType}, argument::Argument}};


/// It needs to check every type, enum etc that’s used is defined in the schema.
/// 
/// Currently Testing:
/// - Unknown type on SchemaType field
///     - Schema type not defined or enum type
/// - Enum values (tuple and object) has unknown type
/// - Vec Types has unknown type
/// - Option Types has unknown type
/// - Function arguments has unknown type
/// - Function return value has unknown type
/// - Directive arguments has unknown type

pub fn self_validate_schema(schema: &SchemaDefinition) -> Result<(), CastleError>{
    //check_types_used_in_enum_values_are_defined()
    for (_schema_type_name, schema_type) in &schema.schema_types {
        for (_field_name, field) in &schema_type.fields {
            check_type_used_in_field_has_been_defined(schema, &field.type_)?;
        }
    }
    //end
    check_types_used_in_enum_values_are_defined(schema)?;

    //check_arguments_or_tuples_are_defined() - end
    return Ok(())
}

/// If the schema_type_or_enum does not exist in enums or schema types throw error
/// Else return Ok(())
fn check_type_or_enum_exists(schema_type_or_enum_name: &Box<str>, schema: &SchemaDefinition) -> Result<(), CastleError> {
    if !schema.schema_types.contains_key(&*schema_type_or_enum_name) &&
    !schema.enums.contains_key(&*schema_type_or_enum_name) {
        return Err(CastleError::UndefinedTypeOrEnumInSchema(format!("Undefined schema type used: {}", schema_type_or_enum_name).into()));
    }
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
fn check_type_used_in_field_has_been_defined(schema: &SchemaDefinition, field_type: &Type) -> Result<(), CastleError> {
    match field_type {
        Type::SchemaTypeOrEnum(schema_type_or_enum_name) => {
            check_type_or_enum_exists(&schema_type_or_enum_name, schema)?;
        },
        _ => {}
    }
    return Ok(())
}

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
fn check_types_used_in_enum_values_are_defined(schema: &SchemaDefinition) -> Result<(), CastleError> {
    for (_enum_name, enum_definition) in &schema.enums {
        for (_variant_name, variant) in &enum_definition.variants {
            match &variant.enum_data_type {
                EnumDataType::EnumTuple(tuple_types) => {
                    //check_arguments_or_tuples_are_defined()
                    for tuple_type in tuple_types {
                        match tuple_type {
                            Argument::Type(Type::SchemaTypeOrEnum(type_to_check)) => {
                                check_type_or_enum_exists(&type_to_check, schema)?;
                            },
                            _ => {}
                        }
                    }
                    //end
                },
                EnumDataType::EnumObject(fields) => {
                    ////check_enum_object_field_types_are_defined()
                    check_enum_object_field_types_are_defined(schema, &fields)?;
                }
                EnumDataType::EnumUnit => {}
            };
        }
    }
    return Ok(())
}

fn check_enum_object_field_types_are_defined(schema: &SchemaDefinition, fields: &HashMap<Box<str>, SchemaField>) -> Result<(), CastleError> {
    Ok(for (_, field) in fields {
        match &field.type_ {
            Type::SchemaTypeOrEnum(type_to_check) => {
                check_type_or_enum_exists(&type_to_check, schema)?;
            },
            _ => { }
        }
    })
}