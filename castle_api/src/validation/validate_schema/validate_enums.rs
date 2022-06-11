use std::collections::HashMap;

use castle_types::{SchemaDefinition, CastleError, EnumDefinition, VariantDefinition, VariantKindDefinition, Kind};

use super::{validate_directives::validate_directive, return_type_exists};


pub(super) fn validate_enums(schema: &SchemaDefinition) -> Result<(), CastleError> {
    for enum_def in schema.enums.values() {
        validate_enum(schema, enum_def)?;
    }
    return Ok(());
}

fn validate_enum(schema: &SchemaDefinition, enum_def: &EnumDefinition) -> Result<(), CastleError> {
    for directive in enum_def.directives.iter() {
        validate_directive(schema, &[&enum_def.ident], directive)?;
    }

    for variant in enum_def.variants.values() {
        validate_variant(schema, &enum_def.ident, variant)?;
    }
    return Ok(());
}

fn validate_variant(schema: &SchemaDefinition, enum_name: &str, variant: &VariantDefinition) -> Result<(), CastleError> {
    for directive in variant.directives.iter() {
        validate_directive(schema, &[&enum_name, &variant.ident], directive)?;
    }

    match &variant.kind {
        VariantKindDefinition::Unit => {},
        VariantKindDefinition::Tuple(tup) => validate_tuple(schema, &enum_name, &variant.ident, &tup)?,
        VariantKindDefinition::Map(map) => validate_map(schema, &enum_name, &variant.ident, &map)?,
    }


    return Ok(());
}

fn validate_tuple(schema: &SchemaDefinition, enum_name: &str, variant_name: &str, tup: &Vec<Kind>) -> Result<(), CastleError> {
    for kind in tup.iter() {
        match return_type_exists(schema, kind) {
            Ok(()) => {}
            Err(e) => Err(CastleError::Validation(format!("{}.{} has invalid kind: {}", enum_name, variant_name, e).into()))?,
        }
    }
    return Ok(())
}

fn validate_map(schema: &SchemaDefinition, enum_name: &str, variant_name: &str, map: &HashMap<Box<str>, Kind>) -> Result<(), CastleError> {
    for (_, value) in map.iter() {
        match return_type_exists(schema, value) {
            Ok(()) => {}
            Err(e) => Err(CastleError::Validation(format!("{}.{} has invalid kind: {}", enum_name, variant_name, e).into()))?,
        };
    }
    return Ok(())
}