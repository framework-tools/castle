use std::collections::HashMap;

use castle_error::CastleError;
use castle_query_parser::{parse_message, Message};
use castle_schema_parser::{parsers::parse_schema::parse_schema, types::SchemaDefinition};

use crate::{
    validation::{
        validate_directives_exist::validate_directives_exist,
        validate_projection::validate_projection,
    },
    Directive, Resolver, context::Context, Value, executor::execute_message,
};
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct Castle {
    pub parsed_schema: SchemaDefinition,
    #[derivative(Debug = "ignore")]
    pub directives: HashMap<Box<str>, Box<dyn Directive>>,
}

impl Castle {

    ///This function runs self validation and cross validates the schema types and enums.
    /// It also checks if the necessary resolvers have also been provided
    /// - Self validate schema
    ///     - all schema_types and enums used as types have been defined in the schema
    /// - Validate schema resolvers & directives (functions) match the ones we've built in Rust
    fn validate(&self) -> Result<(), CastleError> {
        // validate_schema(&self.parsed_schema)?;
        // validate_resolvers_exist(&self.parsed_schema, &self.field_resolvers)?;
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
    ) -> Result<(Value, Vec<anyhow::Error>), CastleError> {
        let mut parsed_message = self.validate_message(query)?;
        Ok(execute_message(
            &mut parsed_message,
            &self.directives,
            &self.parsed_schema,
            ctx,
        ).await)
    }
}