use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::{schema_definition::SchemaDefinition, argument::{ArgumentOrTuple, IdentifierAndTypeArgument}};
use shared::castle_error::CastleError;

use super::check_type::check_type_exists;

pub(crate) fn check_args_are_defined(
    schema: &SchemaDefinition, 
    args: &HashMap<Box<str>, IdentifierAndTypeArgument>
) -> Result<(), CastleError> {
    let result: Result<Vec<()>, CastleError> = args.into_iter()
        .map(|(_, (_, type_))| check_type_exists(schema, type_))
        .collect();

    return match result {
        Result::Ok(_) => Ok(()),
        Result::Err(err) => Err(err)
    }
}