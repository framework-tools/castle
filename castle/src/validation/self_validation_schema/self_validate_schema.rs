use std::{collections::HashMap};

use parser_and_schema::{ast::syntax_definitions::{schema_definition::SchemaDefinition, enum_definition::EnumDataType, directive_definition::{Directive, DirectiveDefinition}, argument::{ArgumentOrTuple, IdentifierAndTypeArgument}}, parsers::schema_parser::types::{type_system::Type, vec_type::VecType, option_type::OptionType, schema_field::SchemaField}};
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
            check_type_used_has_been_defined(schema, &field.type_)?;
            check_directives_use_valid_types(schema, &field.directives)?;
            check_directives_use_valid_directive_definitions(&schema.directives, &field.directives, schema)?;
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
fn check_type_used_has_been_defined(schema: &SchemaDefinition, type_: &Type) -> Result<(), CastleError> {
    match type_ {
        Type::SchemaTypeOrEnum(schema_type_or_enum_name) => check_type_or_enum_exists(&schema_type_or_enum_name, schema)?,
        Type::VecType(VecType { inner_type }) => check_type_used_has_been_defined(schema, inner_type)?,
        Type::OptionType(OptionType { inner_type }) => check_type_used_has_been_defined(schema, inner_type)?,
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
            check_directives_use_valid_directive_definitions(&schema.directives, &variant.directives, schema)?;
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

fn check_arguments_or_tuples_are_defined(schema: &SchemaDefinition, arguments_or_tuples: &Vec<ArgumentOrTuple>) -> Result<(), CastleError> {
    for arg_or_tuple in arguments_or_tuples {
        check_argument_is_defined(schema, arg_or_tuple)?;
    }
    return Ok(())
}

fn check_argument_is_defined(schema: &SchemaDefinition, arg_or_tuple: &ArgumentOrTuple) -> Result<(), CastleError> {
    match arg_or_tuple {
        ArgumentOrTuple::IdentifierAndType(ident_and_type) => check_type_used_has_been_defined(schema, &ident_and_type.1),
        _ => {Ok(())}
    }
}

fn check_enum_object_field_types_are_defined(schema: &SchemaDefinition, fields: &HashMap<Box<str>, SchemaField>) -> Result<(), CastleError> {
    for (_field_name, field) in fields {
        check_type_used_has_been_defined(schema, &field.type_)?;
        check_directives_use_valid_directive_definitions(&schema.directives, &field.directives, schema)?;
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
fn check_directives_use_valid_types(schema: &SchemaDefinition, directives: &Vec<Directive>) -> Result<(), CastleError> {
    // for directive in directives {
    //     for (_ident,arg ) in directive.arguments.values() {
    //         check_argument_is_defined(schema, arg)?;
    //     }
    //     check_arguments_or_tuples_are_defined(schema, &directive.arguments)?;
    // }
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
        let arguments = &fn_definition.args;
        for (_name, type_) in arguments.values() {
            check_type_used_has_been_defined(schema, type_)?;
        }

        match &fn_definition.return_type {
            Type::SchemaTypeOrEnum(type_to_check) => {
                check_type_or_enum_exists(&type_to_check, schema)?;
            },
            _ => {}
        };
    }
    return Ok(())
}

fn check_directives_use_valid_directive_definitions(directive_definitions: &HashMap<Box<str>, DirectiveDefinition>, directives: &Vec<Directive>, schema: &SchemaDefinition) -> Result<(), CastleError> {
    for directive in directives {
        if !directive_definitions.contains_key(&directive.name) {
            return Err(CastleError::UndefinedDirective(format!("Directive {} is not defined", &directive.name).into()));
        }
        else {
            validate_directive_definition_arguments_and_directive_arguments(directive, directive_definitions, schema)?;
            

        }
    }
    return Ok(())
}

fn validate_directive_definition_arguments_and_directive_arguments(directive: &Directive, directive_definitions: &HashMap<Box<str>, DirectiveDefinition>, schema: &SchemaDefinition) -> Result<(), CastleError> {
    let directive_definition = &directive_definitions[&directive.name].function;

    let directive_definition = directive_definition;
    for (_name, type_) in directive_definition.args.values() {
        check_type_used_has_been_defined(schema, type_)?;
    }

    let directive_definition_args = &directive_definition.args;
    let directive_args = &directive.arguments;
    for arg in directive_definition_args.keys() {
        if !directive_args.contains_key(arg) {
            return Err(CastleError::DirectiveDoesNotMatchSchemaDirective(format!("Directive {} does not have argument {:?}", &directive.name, &arg).into()));
        }
    }
    return Ok(())
}