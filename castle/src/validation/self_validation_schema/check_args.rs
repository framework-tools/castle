use parser_and_schema::ast::syntax_definitions::{schema_definition::SchemaDefinition, argument::ArgumentOrTuple};
use shared::castle_error::CastleError;

use super::check_type::check_type_used_has_been_defined;

pub(crate) fn check_arguments_or_tuples_are_defined(
    schema: &SchemaDefinition, 
    arguments_or_tuples: &Vec<ArgumentOrTuple>
) -> Result<(), CastleError> {
    for arg_or_tuple in arguments_or_tuples {
        check_argument_is_defined(schema, arg_or_tuple)?;
    }
    return Ok(())
}

pub(crate) fn check_argument_is_defined(schema: &SchemaDefinition, arg_or_tuple: &ArgumentOrTuple) -> Result<(), CastleError> {
    match arg_or_tuple {
        ArgumentOrTuple::IdentifierAndType(ident_and_type) => check_type_used_has_been_defined(
            schema, &
            ident_and_type.1
        ),
        _ => {Ok(())}
    }
}