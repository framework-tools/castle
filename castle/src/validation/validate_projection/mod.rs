use castle_error::CastleError;
use query_parser::FieldKind;
use schema_parser::types::{SchemaDefinition, FieldDefinition, TypeDefinition, Kind};
use crate::Projection;
use super::{validate_inputs::{type_check_inputs_against_input_definitions}};


pub(crate) fn validate_projection(schema: &SchemaDefinition, projection: &Projection) -> Result<(), CastleError> {
    let root = schema.types.get("Root".into()).ok_or(CastleError::Validation("Schema is missing Root type".into()))?;
    validate_each_projection_field(schema, projection, root)?;
    return Ok(())
}

// for each field in projection:
// -> check field has been defined in schema & get its definition
// -> validate its inputs against it's definitions inputs
// -> match the field:
//     if Single Field -> check the field is defined in the type
//     if Object -> recursively validate each field
fn validate_each_projection_field(
    schema: &SchemaDefinition, 
    projection: &Projection,
    type_being_validated: &TypeDefinition,
) -> Result<(), CastleError> {
    for (name, value) in projection {
        let field_def = get_field_definition(type_being_validated, name)?;
        type_check_inputs_against_input_definitions(schema, &[&*name], &field_def.input_definitions, &value.inputs)?;
        validate_field_kind(&value.kind, schema, name, type_being_validated, field_def)?;
    }
    return Ok(())
}

fn get_field_definition<'a>(root: &'a TypeDefinition, name: &Box<str>) -> Result<&'a FieldDefinition, CastleError> {
    return match root.fields.get(name) {
        Some(definition) => Ok(definition),
        None => Err(CastleError::Validation(format!("Root has no field named: {}", name).into())),
    }
}

fn validate_field_kind(kind: &FieldKind, schema: &SchemaDefinition, name: &Box<str>, type_being_validated: &TypeDefinition, field_def: &FieldDefinition) -> Result<(), CastleError> {
    return match &kind {
        FieldKind::Field => field_exists_in_type(name, type_being_validated),
        FieldKind::Object(projection) => validate_nested_projection(schema, &field_def.return_kind.ident, projection),
        FieldKind::List(projection) => validate_list(schema, field_def, projection),
    }
}

fn field_exists_in_type(field_name: &Box<str>, type_being_validated: &TypeDefinition) -> Result<(), CastleError> {
    match type_being_validated.fields.get(field_name) {
        Some(_) => Ok(()),
        None => Err(CastleError::Validation(format!("Type {} does not have a field named {}", type_being_validated.ident, field_name).into())),
    }
}

fn validate_nested_projection(schema: &SchemaDefinition, type_ident: &Box<str>, projection: &Projection) -> Result<(), CastleError> {
    let type_def = get_type_definition(schema, type_ident)?;
    validate_each_projection_field(schema, projection, type_def)
}


fn get_type_definition<'a>(schema: &'a SchemaDefinition, ident: &Box<str>) -> Result<&'a TypeDefinition, CastleError> {
    schema.types.get(ident).ok_or(CastleError::Validation(format!("{} is not defined in schema", ident).into()))
}

fn validate_list(schema: &SchemaDefinition, field_def: &FieldDefinition, projection: &Projection) -> Result<(), CastleError> {
    return match &*field_def.return_kind.ident {
        "Vec" => {
            let vec_inner_type = &field_def.return_kind.generics[0];
            if is_primitive_or_string_type(vec_inner_type) { return Ok(()) }
            validate_nested_projection(schema, &vec_inner_type.ident, projection)
        },
        _ => Err(CastleError::Validation(format!("Expected Vec kind. Got: {}", field_def.return_kind.ident).into()))?,
    }
}

fn is_primitive_or_string_type(kind: &Kind) -> bool {
    match &*kind.ident {
        "String" | "number" | "bool" => true,
        _ => false,
    }
}