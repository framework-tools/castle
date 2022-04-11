use parser_and_schema::ast::syntax_definitions::schema_definition::SchemaDefinition;
use shared::castle_error::CastleError;

use self::{check_directives::{check_definitions::validate_directive_definitions}, check_enums::{validate_enums}, check_resolvers::for_each_fn_check_arguments_and_return_types_are_valid, check_type::check_type_is_valid};

pub mod check_directives;
pub mod check_type;
pub mod check_enums;
pub mod check_args;
pub mod check_resolvers;

/// It needs to check every type, enum etc that’s used is defined in the schema.
/// 
/// Currently Testing:
/// - Unknown type on SchemaType field
///     - Schema type not defined or enum type
/// - Enum values (tuple and object) has unknown type
/// - Vec Types has unknown type
/// - Option Types has unknown type
/// - Function arguments has unknown type
/// - Function return value has unknown type
/// - Directive arguments has unknown type

pub fn self_validate_schema(schema: &SchemaDefinition) -> Result<(), CastleError>{
    validate_directive_definitions(schema)?;
    check_type_is_valid(schema)?;
    validate_enums(schema)?;
    for_each_fn_check_arguments_and_return_types_are_valid(schema)?;
    return Ok(())
}

