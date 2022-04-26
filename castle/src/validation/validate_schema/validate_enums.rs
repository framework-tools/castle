use castle_error::CastleError;
use schema_parser::types::{EnumDefinition, SchemaDefinition, VariantDefinition, DirectiveLocation};

use super::validate_directives::validate_directive;


pub(super) fn validate_enums(schema: &SchemaDefinition) -> Result<(), CastleError> {
    for enum_def in schema.enums.values() {
        validate_enum(schema, enum_def)?;
    }
    return Ok(());
}

fn validate_enum(schema: &SchemaDefinition, enum_def: &EnumDefinition) -> Result<(), CastleError> {
    for directive in enum_def.directives.iter() {
        validate_directive(schema, &[&enum_def.ident], directive, DirectiveLocation::EnumDirective)?;
    }

    for variant in enum_def.variants.values() {
        validate_variant(schema, &enum_def.ident, variant)?;
    }
    return Ok(());
}

fn validate_variant(schema: &SchemaDefinition, enum_name: &str, variant: &VariantDefinition) -> Result<(), CastleError> {
    for directive in variant.directives.iter() {
        validate_directive(schema, &[&enum_name, &variant.ident], directive, DirectiveLocation::VariantDirective)?;
    }

    // TODO: validate each enum variant type (map, unit, tuple) and check that each type is defined in the schema

    return Ok(());
}
