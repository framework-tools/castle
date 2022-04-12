use std::collections::HashMap;
use parser_and_schema::{ast::syntax_definitions::{fn_definition::FnDefinition, schema_definition::SchemaDefinition, want::{Wants, Want}, expressions::PrimitiveValue}, parsers::schema_parser::types::schema_field::SchemaField};
use shared::castle_error::CastleError;

use super::{query_resolver_is_valid::{get_resolver, check_arguments_are_compatible, get_resolvers_return_fields}, validate_wants};

pub(crate) fn validate_object_projection_want(
    identifier: &Box<str>, 
    arguments: &HashMap<Box<str>, (Box<str>, PrimitiveValue)>,
    schema_definition: &SchemaDefinition, 
    wants: &Wants
) -> Result<(), CastleError>{
    let resolver = get_resolver(schema_definition, identifier)?;
    check_arguments_are_compatible(resolver, &arguments)?;
    let return_types_fields = get_resolvers_return_fields(resolver, schema_definition)?;
    validate_wants(wants, schema_definition, Some(return_types_fields))?;
    return Ok(())
}

pub(crate) fn if_inside_object_projection_check_field_exists_on_type(
    identifier: &Box<str>, 
    fields_to_compare: Option<&HashMap<Box<str>, SchemaField>>
) -> Result<(), CastleError> {
    return match fields_to_compare {
        Some(fields) if !fields.contains_key(identifier) =>
            Err(CastleError::FieldsInReturnTypeDoNotMatchQuery(format!("no fields in return type. Got: '{}' in query ", identifier).into())),
        _ => Ok(())
    }
}