use parser_and_schema::ast::syntax_definitions::schema_definition::SchemaDefinition;
use shared::castle_error::CastleError;

use self::{check_directives::{check_definitions::validate_directive_definitions}, check_enums::{validate_enums}, check_resolvers::{validate_resolvers}, check_type::{validate_types}};

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
    validate_types(schema)?;
    validate_enums(schema)?;
    validate_resolvers(schema)?;
    return Ok(())
}

