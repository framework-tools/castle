use parser_and_schema::ast::syntax_definitions::{schema_definition::SchemaDefinition};
use shared::castle_error::CastleError;
use crate::validation::self_validation_schema::{check_args::{check_args_exist}};

pub(crate) fn validate_directive_definitions(schema: &SchemaDefinition) -> Result<(), CastleError> {
    let directives = &schema.directives;
    let result: Result<Vec<()>, CastleError> = directives.into_iter()
        .map(|(_, def)| check_args_exist(schema, &def.args))
        .collect();
    result?;
    return Ok(())
}

