use parser_and_schema::ast::syntax_definitions::schema_definition::SchemaDefinition;
use shared::castle_error::CastleError;

use super::check_type::check_type_exists;

/// Checks all functions arguments and return types have been defined
/// Takes in parsed schema
/// - For each function in schema.functions
///    - Match function.arguments
///    - IF none, continue
///    - Else, follow below instructions
///    - Checks all arguments are valid: Call check_arguments_or_tuples_are_defined() - parse in function.args
///    - Checks return type is valid: Call check_type_or_enum_exists(&schema_type_or_enum_name, schema)?; - parse in function.return_type
pub(crate) fn for_each_fn_check_arguments_and_return_types_are_valid(schema: &SchemaDefinition) -> Result<(), CastleError> {
    for (_fn_name, fn_definition) in &schema.functions {
        let arguments = &fn_definition.args;
        for (_name, type_) in arguments.values() {
            check_type_exists(schema, type_)?;
        }
        check_type_exists(schema, &fn_definition.return_type)?; //check return type
    }
    return Ok(())
}