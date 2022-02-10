use std::{collections::HashMap};

use shared::CastleError;


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
    for_each_schema_type_check_field_type_is_valid(schema)?;
    for_each_enum_check_all_types_in_their_values_are_valid(schema)?;
    for_each_fn_check_arguments_and_return_types_are_valid(schema)?;
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

fn for_each_schema_type_check_field_type_is_valid(schema: &SchemaDefinition) -> Result<(), CastleError> {
    for (_schema_type_name, schema_type) in &schema.schema_types {
        for (_field_name, field) in &schema_type.fields {
            check_type_used_in_field_has_been_defined(schema, &field.type_)?;
            check_directives_use_valid_types(schema, &field.directives)?;
        }
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
        Type::SchemaTypeOrEnum(schema_type_or_enum_name) => check_type_or_enum_exists(&schema_type_or_enum_name, schema)?,
        Type::VecType(VecType { inner_type }) => check_type_used_in_field_has_been_defined(schema, inner_type)?,
        Type::OptionType(OptionType { inner_type }) => check_type_used_in_field_has_been_defined(schema, inner_type)?,
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
fn for_each_enum_check_all_types_in_their_values_are_valid(schema: &SchemaDefinition) -> Result<(), CastleError> {
    for (_enum_name, enum_definition) in &schema.enums {
        for (_variant_name, variant) in &enum_definition.variants {
            match &variant.enum_data_type {
                EnumDataType::EnumTuple(tuple_types) => {
                    check_arguments_or_tuples_are_defined(schema, tuple_types)?;
                },
                EnumDataType::EnumObject(fields) => {
                    check_enum_object_field_types_are_defined(schema, &fields)?;
                }
                EnumDataType::EnumUnit => {}
            };
        }
    }
    return Ok(())
}

fn check_arguments_or_tuples_are_defined(schema: &SchemaDefinition, arguments_or_tuples: &Vec<Argument>) -> Result<(), CastleError> {
    for arg_or_tuple in arguments_or_tuples {
        match arg_or_tuple {
            Argument::Type(Type::SchemaTypeOrEnum(type_to_check)) => {
                check_type_or_enum_exists(&type_to_check, schema)?;
            },
            Argument::Type(type_) => check_type_used_in_field_has_been_defined(schema, &type_)?,
            Argument::IdentifierAndType(_, type_) => check_type_used_in_field_has_been_defined(schema, &type_)?,
            _ => {}
        }
    }
    return Ok(())
}

fn check_enum_object_field_types_are_defined(schema: &SchemaDefinition, fields: &HashMap<Box<str>, SchemaField>) -> Result<(), CastleError> {
    for (_field_name, field) in fields {
        check_type_used_in_field_has_been_defined(schema, &field.type_)?;
    }
    return Ok(())
}

/// Checks args on directives are valid
/// - For directive in directives
/// - Match Some and None case
///     - IF None, continue
///     - If Some:
///     - call check_arguments_or_tuples_are_defined
///    - Return Ok(()) at bottom outside loop
fn check_directives_use_valid_types(schema: &SchemaDefinition, directives: &Vec<DirectiveDefinition>) -> Result<(), CastleError> {
    for directive in directives {
        match &directive.arguments {
            Some(arguments) => {
                check_arguments_or_tuples_are_defined(schema, &arguments)?;
            },
            None => {}
        };
    }
    return  Ok(())
}

/// Checks all functions arguments and return types have been defined
/// Takes in parsed schema
/// - For each function in schema.functions
///    - Match function.arguments
///    - IF none, continue
///    - Else, follow below instructions
///    - Checks all arguments are valid: Call check_arguments_or_tuples_are_defined() - parse in function.args
///    - Checks return type is valid: Call check_type_or_enum_exists(&schema_type_or_enum_name, schema)?; - parse in function.return_type
fn for_each_fn_check_arguments_and_return_types_are_valid(schema: &SchemaDefinition) -> Result<(), CastleError> {
    for (_fn_name, fn_definition) in &schema.functions {
        match &fn_definition.args {
            Some(arguments) => {
                check_arguments_or_tuples_are_defined(schema, arguments)?;
            },
            None => {}
        };
        match &fn_definition.return_type {
            Some(Type::SchemaTypeOrEnum(type_to_check)) => {
                check_type_or_enum_exists(&type_to_check, schema)?;
            },
            _ => {}
        };
    }
    return Ok(())
}
