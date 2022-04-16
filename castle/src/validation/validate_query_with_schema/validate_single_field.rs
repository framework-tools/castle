use std::collections::HashMap;

use parser_and_schema::{ast::syntax_definitions::{field_definition::FnDefinition, want::Want, schema_definition::{self, SchemaDefinition}, expressions::PrimitiveValue}, parsers::schema_parser::types::parse_type::Type};
use shared::castle_error::CastleError;

use super::query_resolver_is_valid::{check_arguments_are_compatible, get_resolver};

pub(crate) fn validate_single_field_want(
    identifier: &Box<str>,
    arguments: &HashMap<Box<str>, (Box<str>, PrimitiveValue)>,
    schema_definition: &SchemaDefinition,
) -> Result<(), CastleError> {
    let resolver = get_resolver(schema_definition, identifier)?;
    check_arguments_are_compatible(resolver, arguments)?;
    match resolver.return_type {
        Type::SchemaTypeOrEnum(_) => {
            Err(CastleError::FieldsInReturnTypeDoNotMatchQuery(format!("no fields in return type. Got: '{}' in query ", identifier).into()))
        },
        Type::Void => {
            Err(CastleError::FieldsInReturnTypeDoNotMatchQuery(format!("no fields in return type. Got: '{}' in query ", identifier).into()))
        },
        _ => return Ok(())
    }
}