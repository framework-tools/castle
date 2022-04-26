use castle_error::CastleError;
use schema_parser::types::{SchemaDefinition, TypeDefinition, FieldDefinition, DirectiveLocation};

use super::{type_exists, validate_directives::validate_directive};

pub(super) fn validate_types(schema: &SchemaDefinition) -> Result<(), CastleError> {
    for type_def in schema.types.values() {
        validate_type(schema, type_def)?;
    }
    return Ok(());
}


/// Validates a type definition.
/// - validates each field
/// - validates each directive applied on the type
fn validate_type(schema: &SchemaDefinition, type_def: &TypeDefinition) -> Result<(), CastleError> {
    for directive in type_def.directives.iter() {
        validate_directive(schema, &[&type_def.ident], directive, DirectiveLocation::EnumDirective)?;
    }

    for field in type_def.fields.values() {
        validate_field(schema, &type_def.ident, field)?;
    }
    return Ok(());
}

// Validates a field definition.
// - validates each directive applied on the field
// - validates field return kind
fn validate_field(schema: &SchemaDefinition, type_name: &str, field: &FieldDefinition) -> Result<(), CastleError> {
    match type_exists(schema, &field.return_kind) {
        Ok(()) => {}
        Err(e) => Err(CastleError::SchemaValidation(format!("{}.{} has invalid return type: {}", type_name, field.name, e).into()))?,
    };

    for directive in field.directives.iter() {
        validate_directive(schema, &[&type_name, &field.name], directive, DirectiveLocation::FieldDirective)?;
    }
    return Ok(());
}