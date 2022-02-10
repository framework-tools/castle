
///This function runs every validation for schema, parser, and resolvers
/// - Self validate schema 
///     - all schema_types and enums used as types have been defined in the schema
/// - Validate schema resolvers & directives (functions) match the ones we've built in Rust
/// - Cross validate query and schema
///    - query resolvers match the resolvers defined in the schema

pub fn validate_everything(schema: &str, query: &str) -> Result<(), CastleError> {
    
    self_validate_schema()?;
}