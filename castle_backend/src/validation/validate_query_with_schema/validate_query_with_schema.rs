use parser_and_schema::{parsers::query_parser::parse_query::ParsedQuery, ast::syntax_definitions::schema_definition::SchemaDefinition};
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
///    - check if the want is defined in the schema
///    (HashMap.get(want.identifier))
///    - If Some value, unwrap the value & continue
///    - Else if None, throw error
///    - Check that the argument/s in the Want fit the types
///      in the schemas function signature
///    - If any are not compatible, throw error
///    (Next, Check that the Want is either equivalent to, or a subset of the return type)
///    - Use return type identifier to get the type from the schema_types hashmap
///    - for each field in the Want
///       - check if the field is in the return type
///       - If not, throw error
///  - If no errors at this point, return Ok(())

pub fn validate_query_with_schema(parsed_query: &ParsedQuery, schema_definition: &SchemaDefinition) -> Result<(), CastleError> {
    return Ok(())
}