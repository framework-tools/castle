use parser_and_schema::ast::syntax_definitions::{match_statement::{MatchStatement, MatchArm}, schema_definition::SchemaDefinition, enum_definition::EnumDataType, want::Want};
use shared::castle_error::CastleError;

pub(crate) fn validate_all_match_arms(
    match_statement: &MatchStatement, 
    schema_definition: &SchemaDefinition
) -> Result<(), CastleError>{
    for match_arm in match_statement {
        validate_enum_used_is_defined_in_schema(match_arm, schema_definition)?;
        validate_match_arm_fields_are_valid_for_return_type(match_arm, schema_definition)?;
    }
    return Ok(())
}

fn validate_enum_used_is_defined_in_schema(
    match_arm: &MatchArm, 
    schema_definition: &SchemaDefinition
) -> Result<(), CastleError>{
    let condition = &match_arm.condition;
    let condition_parent = &condition.enum_parent;
    if !schema_definition.enums.contains_key(condition_parent) {
        return Err(CastleError::EnumInQueryNotDefinedInSchema(format!("Enum: {} not defined in schema", condition_parent).into()));
    } 
    else {
        let enum_definition = schema_definition.enums.get(condition_parent).unwrap();
        let condition_variant = &condition.variant;
        if !enum_definition.variants.contains_key(condition_variant){
            return Err(CastleError::EnumInQueryNotDefinedInSchema(format!("Enum variant: {} not defined in schema", condition_variant).into()));
        }
    }
    return Ok(())
}

/// for match arm enum get enums fields
/// for each field, check it exists in the enum's fields
/// if not, return error
fn validate_match_arm_fields_are_valid_for_return_type(match_arm: &MatchArm, schema_definition: &SchemaDefinition) -> Result<(), CastleError> {
    let condition = &match_arm.condition;
    let enum_definition = schema_definition.enums.get(&condition.enum_parent).unwrap();
    let enum_variant = enum_definition.variants.get(&condition.variant).unwrap();
    match &enum_variant.enum_data_type {
        EnumDataType::EnumUnit => {
            let variant_type = schema_definition.schema_types.get(&enum_variant.name);
            if variant_type.is_none() {
                return Err(CastleError::EnumVariantDoesNotHaveMatchingType(format!("Enum variant not defined as a type in schema: {}", enum_variant.name).into()));
            } else {
                let variant_type = variant_type.unwrap();
                let obj = &match_arm.object;
                match obj {
                    Want::ObjectProjection(fields, _) => {
                        for field in fields.keys() {
                            if !variant_type.fields.contains_key(field) {
                                return Err(CastleError::EnumVariantDoesNotHaveMatchingType(format!("Enum variant not defined as a type in schema: {}", enum_variant.name).into()));
                            }
                        }
                    },
                    _ => {}
                }
            }
        },
        _ => {
            return Err(CastleError::EnumVariantDoesNotHaveMatchingType(format!("Enum field: {:?} uses incorrect data type. expected enum unit", enum_variant.enum_data_type).into()));
        }
    }
    Ok(())
}