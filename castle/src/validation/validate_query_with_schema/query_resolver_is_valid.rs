use std::collections::HashMap;
use parser_and_schema::{ast::syntax_definitions::{schema_definition::SchemaDefinition, field_definition::FnDefinition, want::{WantArguments}, argument::{IdentifierAndTypeArgument, IdentifierAndValueArgument}}, parsers::schema_parser::types::{parse_type::Type, schema_field::SchemaField}};
use shared::castle_error::CastleError;

use super::validate_value_to_type::check_type_and_value_are_compatible;

pub(crate) fn get_resolver<'a>(
    schema_definition: &'a SchemaDefinition, 
    identifier: &Box<str>
) -> Result<&'a FnDefinition, CastleError>{
    let resolver = schema_definition.functions.get(identifier);
    return match resolver {
        Some(r) => Ok(r),
        None => Err(CastleError::QueryResolverNotDefinedInSchema(format!("no matching resolver found in schema. Got: '{}' in query ", identifier).into())),
    }
}

pub(crate) fn check_arguments_are_compatible(resolver: &FnDefinition, arguments: &WantArguments) -> Result<(), CastleError> {
    check_if_arguments_in_query_have_different_lengths(&resolver.args, &arguments)?;
    iterate_through_arguments_and_check_compatibility(&resolver.args, &arguments)?;
    Ok(())
}

pub(crate) fn get_resolvers_return_fields<'a>(
    resolver: &FnDefinition,
    schema_definition: &'a SchemaDefinition
) -> Result<&'a HashMap<Box<str>, SchemaField>, CastleError> {
    let return_type = check_return_type_is_schema_type_or_enum(&resolver.return_type)?;
    let schema_type = schema_definition.schema_types.get(return_type);    // use resolver.return_type to get the schema_type from schema_definition.schema_types
    let type_ = match schema_type {
        Some(t) => t,
        _ => return Err(CastleError::FieldsInReturnTypeDoNotMatchQuery(format!("no matching schema type found in schema. Got: '{}' in query ", return_type).into())),
    };
    return Ok(&type_.fields)
}

pub(crate) fn check_return_type_is_schema_type_or_enum(return_type: &Type) -> Result<&Box<str>, CastleError> {
    return match return_type {
        Type::SchemaTypeOrEnum(schema_type_or_enum) => Ok(schema_type_or_enum),
        _ => Err(CastleError::NoIdentifierOnObjectProjection(format!(" on resolver return type Not valid. Type: {:?}", return_type).into())),
    }
}


pub(crate) fn check_if_arguments_in_query_have_different_lengths(
    resolver_args: &HashMap<Box<str>, IdentifierAndTypeArgument>, 
    query_args: &HashMap<Box<str>, IdentifierAndValueArgument>
) -> Result<(), CastleError> {  
    //check args are compatible
    if query_args.len() != resolver_args.len() {
        return Err(CastleError::ArgumentsInQueryDoNotMatchResolver(format!("arguments in query have different lengths. Query args length: {}, resolver args length: {}", 
        query_args.len(), resolver_args.len()).into()));
    }
    else {
        Ok(())
    }
}

pub(crate) fn iterate_through_arguments_and_check_compatibility(resolver_args: &HashMap<Box<str>, IdentifierAndTypeArgument>, query_args: &HashMap<Box<str>, IdentifierAndValueArgument>)
-> Result<(), CastleError> {
    for (query_arg_name, query_arg_value) in query_args.values() {
        let arg_in_resolver = resolver_args.get(query_arg_name);
        if arg_in_resolver.is_none() {
            return Err(CastleError::QueryResolverNotDefinedInSchema(format!("no argument found in resolver with matching identifier: {}", query_arg_name).into()));
        }
        else {
            let arg_in_resolver = arg_in_resolver.as_ref().unwrap();
            check_type_and_value_are_compatible(&arg_in_resolver.1, query_arg_value)?;
        }
        let arg_in_resolver = arg_in_resolver.unwrap();
        let compatible = check_type_and_value_are_compatible(&arg_in_resolver.1, &query_arg_value)?;
        if !compatible { return Err(CastleError::ArgumentsInQueryDoNotMatchResolver(
            format!("for argument with identifier: {}, type in schema: {:?}, and value in query: {:?} are not compatible", query_arg_name, arg_in_resolver.1, query_arg_value).into())); }
    }
    Ok(())
}