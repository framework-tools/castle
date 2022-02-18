use std::{collections::HashMap};

use parser_and_schema::{ast::syntax_definitions::{schema_definition::{SchemaDefinition}, fn_definition::FnDefinition, directive_definition::{self, DirectiveDefinition}}, parsers::{schema_parser::parse_schema::parse_schema, query_parser::parse_query::parse_query}};
use shared::CastleError;

use crate::{resolvers::{resolver_map::ResolverMap, resolve_query_wants::resolve_all_wants, resolver_type::{TopLevelResolvers, Resolver}, individual_resolvers::page_resolvers::basic_page_info::basic_page_info}, directives::directives::DirectiveMap, validation::{self_validation_schema::self_validate_schema::self_validate_schema, validate_backend_fns_with_schema::validate_backend_fns_with_schema::validate_schema_with_resolvers_and_directives, validate_query_with_schema::validate_query_with_schema::validate_query_with_schema}, castle_schema::castle_schema::SCHEMA};

pub struct Castle<C, R>{
    pub resolver_map: ResolverMap<C, R>,
    pub parsed_schema: SchemaDefinition,
    pub directives: DirectiveMap<C, R>,
}

impl<C, R> Castle<C, R> {

    pub fn build_and_validate(
        resolvers: ResolverMap<C, R>,
        directives: DirectiveMap<C, R>,
        schema: &str,
    ) -> Result<Castle<C, R>, CastleError> {
        let parsed_schema = parse_schema(&schema)?;
        let castle = Castle {
            resolvers,
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
        validate_schema_with_resolvers_and_directives(&self.parsed_schema, &self.resolvers, &self.directives)?;
        return Ok(())
    }

    /// Parse query
    /// Cross validate query and schema
    /// resolve all wants
    pub fn parse_query_resolve_wants(&self, query: &str, context: C) -> Result<TopLevelResolvers<R>, CastleError> {
        let parsed_query = parse_query(query)?;
        validate_query_with_schema(&parsed_query, &self.parsed_schema)?;
        let resolved_wants = resolve_all_wants(parsed_query.wants, &self.resolvers, context)?;
        return Ok(resolved_wants)
    }
}

pub struct CastleBuilder<'a, C, R> {
    pub resolver_map: ResolverMap<C, R>,
    pub directives: DirectiveMap<C, R>,
    schema: Option<&'a str>,
}

impl<'a, C, R> CastleBuilder<'a, C, R> {
    pub fn new() -> Self {
        Self {
            resolvers: ResolverMap::new(),
            schema: None,
            directives: HashMap::new(),
        }
    }

    pub fn add_schema(mut self, schema: &'a str) -> Self {
        self.schema = Some(schema);
        self
    }

    pub fn apply_current_schema(&mut self) {
        self.schema = Some(SCHEMA);
    }

    pub fn build_and_validate(self) -> Result<Castle<C, R>, CastleError> {
        let schema;
        if self.schema.is_none() {
            return Err(CastleError::MissingSchema("No schema provided".into()));
        }
        else {
            schema = self.schema.unwrap();
        }
        Castle::build_and_validate(self.resolvers, self.directives, schema)
    }

    pub fn add_resolver(&mut self, resolver_name: &str, resolver: Resolver<C, R>) {
        self.resolvers.resolvers.insert(resolver_name.into(), resolver);
    }

    pub fn add_directive(&mut self, directive_name: &str, directive: Resolver<C, R>) {
        self.directives.insert(directive_name.into(), directive);
    }

    pub fn add_all_resolvers(&mut self) {
        self.resolvers.resolvers = create_resolver_map_with_all_resolvers();
    }
}