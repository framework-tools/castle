use std::{collections::HashMap};

use parser_and_schema::{parsers::{query_parser::parse_query::ParsedQuery, schema_parser::types::{ type_system::Type, primitive_type::PrimitiveType}}, ast::syntax_definitions::{schema_definition::SchemaDefinition, want::{ Want, ObjectProjection}, expressions::PrimitiveValue, fn_definition::FnDefinition, argument::{IdentifierAndTypeArgument, IdentifierAndValueArgument}}};
use shared::CastleError;

/// Cross-Validation Between Query Parser & Schema Parser
/// Needs to validate that the current schema and the query are compatible
/// To do this it needs to validate:
/// - Resolvers(object projections) in query are defined in the schema
/// - They argument in the query is compatible with the argument type in schema definition
/// - The query (object projection) should be the same or
///    a subset of the schema. (Essentially, can't have a field not defined in the return type)
/// 
/// For example:
/// Query = "               Schema = "
///     me(542) {             fn me(id: Int) -> User
///         first_name
///         last_name         type User {
///         age                 id: uuid,     
///     }                       first_name: String,
/// ";                          last_name: String,
///                             alias: Option<String>,
///                             age: Option<Int>,
///                             email: String,
///                             password: String,
///                           }
///                         ";
/// Steps:
/// - For each Want in the query:
///    - match want to want to single field (leave this empty for now) & object projection
///    - check if the want is defined in the schema
///    (HashMap.get(want.identifier))
///    - If Some value, unwrap the value & continue 
///    - Else if None, throw error DONE
///    - Check that the argument/s in the Want fit the types
///      in the schemas function signature
///    - If any are not compatible, throw error
/// --
///    (Next, Check that the Want is either equivalent to, or a subset of the return type)
///    - Use return type identifier to get the type from the schema_types hashmap
///    - for each field in the Want
///       - check if the field is in the return type
///       - If not, throw error
///  - If no errors at this point, return Ok(())

pub fn validate_query_with_schema(parsed_query: &ParsedQuery, schema_definition: &SchemaDefinition) -> Result<(), CastleError>{
    for want in parsed_query.wants.values() {
        match want {
            Want::SingleField(_single_field) => {
            },
            Want::ObjectProjection(object_projection) => {
                let fields = unwrap_fields_throw_error_if_none(&object_projection.fields)?; //Need to think about match
                let resolver = schema_definition.functions.get(&object_projection.identifier);
                check_if_resolver_is_none_else_unwrap_resolver_and_check_if_arguments_are_compatible(resolver, object_projection, &schema_definition, fields)?;
            }
        }
    }
    return Ok(())
}

fn unwrap_fields_throw_error_if_none(fields: &Option<HashMap<Box<str>, Want>>) -> Result<&HashMap<Box<str>, Want>, CastleError> {
    return match fields {
        Some(identifier)  => Ok(identifier),
        None => Err(CastleError::NoIdentifierOnObjectProjection("No identifier on object projection".into())),
    };
}

fn check_if_resolver_is_none_else_unwrap_resolver_and_check_if_arguments_are_compatible(
    resolver: Option<&FnDefinition>, 
    object_projection: &ObjectProjection, 
    schema_definition: &SchemaDefinition, 
    fields: &HashMap<Box<str>, Want>)
    -> Result<(), CastleError> {
    if resolver.is_none() {
        return Err(CastleError::QueryResolverNotDefinedInSchema("no matching resolver found for in schema".into()));
    } 
    else {
        validate_resolver_and_assign_schema_type_for_fields_validation(resolver, object_projection, schema_definition, fields)?;
    }
    Ok(())
}

fn validate_resolver_and_assign_schema_type_for_fields_validation(
    resolver: Option<&FnDefinition>, 
    object_projection: &ObjectProjection, 
    schema_definition: &SchemaDefinition, 
    fields: &HashMap<Box<str>, Want>) -> Result<(), CastleError> {
        let resolver = resolver.unwrap();
        take_unwrapped_resolver_and_throw_error_if_none_and_check_length_if_some(resolver, object_projection)?;
        let return_type = unwrap_return_type_throw_error_if_none(&resolver.return_type)?;
        let schema_type = schema_definition.schema_types.get(return_type);                    // use resolver.return_type to get the schema_type from schema_definition.schema_types
        let schema_type = schema_type.unwrap();
        let schema_type_fields = &schema_type.fields;                               // unwrap the type from the schema_type
        for field in fields.keys(){                                                                   // for each field in fields, check if the field is in schema_type_fields
            if !schema_type_fields.contains_key(field) {
                return Err(CastleError::FieldsInReturnTypeDoNotMatchQuery("no type found for want".into()));
            }
        }
        Ok(())
}

fn unwrap_return_type_throw_error_if_none(return_type: &Type) -> Result<&Box<str>, CastleError> {
    return match return_type {
        Type::SchemaTypeOrEnum(schema_type_or_enum) => Ok(schema_type_or_enum),
        _ => Err(CastleError::NoIdentifierOnObjectProjection(" on resolver return type Not valid".into())),
    }
}

fn take_unwrapped_resolver_and_throw_error_if_none_and_check_length_if_some(resolver: &FnDefinition, object_projection: &ObjectProjection) -> Result<(), CastleError> {
    check_if_arguments_in_query_have_different_lengths(&resolver.args, &object_projection.arguments)?;
    iterate_through_argument_length_and_check_compatibility(&resolver.args, &object_projection.arguments)?;
    Ok(())
}

fn check_if_arguments_in_query_have_different_lengths(resolver_args: &HashMap<Box<str>, IdentifierAndTypeArgument>, query_args: &HashMap<Box<str>, IdentifierAndValueArgument>)
-> Result<(), CastleError> {
    
    //check args are compatible
    if query_args.len() != resolver_args.len() {
        return Err(CastleError::ArgumentsInQueryDoNotMatchResolver("arguments in query have different lengths".into()));
    }
    else {
        Ok(())
    }
}

fn iterate_through_argument_length_and_check_compatibility(resolver_args: &HashMap<Box<str>, IdentifierAndTypeArgument>, query_args: &HashMap<Box<str>, IdentifierAndValueArgument>)
-> Result<(), CastleError> {
    for (query_arg_name, query_arg_value) in query_args.values() {
        let arg_in_resolver = resolver_args.get(query_arg_name);
        if arg_in_resolver.is_none() {
            return Err(CastleError::QueryResolverNotDefinedInSchema("no matching with same identifier found for in schema".into()));
        }
        else {
            let arg_in_resolver = arg_in_resolver.as_ref().unwrap();
            check_type_and_value_are_compatible(&arg_in_resolver.1, query_arg_value)?;
        }
        let arg_in_resolver = arg_in_resolver.unwrap();
        let compatible = check_type_and_value_are_compatible(&arg_in_resolver.1, &query_arg_value)?;
        if !compatible { return Err(CastleError::ArgumentsInQueryDoNotMatchResolver("arguments in query do not match resolver".into())); }
    }
    Ok(())
}

/// use match to unwrap the argument in schema so we can use the type
/// convert argument from query to its equivalent type (value) -> type
/// if compatible return true, else return false
fn check_type_and_value_are_compatible(arg_in_resolver: &Type, arg_in_query: &PrimitiveValue)
-> Result<bool, CastleError>{
    let schema_type = arg_in_resolver;
    let arg_value = arg_in_query;
    let query_type = convert_value_to_corresponding_type(&arg_value)?;
    if &query_type == schema_type { return Ok(true) }
    else { return Ok(false) }
}

fn convert_value_to_corresponding_type(arg_value: &PrimitiveValue) -> Result<Type, CastleError> {
    match arg_value {
        PrimitiveValue::String(_) => Ok(Type::PrimitiveType(PrimitiveType::String)),
        PrimitiveValue::Int(_) => Ok(Type::PrimitiveType(PrimitiveType::Int)),
        PrimitiveValue::UInt(_) => Ok(Type::PrimitiveType(PrimitiveType::UInt)),
        PrimitiveValue::Float(_) => Ok(Type::PrimitiveType(PrimitiveType::Float)),
        PrimitiveValue::Boolean(_) => Ok(Type::PrimitiveType(PrimitiveType::Bool)),
    }
}
// need to add single field validation & match arm validation