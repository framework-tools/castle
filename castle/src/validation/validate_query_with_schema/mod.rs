use std::collections::HashMap;

use parser_and_schema::{parsers::{query_parser::parse_query::ParsedQuery, schema_parser::types::schema_field::SchemaField}, ast::syntax_definitions::{schema_definition::SchemaDefinition, want::Want}};
use shared::castle_error::CastleError;

use self::{validate_object_projection::{if_inside_object_projection_check_field_exists_on_type, validate_object_projection_want}, validate_match_arms::validate_all_match_arms, validate_single_field::validate_single_field_want};

pub mod query_resolver_is_valid;
pub mod validate_match_arms;
pub mod validate_single_field;
pub mod validate_object_projection;
pub mod validate_value_to_type;

/// Cross-Validation Between Query Parser & Schema Parser
/// Needs to validate that the current schema and the query are compatible
/// To do this it needs to validate:
/// - Resolvers(object projections) in query are defined in the schema
/// - The argument in the query is compatible with the argument type in schema definition
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

pub fn validate_query_with_schema(query: &ParsedQuery, schema: &SchemaDefinition) -> Result<Vec<()>, CastleError> {
    let wants = &query.wants;
    return validate_wants(wants, schema, None)
}

pub(crate) fn validate_wants(
    wants: &HashMap<Box<str>, Want>, 
    schema: &SchemaDefinition,
    fields_to_compare: Option<&HashMap<Box<str>, SchemaField>>
) -> Result<Vec<()>, CastleError> {
    let result: Result<Vec<()>, CastleError> = wants.into_iter()
    .map(|(identifier, want)| validate_want(schema, &identifier, &want, fields_to_compare))
    .collect();
    return result
}

fn validate_want(
    schema: &SchemaDefinition, 
    identifier: &Box<str>, 
    want: &Want,
    fields_to_compare: Option<&HashMap<Box<str>, SchemaField>>
) -> Result<(), CastleError> {
    if_inside_object_projection_check_field_exists_on_type(identifier, fields_to_compare)?;

    return match want {
        Want::SingleField(arguments) if fields_to_compare == None => 
            validate_single_field_want(identifier, arguments, schema),
        
        Want::SingleField(_) => Ok(()),

        Want::ObjectProjection(fields, arguments) => 
            validate_object_projection_want(identifier, arguments, schema, fields),

        Want::Match(match_statement) =>
            validate_all_match_arms(match_statement, schema) 
    }
}

