use std::collections::HashMap;

use castle_error::CastleError;
use query_parser::parse_query;
use schema_parser::{parsers::parse_schema::parse_schema, types::SchemaDefinition};

use crate::{
    validation::{
        validate_directives_exist::validate_directives_exist, validate_query::validate_query,
        validate_resolvers_exist::validate_resolvers_exist, validate_schema::validate_schema,
    },
    Directive, Resolver, Value,
};

pub struct Castle<C, R> {
    pub field_resolvers: HashMap<Box<str>, Resolver<C, R>>,
    pub parsed_schema: SchemaDefinition,
    pub directives: HashMap<Box<str>, Box<dyn Directive<C, R>>>,
}

impl<C, R> Castle<C, R> {
    pub(crate) fn build_and_validate(
        field_resolvers: HashMap<Box<str>, Resolver<C, R>>,
        directives: HashMap<Box<str>, Box<dyn Directive<C, R>>>,
        parsed_schema: SchemaDefinition,
    ) -> Result<Castle<C, R>, CastleError> {
        let castle = Castle {
            field_resolvers,
            parsed_schema,
            directives,
        };
        castle.validate()?;
        Ok(castle)
    }

    ///This function runs self validation and cross validates the schema types and enums.
    /// It also checks if the necessary resolvers have also been provided
    /// - Self validate schema
    ///     - all schema_types and enums used as types have been defined in the schema
    /// - Validate schema resolvers & directives (functions) match the ones we've built in Rust
    fn validate(&self) -> Result<(), CastleError> {
        validate_schema(&self.parsed_schema)?;
        validate_resolvers_exist(&self.parsed_schema, &self.field_resolvers)?;
        validate_directives_exist(&self.parsed_schema, &self.directives)?;
        return Ok(());
    }

    /// Runs a query
    /// - Validates query against the schema for validity and type correctness
    /// - Runs the query using the resolvers
    /// - Returns the result
    pub fn run_query(&self, query: &str) -> Result<Value<C, R>, CastleError> {
        let parsed_query = parse_query(query)?;
        validate_query(&self.parsed_schema, parsed_query)?;
        // execute_query(parsed_query, &self.field_resolvers, &self.directive_resolvers)
        unimplemented!()
    }
}

pub struct CastleBuilder<C, R> {
    pub resolver_map: HashMap<Box<str>, Resolver<C, R>>,
    pub directives: HashMap<Box<str>, Box<dyn Directive<C, R>>>,
    pub parsed_schema: SchemaDefinition,
}

impl<C: Send + 'static, R: 'static> CastleBuilder<C, R> {
    pub fn new(schema: &str) -> Result<Self, CastleError> {
        Ok(Self {
            resolver_map: HashMap::new(),
            parsed_schema: parse_schema(schema)?,
            directives: HashMap::new(),
        })
    }

    pub fn build(self) -> Result<Castle<C, R>, CastleError> {
        Castle::build_and_validate(self.resolver_map, self.directives, self.parsed_schema)
    }

    pub fn add_resolver(&mut self, resolver_name: &str, resolver: Resolver<C, R>) -> &mut Self {
        self.resolver_map.insert(resolver_name.into(), resolver);
        self
    }

    pub fn add_directive(
        &mut self,
        directive_name: &str,
        directive: impl Directive<C, R> + 'static,
    ) -> &mut Self {
        self.directives
            .insert(directive_name.into(), Box::new(directive));

        self
    }
}
