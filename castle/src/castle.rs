use std::{collections::HashMap, fmt::Debug};

use castle_error::CastleError;
use query_parser::{parse_message, Message};
use schema_parser::{parsers::parse_schema::parse_schema, types::SchemaDefinition};

use crate::{
    validation::{
        executor::execute_message, validate_directives_exist::validate_directives_exist,
        validate_projection::validate_projection,
        validate_resolvers_exist::validate_resolvers_exist, validate_schema::validate_schema,
    },
    Directive, Resolver, Value,
};
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct Castle<Ctx: Debug, E: Debug> {
    pub parsed_schema: SchemaDefinition,
    #[derivative(Debug = "ignore")]
    pub field_resolvers: HashMap<Box<str>, Box<dyn Resolver<Ctx, E>>>,
    #[derivative(Debug = "ignore")]
    pub directives: HashMap<Box<str>, Box<dyn Directive<Ctx, E>>>,
}

impl<Ctx: Debug, E: Debug> Castle<Ctx, E> {
    pub(crate) fn build_and_validate(
        field_resolvers: HashMap<Box<str>, Box<dyn Resolver<Ctx, E>>>,
        directives: HashMap<Box<str>, Box<dyn Directive<Ctx, E>>>,
        parsed_schema: SchemaDefinition,
    ) -> Result<Castle<Ctx, E>, CastleError> {
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
        for projection in &parsed_message.projections {
            validate_projection(&self.parsed_schema, projection)?;
        }
        Ok(parsed_message)
    }

    /// Runs a query
    /// - Validates query against the schema for validity and type correctness
    /// - Runs the query using the resolvers
    /// - Returns the result
    pub async fn run_message(&self, query: &str, ctx: &Ctx) -> Result<Result<Value<Ctx, E>, E>, CastleError> {
        let mut parsed_message = self.validate_message(query)?;
        execute_message(&mut parsed_message, &self.field_resolvers, &self.directives, ctx).await
    }
}

#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct CastleBuilder<Ctx: Debug, E: Debug> {
    #[derivative(Debug = "ignore")]
    pub resolver_map: HashMap<Box<str>, Box<dyn Resolver<Ctx, E>>>,
    #[derivative(Debug = "ignore")]
    pub directives: HashMap<Box<str>, Box<dyn Directive<Ctx, E>>>,
    pub parsed_schema: Result<SchemaDefinition, CastleError>,
}

impl<Ctx: Send + 'static + Debug, E: Debug + 'static> CastleBuilder<Ctx, E> {
    pub fn new(schema: &str) -> Self {
        Self {
            resolver_map: HashMap::new(),
            parsed_schema: parse_schema(schema),
            directives: HashMap::new(),
        }
    }

    pub fn build(self) -> Result<Castle<Ctx, E>, CastleError> {
        Castle::build_and_validate(self.resolver_map, self.directives, self.parsed_schema?)
    }

    pub fn add_resolver(
        mut self,
        resolver_name: &str,
        resolver: impl Resolver<Ctx, E> + 'static
    ) -> Self {
        self.resolver_map.insert(resolver_name.into(), Box::new(resolver));
        self
    }

    pub fn add_directive(
        mut self,
        directive_name: &str,
        directive: impl Directive<Ctx, E> + 'static,
    ) -> Self {
        self.directives
            .insert(directive_name.into(), Box::new(directive));
        self
    }
}
