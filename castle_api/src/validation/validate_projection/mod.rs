
use castle_types::{SchemaDefinition, Projection, CastleError, TypeDefinition, FieldKind, FieldDefinition, Kind};

use super::{validate_inputs::{type_check_inputs_against_input_definitions}, join_paths};


pub(crate) fn validate_projection(schema: &SchemaDefinition, projection: &Projection) -> Result<(), CastleError> {
    let root = schema.types.get("Root".into()).ok_or(CastleError::Validation("Schema is missing Root type".into()))?;
    validate_each_projection_field(schema, projection, root, &["root"])?;
    return Ok(())
}

fn validate_each_projection_field(
    schema: &SchemaDefinition,
    projection: &Projection,
    type_being_validated: &TypeDefinition,
    path: &[&str]
) -> Result<(), CastleError> {
    for (name, value) in projection {
        let field_def = type_being_validated.fields.get(name)
            .ok_or(CastleError::Validation(format!("{} has no field named: {}", join_paths(path), name).into()))?;

        type_check_inputs_against_input_definitions(schema, &[path, &[name]].concat(), &field_def.input_definitions, &value.inputs)?;
        validate_field_kind(&value.kind, schema, field_def, &[path, &[name]].concat())?;
    }
    Ok(())
}

pub(crate)fn validate_field_kind(
    input_kind: &FieldKind,
    schema: &SchemaDefinition,
    field_def: &FieldDefinition,
    path: &[&str]
) -> Result<(), CastleError> {
    match input_kind {
        FieldKind::Field => match is_scalar(&field_def.return_kind) {
            true => Ok(()),
            false => Err(CastleError::Validation(format!("{} is not a scalar type", join_paths(path)).into()))
        },
        FieldKind::Object(projection) => match schema.types.get(&get_nested_option_type(&field_def.return_kind).ident) {
            Some(type_def) => validate_each_projection_field(schema, projection, type_def, path),
            None => Err(CastleError::Validation(format!("{} tried to project an fields on type {}", join_paths(path), field_def.return_kind).into()))
        },
        FieldKind::List(projection) => validate_list(schema, field_def, projection, path),
    }
}

fn get_nested_option_type(kind: &Kind) -> &Kind {
    if &*kind.ident == "Option" {
        get_nested_option_type(&kind.generics[0])
    } else {
        &kind
    }
}

fn validate_list(schema: &SchemaDefinition, field_def: &FieldDefinition, projection: &Projection, path: &[&str]) -> Result<(), CastleError> {
    match &*field_def.return_kind.ident {
        "Vec" => match schema.types.get(&field_def.return_kind.generics[0].ident) {
            Some(type_def) => validate_each_projection_field(schema, projection, type_def, path),
            None => Err(CastleError::Validation(format!("{} tried to project an fields on type {}", join_paths(path), field_def.return_kind).into()))
        },
        _ => Err(CastleError::Validation(format!("{} tried to project an fields on type {}", join_paths(path), field_def.return_kind).into()))
    }
}

fn is_scalar(kind: &Kind) -> bool {
    match &*kind.ident {
        "String" | "number" | "bool" | "void" => true,
        "Vec" if is_scalar(&kind.generics[0]) => true,
        "Option" if is_scalar(&kind.generics[0]) => true,
        _ => false,
    }
}