use std::{collections::HashMap};

use parser_and_schema::{parsers::{query_parser::parse_query::ParsedQuery, schema_parser::types::{ type_system::Type, primitive_type::PrimitiveType}}, ast::syntax_definitions::{schema_definition::SchemaDefinition, want::{Want, Wants, WantArguments}, expressions::PrimitiveValue, fn_definition::FnDefinition, argument::{IdentifierAndTypeArgument, IdentifierAndValueArgument, self}, match_statement::{self, MatchArm}, enum_definition}};
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
///    - match want to want to single field & object projection
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
    for (identifier, want) in &parsed_query.wants {
        let resolver = schema_definition.functions.get(identifier);
        //check resolver exists in schema
        if resolver.is_none() {
            return Err(CastleError::QueryResolverNotDefinedInSchema(format!("no matching resolver found in schema. Got: '{}' in query ", identifier).into()));
        } 
        else {
            match want {
                Want::SingleField(arguments) => {
                    validate_resolver_and_assign_schema_type_for_fields_validation(resolver, identifier, &None, arguments, schema_definition, )?;
                },
                Want::ObjectProjection(fields, arguments) => {
                    validate_resolver_and_assign_schema_type_for_fields_validation(resolver, identifier, &Some(&fields), arguments, schema_definition, )?;
                },
                Want::Match(match_statement) => {
                    validate_enum_used_is_defined_in_schema(match_statement, schema_definition)?;
                },  // need to implement
            }
        }
    }
    return Ok(())
}

fn validate_resolver_and_assign_schema_type_for_fields_validation(
    resolver: Option<&FnDefinition>,
    identifier: &Box<str>,
    fields: &Option<&Wants>,
    arguments: &WantArguments,
    schema_definition: &SchemaDefinition
) -> Result<(), CastleError> {
    let resolver = resolver.unwrap();
    check_arguments_are_compatible(resolver, &arguments)?;
    if fields.is_none() {
        validate_single_field_want(resolver, &identifier)?;
    }
    else {
        validate_object_projection_want(resolver, schema_definition, fields)?;
    }
    return Ok(())
}

fn validate_single_field_want(resolver: &FnDefinition, identifier: &Box<str>) -> Result<(), CastleError> {
    match resolver.return_type {
        Type::SchemaTypeOrEnum(_) => {
            Err(CastleError::FieldsInReturnTypeDoNotMatchQuery(format!("no fields in return type. Got: '{}' in query ", identifier).into()))
        },
        Type::Void => {
            Err(CastleError::FieldsInReturnTypeDoNotMatchQuery(format!("no fields in return type. Got: '{}' in query ", identifier).into()))
        },
        _ => return Ok(())
    }
}

fn validate_object_projection_want(resolver: &FnDefinition, schema_definition: &SchemaDefinition, fields: &Option<&Wants>) -> Result<(), CastleError>{
    let return_type = check_return_type_is_schema_type_or_enum(&resolver.return_type)?;
    let schema_type = schema_definition.schema_types.get(return_type);    // use resolver.return_type to get the schema_type from schema_definition.schema_types
    let schema_type = schema_type.unwrap();
    let schema_type_fields = &schema_type.fields;     // unwrap the type from the schema_type
    let fields = fields.unwrap();
    for (identifier, field) in fields{    // for each field in fields, check if the field is in schema_type_fields
        if !schema_type_fields.contains_key(identifier) {
            return Err(CastleError::FieldsInReturnTypeDoNotMatchQuery(format!("Field in want: {}, not found in schema type: {:?}", identifier, schema_type).into() ));
        }
        match field{
            Want::Match(match_statement) => {
                validate_enum_used_is_defined_in_schema(match_statement, schema_definition)?;
            },
            _ => {
                // do nothing
            }
        }
    }
    Ok(())
}

fn validate_enum_used_is_defined_in_schema(match_statement: &Vec<MatchArm>, schema_definition: &SchemaDefinition) -> Result<(), CastleError>{
    for match_arm in match_statement {
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
    }
    return Ok(())
}

fn check_return_type_is_schema_type_or_enum(return_type: &Type) -> Result<&Box<str>, CastleError> {
    return match return_type {
        Type::SchemaTypeOrEnum(schema_type_or_enum) => Ok(schema_type_or_enum),
        _ => Err(CastleError::NoIdentifierOnObjectProjection(format!(" on resolver return type Not valid. Type: {:?}", return_type).into())),
    }
}

fn check_arguments_are_compatible(resolver: &FnDefinition, arguments: &WantArguments,) -> Result<(), CastleError> {
    check_if_arguments_in_query_have_different_lengths(&resolver.args, &arguments)?;
    iterate_through_arguments_and_check_compatibility(&resolver.args, &arguments)?;
    Ok(())
}

fn check_if_arguments_in_query_have_different_lengths(resolver_args: &HashMap<Box<str>, IdentifierAndTypeArgument>, query_args: &HashMap<Box<str>, IdentifierAndValueArgument>)
-> Result<(), CastleError> {
    
    //check args are compatible
    if query_args.len() != resolver_args.len() {
        return Err(CastleError::ArgumentsInQueryDoNotMatchResolver(format!("arguments in query have different lengths. Query args length: {}, resolver args length: {}", 
        query_args.len(), resolver_args.len()).into()));
    }
    else {
        Ok(())
    }
}

fn iterate_through_arguments_and_check_compatibility(resolver_args: &HashMap<Box<str>, IdentifierAndTypeArgument>, query_args: &HashMap<Box<str>, IdentifierAndValueArgument>)
-> Result<(), CastleError> {
    for (query_arg_name, query_arg_value) in query_args.values() {
        let arg_in_resolver = resolver_args.get(query_arg_name);
        if arg_in_resolver.is_none() {
            return Err(CastleError::QueryResolverNotDefinedInSchema(format!("no argument found in resolver with matching identifier: {}", query_arg_name).into()));
        }
        else {
            let arg_in_resolver = arg_in_resolver.as_ref().unwrap();
            check_type_and_value_are_compatible(&arg_in_resolver.1, query_arg_value)?;
        }
        let arg_in_resolver = arg_in_resolver.unwrap();
        let compatible = check_type_and_value_are_compatible(&arg_in_resolver.1, &query_arg_value)?;
        if !compatible { return Err(CastleError::ArgumentsInQueryDoNotMatchResolver(
            format!("for argument with identifier: {}, type in schema: {:?}, and value in query: {:?} are not compatible", query_arg_name, arg_in_resolver.1, query_arg_value).into())); }
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
    else if query_type == Type::PrimitiveType(PrimitiveType::UInt) && schema_type == &Type::PrimitiveType(PrimitiveType::Int){
        return Ok(true)
    }
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