use std::{collections::HashMap};

use parser_and_schema::{parsers::{query_parser::parse_query::ParsedQuery, schema_parser::types::{ type_system::Type, primitive_type::PrimitiveType}}, ast::syntax_definitions::{schema_definition::SchemaDefinition, want::{ Want, SingleField, ObjectProjection}, argument::Argument, expressions::PrimitiveValue}, token::token::Identifier};
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
                let identifier = unwrap_identifier_throw_error_if_none(&object_projection.identifier)?;
                let fields = unwrap_fields_throw_error_if_none(&object_projection.fields)?; //Need to think about match
                
                let resolver = schema_definition.functions.get(identifier);
                if resolver.is_none() {
                    return Err(CastleError::QueryResolverNotDefinedInSchema("no matching resolver found for in schema".into()));
                } else {
                    let resolver = resolver.unwrap();
                    if resolver.args.is_none() && object_projection.arguments.is_none() { } 
                    else if resolver.args.is_none() || object_projection.arguments.is_none() {
                        return Err(CastleError::ArgumentsInQueryDoNotMatchResolver("arguments in query do not match resolver. One has no arguments".into()));
                    }
                    else {
                        let resolver_args = resolver.args.as_ref().unwrap();
                        let query_args = object_projection.arguments.as_ref().unwrap();
                        //check args are compatible
                        if query_args.len() != resolver_args.len() {
                            return Err(CastleError::ArgumentsInQueryDoNotMatchResolver("arguments in query have different lengths".into()));
                        }
                        let mut i = 0;
                        while i < query_args.len() {
                            let arg_in_resolver = &resolver_args[i];
                            let arg_in_query = &query_args[i];
                            let compatible = check_arg_compatible(&arg_in_resolver, &arg_in_query)?;
                            if !compatible { return Err(CastleError::ArgumentsInQueryDoNotMatchResolver("arguments in query do not match resolver".into())); }

                            i += 1;
                        }
                    }

                    let return_type = unwrap_return_type_throw_error_if_none(&resolver.return_type)?;
                    // use resolver.return_type to get the schema_type from schema_definition.schema_types
                    let schema_type = schema_definition.schema_types.get(return_type);
                    let schema_type = schema_type.unwrap();
                    let schema_type_fields = &schema_type.fields;
                    // unwrap the type from the schema_type
                    // for each field in fields, check if the field is in schema_type_fields
                    for field in fields.keys(){
                        if !schema_type_fields.contains_key(field) {
                            return Err(CastleError::FieldsInReturnTypeDoNotMatchQuery("no type found for want".into()));
                        }
                    }
                }
            }
        }
    }
    return Ok(())
}
fn unwrap_identifier_throw_error_if_none(identifier: &Option<Box<str>>) -> Result<&Box<str>, CastleError> {
    return match identifier {
        Some(identifier)  => Ok(identifier),
        None => Err(CastleError::NoIdentifierOnObjectProjection("No identifier on object projection".into())),
    }
}

fn unwrap_fields_throw_error_if_none(fields: &Option<HashMap<Box<str>, Want>>) -> Result<&HashMap<Box<str>, Want>, CastleError> {
    return match fields {
        Some(identifier)  => Ok(identifier),
        None => Err(CastleError::NoIdentifierOnObjectProjection("No identifier on object projection".into())),
    };
}

fn unwrap_return_type_throw_error_if_none(return_type: &Option<Type>) -> Result<&Box<str>, CastleError> {
    return match return_type {
        Some(return_type)  =>match return_type {
            Type::SchemaTypeOrEnum(schema_type_or_enum) => Ok(schema_type_or_enum),
            _ => Err(CastleError::NoIdentifierOnObjectProjection(" on resolver return type Not valid".into())),
        },
        None => Err(CastleError::NoIdentifierOnObjectProjection("No identifier on object projection".into())),
    };
}

/// use match to unwrap the argument in schema so we can use the type
/// convert argument from query to its equivilent type (value) -> type
/// if compatible return true, else return false
/// 
fn check_arg_compatible(arg_in_resolver: &Argument, arg_in_query: &Argument) -> Result<bool, CastleError>{
    let schema_type = match arg_in_resolver {
        Argument::IdentifierAndType(_identifier, type_) => type_,
        _ => return Err(CastleError::ArgumentsInQueryDoNotMatchResolver("Argument in resolver is not a schema type".into())),
    };
    let arg_value = match arg_in_query {
        Argument::PrimitiveValue(value) => value,
        _ => return Err(CastleError::ArgumentsInQueryDoNotMatchResolver("Argument in query is not a primitive value".into())),
    };
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