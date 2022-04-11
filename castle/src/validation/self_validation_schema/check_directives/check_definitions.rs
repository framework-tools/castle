use parser_and_schema::ast::syntax_definitions::{schema_definition::SchemaDefinition, directive_definition::{DirectiveDefinition}};
use shared::castle_error::CastleError;

use crate::validation::self_validation_schema::check_type::check_type_used_has_been_defined;


pub(crate) fn validate_directive_definitions(schema: &SchemaDefinition) -> Result<(), CastleError> {
    for (_directive_name, directive_definition) in &schema.directives {
        check_directive_arguments_exist(schema, directive_definition)?;
    }
    return Ok(())
}

fn check_directive_arguments_exist(
    schema: &SchemaDefinition,
    directive_definition: &DirectiveDefinition,
) -> Result<(), CastleError> {
    for (_, (_ident, type_)) in &directive_definition.function.args {
        check_type_used_has_been_defined(schema, type_)?;
    }
    return Ok(())
}

