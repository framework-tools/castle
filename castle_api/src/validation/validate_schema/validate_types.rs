

use castle_types::{SchemaDefinition, CastleError, TypeDefinition, DirectiveLocation, FieldDefinition};

use super::{validate_directives::validate_directive, return_type_exists, input_type_exists};

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
        validate_directive(schema, &[&type_def.ident], directive, DirectiveLocation::EnumDefinition)?;
    }

    for field in type_def.fields.values() {
        validate_field(schema, &type_def.ident, field)?;
    }
    return Ok(());
}

// Validates a field definition.
// - validates each directive applied on the field
// - validates each input on the field
// - validates field return kind
fn validate_field(schema: &SchemaDefinition, type_name: &str, field: &FieldDefinition) -> Result<(), CastleError> {
    match return_type_exists(schema, &field.return_kind) {
        Ok(()) => {}
        Err(e) => Err(CastleError::Validation(format!("{}.{} has invalid return type: {}", type_name, field.ident, e).into()))?,
    };

    for input_def in field.input_definitions.values() {
        match input_type_exists(schema, &input_def.input_kind) {
            Ok(()) => {}
            Err(e) => Err(CastleError::Validation(format!("{}.{} input {} with type {} does not exist in schema: {}", type_name, field.ident, input_def.ident, input_def.input_kind, e).into()))?,
        }
    }

    for directive in field.directives.iter() {
        validate_directive(schema, &[&type_name, &field.ident], directive, DirectiveLocation::FieldDefinition)?;
    }

    return Ok(());
}