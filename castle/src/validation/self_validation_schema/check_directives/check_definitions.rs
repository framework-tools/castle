use parser_and_schema::ast::syntax_definitions::{schema_definition::SchemaDefinition, directive_definition::{DirectiveDefinition}};
use shared::castle_error::CastleError;
use crate::validation::self_validation_schema::{check_args::check_args_are_defined};

pub(crate) fn validate_directive_definitions(schema: &SchemaDefinition) -> Result<(), CastleError> {
    for (_directive_name, directive_definition) in &schema.directives {
        check_args_are_defined(schema, &directive_definition.function.args)?;
    }
    return Ok(())
}

