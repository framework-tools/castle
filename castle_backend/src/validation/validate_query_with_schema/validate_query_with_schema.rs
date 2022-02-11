use std::{collections::HashMap, slice::SliceIndex};

use parser_and_schema::{parsers::{query_parser::parse_query::ParsedQuery, schema_parser::types::{schema_type::SchemaType, type_system::Type}}, ast::syntax_definitions::{schema_definition::SchemaDefinition, want::{self, Want, SingleField, ObjectProjection}}};
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
            Want::SingleField(single_field) => {
            },
            Want::ObjectProjection(object_projection) => {
                let identifier = unwrap_identifier_throw_error_if_none(&object_projection.identifier)?;
                let fields = unwrap_fields_throw_error_if_none(&object_projection.fields)?;
                
                let resolver = schema_definition.functions.get(identifier);
                if resolver.is_none() {
                    return Err(CastleError::AbruptEOF("no type found for want".into()));
                } else {
                    let resolver = resolver.unwrap();
                    let return_type = unwrap_return_type_throw_error_if_none(&resolver.return_type)?;
                    // use resolver.return_type to get the schema_type from schema_definition.schema_types
                    let schema_type = schema_definition.schema_types.get(return_type);
                    let schema_type = schema_type.unwrap();
                    let schema_type_fields = &schema_type.fields;
                    // unwrap the type from the schema_type
                    // for each field in fields, check if the field is in schema_type_fields
                    for field in fields.keys(){
                        if !schema_type_fields.contains_key(field) {
                            return Err(CastleError::AbruptEOF("no type found for want".into()));
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

// need to add single field validation & match arm validation