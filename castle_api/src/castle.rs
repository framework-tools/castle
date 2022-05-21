use std::{collections::HashMap, error::Error};

use castle_error::CastleError;
use castle_query_parser::{parse_message, Message};
use castle_schema_parser::{parsers::parse_schema::parse_schema, types::SchemaDefinition};

use crate::{
    executor::execute_message,
    validation::{
        validate_directives_exist::validate_directives_exist,
        validate_projection::validate_projection,
        validate_resolvers_exist::validate_resolvers_exist, validate_schema::validate_schema,
    },
    Directive, Resolver, context::Context, Value,
};
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct Castle {
    pub parsed_schema: SchemaDefinition,
    #[derivative(Debug = "ignore")]
    pub field_resolvers: HashMap<Box<str>, Box<dyn Resolver>>,
    #[derivative(Debug = "ignore")]
    pub directives: HashMap<Box<str>, Box<dyn Directive>>,
}

impl Castle {
    pub(crate) fn build_and_validate(
        field_resolvers: HashMap<Box<str>, Box<dyn Resolver>>,
        directives: HashMap<Box<str>, Box<dyn Directive>>,
        parsed_schema: SchemaDefinition,
    ) -> Result<Castle, CastleError> {
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

    pub fn validate_message(&self, query: &str) -> Result<Message, CastleError> {
        let parsed_message = parse_message(query)?;
        validate_projection(&self.parsed_schema, &parsed_message.projection)?;
        Ok(parsed_message)
    }

    /// Runs a query
    /// - Validates query against the schema for validity and type correctness
    /// - Runs the query using the resolvers
    /// - Returns the result
    pub async fn run_message(
        &self,
        query: &str,
        ctx: &Context,
    ) -> Result<(Value, Vec<Error>), CastleError> {
        let mut parsed_message = self.validate_message(query)?;
        execute_message(
            &mut parsed_message,
            &self.field_resolvers,
            &self.directives,
            &self.parsed_schema,
            ctx,
        )
        .await
    }
}

#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct CastleBuilder {
    #[derivative(Debug = "ignore")]
    resolver_map: HashMap<Box<str>, Box<dyn Resolver>>,
    #[derivative(Debug = "ignore")]
    directives: HashMap<Box<str>, Box<dyn Directive>>,
    schema: String,
}

impl CastleBuilder {
    pub fn new(schema: &str) -> Self {
        Self {
            resolver_map: HashMap::new(),
            schema: schema.into(),
            directives: HashMap::new(),
        }
    }

    pub fn build(&mut self) -> Result<Castle, CastleError> {
        Castle::build_and_validate(
            self.resolver_map.drain().collect(),
            self.directives.drain().collect(),
            parse_schema(&self.schema)?,
        )
    }

    pub fn add_resolver(
        &mut self,
        resolver_name: &str,
        resolver: impl Resolver + 'static,
    ) -> &mut Self {
        self.resolver_map
            .insert(resolver_name.into(), Box::new(resolver));
        self
    }

    pub fn add_directive(
        &mut self,
        directive_name: &str,
        directive: impl Directive + 'static,
    ) -> &mut Self {
        self.directives
            .insert(directive_name.into(), Box::new(directive));
        self
    }
}
