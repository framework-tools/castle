use std::{collections::HashMap, option};

use parser_and_schema::{ast::syntax_definitions::schema_definition::{SchemaDefinition, self}, parsers::{schema_parser::parse_schema::parse_schema, query_parser::parse_query::parse_query}};
use shared::CastleError;

use crate::{resolvers::resolvers::{ResolverMap, Resolver, TopLevelResolvers, resolve_all_wants, AllResolvedWants}, directives::directives::DirectiveMap, validation::{self_validation_schema::self_validate_schema::self_validate_schema, validate_schema_with_functions::validate_schema_with_resolvers::validate_schema_with_resolvers_and_directives, validate_query_with_schema::validate_query_with_schema::validate_query_with_schema}};

pub struct Castle<C, R>{
    resolvers: ResolverMap<C, R>,
    schema: String,
    parsed_schema: SchemaDefinition,
    directives: DirectiveMap<C, R>,
}

impl<C, R> Castle<C, R> {
    pub fn builder() -> CastleBuilder<C, R> {
        CastleBuilder::new()
    }

    pub fn build_and_validate(
        resolvers: ResolverMap<C, R>,
        directives: DirectiveMap<C, R>,
        schema: String
    ) -> Result<Castle<C, R>, CastleError> {
        let parsed_schema = parse_schema(&schema)?;
        let mut castle = Castle {
            resolvers,
            schema,
            parsed_schema,
            directives
        };

        castle.validate()?;

        // castle.validate_resolvers_to_schema()?;

        Ok(castle)
    }
    ///This function runs every validation for schema, parser, and resolvers
    /// - Self validate schema 
    ///     - all schema_types and enums used as types have been defined in the schema
    /// - Validate schema resolvers & directives (functions) match the ones we've built in Rust
    /// - Cross validate query and schema
    ///    - query resolvers match the resolvers defined in the schema

    pub fn validate(&self) -> Result<(), CastleError> {
        self_validate_schema(&self.parsed_schema)?;
        validate_schema_with_resolvers_and_directives(&self.parsed_schema, &self.resolvers, &self.directives)?;
        return Ok(())
    }

    pub fn parse_query_resolve_wants(&self, query: &str, context: C) -> Result<AllResolvedWants<R>, CastleError> {
        let parsed_query = parse_query(query)?;
        validate_query_with_schema(&parsed_query, &self.parsed_schema)?;
        let resolved_wants = resolve_all_wants(parsed_query.wants, &self.resolvers, context)?;
        return Ok(resolved_wants)
    }
}

pub struct CastleBuilder<C, R> {
    pub resolvers: ResolverMap<C, R>,
    pub directives: DirectiveMap<C, R>,
    pub schema: Option<String>,
    pub parsed_schema: Option<SchemaDefinition>,
}

impl<C, R> CastleBuilder<C, R> {
    pub fn new() -> Self {
        Self {
            resolvers: HashMap::new(),
            schema: None,
            parsed_schema: None,
            directives: HashMap::new(),
        }
    }

    pub fn schema(mut self, schema: String) -> Self {
        self.schema = Some(schema);
        self
    }

    pub fn build(self) -> Result<Castle<C, R>, CastleError> {
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
        self.resolvers.insert(resolver_name.into(), resolver);
    }
}