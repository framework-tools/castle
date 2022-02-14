use std::collections::HashMap;

use parser_and_schema::{parsers::{schema_parser::parse_schema::parse_schema, query_parser::parse_query::parse_query}, ast::syntax_definitions::{schema_definition::SchemaDefinition, directive_definition::DirectiveDefinition}};
use shared::CastleError;


use crate::{resolvers::resolvers::{ResolverMap}, directives::directives::DirectiveMap};

use super::{self_validation_schema::self_validate_schema::self_validate_schema, validate_schema_with_functions::validate_schema_with_resolvers::validate_schema_with_resolvers_and_directives, validate_query_with_schema::validate_query_with_schema::validate_query_with_schema};


///This function runs every validation for schema, parser, and resolvers
/// - Self validate schema 
///     - all schema_types and enums used as types have been defined in the schema
/// - Validate schema resolvers & directives (functions) match the ones we've built in Rust
/// - Cross validate query and schema
///    - query resolvers match the resolvers defined in the schema

pub fn validate_everything<C, T>(
    schema: &str, 
    query: &str, 
    resolvers: ResolverMap<C, T>, 
    directives: DirectiveMap<C, T>, 
    ) -> Result<(), CastleError> {

    let schema_definition = parse_schema(schema)?;
    let parsed_query = parse_query(query)?;
    self_validate_schema(&schema_definition)?;
    validate_schema_with_resolvers_and_directives(&schema_definition, &resolvers, &directives)?;
    validate_query_with_schema(&parsed_query, &schema_definition)?;
    return Ok(())
}