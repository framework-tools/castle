use castle_query_parser::parse_message;
use castle_types::{
    CastleError, Context, Directive, Message, ResolvesFields, SchemaDefinition, SchemaItem, Value,
};
use std::collections::HashMap;

use crate::{
    executor::execute_message,
    validation::{
        validate_directives_exist::validate_directives_exist,
        validate_projection::validate_projection, validate_schema::validate_schema, validate_resolvers_exist::validate_resolvers_exist,
    },
};
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct Castle {
    root: Box<dyn ResolvesFields>,
    parsed_schema: SchemaDefinition,
    #[derivative(Debug = "ignore")]
    directives: HashMap<Box<str>, Box<dyn Directive>>,
}

impl Castle {
    pub(crate) fn new_and_validate(
        root: Box<dyn ResolvesFields>,
        schema_def: SchemaDefinition,
        directives: HashMap<Box<str>, Box<dyn Directive>>,
    ) -> Result<Self, CastleError> {
        let mut castle = Castle {
            root,
            parsed_schema: schema_def,
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
        )
        .await)
    }
}

#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct CastleBuilder {
    root: Option<Box<dyn ResolvesFields>>,
    schema_def: Option<SchemaDefinition>,
    #[derivative(Debug = "ignore")]
    directives: HashMap<Box<str>, Box<dyn Directive>>,
}

impl CastleBuilder {
    pub fn new<Root: ResolvesFields + SchemaItem + 'static>(root: Root) -> Self {
        let mut schema_def = SchemaDefinition::new();
        Root::initialize_item(&mut schema_def);

        Self {
            root: Some(Box::new(root)),
            schema_def: Some(schema_def),
            directives: HashMap::new(),
        }
    }

    pub fn add_directive<T: Directive + SchemaItem + 'static>(&mut self, name: &str, directive: T) -> &mut Self {
        self.schema_def.as_mut().map(|schema_def| T::initialize_item(schema_def));
        self.directives
            .insert(name.into(), Box::new(directive));
        self
    }

    pub fn build(&mut self) -> Result<Castle, CastleError> {
        Castle::new_and_validate(
            self.root.take().unwrap(),
            self.schema_def.take().unwrap(),
            self.directives.drain().collect(),
        )
    }
}
