use std::{collections::HashMap};

use parser_and_schema::{ast::syntax_definitions::{schema_definition::{SchemaDefinition}}, parsers::{schema_parser::parse_schema::parse_schema, query_parser::parse_query::parse_query}};
use shared::castle_error::CastleError;


use crate::{resolvers::{resolver_map::{ResolverMap}, resolver_type::{Resolver}}, directives::directives::DirectiveMap, validation::{validate_backend_fns_with_schema::validate_backend_fns_with_schema::validate_schema_with_resolvers_and_directives, validate_query_with_schema::validate_query_with_schema::validate_query_with_schema, self_validation_schema::self_validate_schema}};

pub struct Castle<C, R>{
    pub resolver_map: ResolverMap<C, R>,
    pub parsed_schema: SchemaDefinition,
    pub directives: DirectiveMap<C, R>,
}

impl<C, R> Castle<C, R> {

    pub fn build_and_validate(
        resolver_map: ResolverMap<C, R>,
        directives: DirectiveMap<C, R>,
        parsed_schema: SchemaDefinition,
    ) -> Result<Castle<C, R>, CastleError> {
        let castle = Castle {
            resolver_map,
            parsed_schema,
            directives
        };
        castle.validate()?;
        Ok(castle)
    }

    ///This function runs self validation and cross validation with resolvers and schemas
    /// - Self validate schema 
    ///     - all schema_types and enums used as types have been defined in the schema
    /// - Validate schema resolvers & directives (functions) match the ones we've built in Rust
    pub fn validate(&self) -> Result<(), CastleError> {
        self_validate_schema(&self.parsed_schema)?;
        validate_schema_with_resolvers_and_directives(&self.parsed_schema, &self.resolver_map, &self.directives)?;
        return Ok(())
    }

    /// Parse query
    /// Cross validate query and schema
    /// resolve all wants
    pub fn parse_query_and_validate(&self, query: &str) -> Result<(), CastleError> {
        let parsed_query = parse_query(query)?;
        validate_query_with_schema(&parsed_query, &self.parsed_schema)?;
        return Ok(())
    }
}

pub struct CastleBuilder<C, R> {
    pub resolver_map: ResolverMap<C, R>,
    pub directives: DirectiveMap<C, R>,
    pub parsed_schema: Option<SchemaDefinition>,
}

impl<'a, C, R> CastleBuilder<C, R> {
    pub fn new() -> Self {
        Self {
            resolver_map: ResolverMap::new(),
            parsed_schema: None,
            directives: HashMap::new(),
        }
    }

    pub fn add_schema(&mut self, schema: &'a str) -> Result<(), CastleError> {
        let parsed_schema  = parse_schema(schema)?;
        self.parsed_schema = Some(parsed_schema);
        Ok(())
    }

    pub fn build_and_validate(self) -> Result<Castle<C, R>, CastleError> {
        let schema;
        if self.parsed_schema.is_none() {
            return Err(CastleError::MissingSchema("No schema provided".into()));
        }
        else {
            schema = self.parsed_schema.unwrap();
        }
        Castle::build_and_validate(self.resolver_map, self.directives, schema)
    }

    pub fn add_resolver(&mut self, resolver_name: &str, resolver: Resolver<C, R>) {
        self.resolver_map.resolvers.insert(resolver_name.into(), resolver);
    }

    pub fn add_directive(&mut self, directive_name: &str, directive: Resolver<C, R>) {
        self.directives.insert(directive_name.into(), directive);
    }
}