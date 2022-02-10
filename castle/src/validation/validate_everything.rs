use parser_and_schema::parsers::schema_parser::parse_schema::parse_schema;
use shared::CastleError;

use super::self_validation_schema::self_validate_schema::self_validate_schema;


///This function runs every validation for schema, parser, and resolvers
/// - Self validate schema 
///     - all schema_types and enums used as types have been defined in the schema
/// - Validate schema resolvers & directives (functions) match the ones we've built in Rust
/// - Cross validate query and schema
///    - query resolvers match the resolvers defined in the schema

pub fn validate_everything(schema: &str, query: &str) -> Result<(), CastleError> {
    let parsed_schema = parse_schema(schema)?;
    let parsed_query = parse_query(query)?;
    self_validate_schema(parsed_schema)?;
    validate_schema_with_resolvers_and_directives()?;
    validate_query_with_schema()?;
    return Ok(())
}