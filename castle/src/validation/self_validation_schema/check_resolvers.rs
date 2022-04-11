use parser_and_schema::ast::syntax_definitions::{schema_definition::SchemaDefinition, fn_definition::FnDefinition};
use shared::castle_error::CastleError;

use super::{check_type::check_type_is_valid, check_args::check_args_exist};

/// Checks all functions arguments and return types have been defined
/// Takes in parsed schema
/// - For each function in schema.functions
///    - Match function.arguments
///    - IF none, continue
///    - Else, follow below instructions
///    - Checks all arguments are valid: Call check_arguments_or_tuples_are_defined() - parse in function.args
///    - Checks return type is valid: Call check_type_or_enum_exists(&schema_type_or_enum_name, schema)?; - parse in function.return_type
pub(crate) fn validate_resolvers(schema: &SchemaDefinition) -> Result<(), CastleError> {
    let functions = &schema.functions;
    let result: Result<Vec<()>, CastleError> = functions.into_iter()
        .map(|(_, fn_def)| validate_args_and_return_type(schema, &fn_def))
        .collect();
    result?;
    return Ok(())
}

fn validate_args_and_return_type(
    schema: &SchemaDefinition,
    fn_def: &FnDefinition,
) -> Result<(), CastleError> {
    check_args_exist(schema, &fn_def.args)?;
    check_type_is_valid(schema, &fn_def.return_type)?; //check return type
    return Ok(())
}