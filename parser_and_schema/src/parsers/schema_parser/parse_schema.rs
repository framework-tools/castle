

use shared::CastleError;

use crate::{tokenizer::tokenizer::Tokenizer, ast::syntax_definitions::schema_definition::SchemaDefinition};

use super::{parse_schema_type::check_token_and_parse_schema_or_break};

/// takes in schema as string and returns parsed schema as hashmap
///     - creates tokenizer
///     - loops through tokens
///     - if token is type keyword, parse type
///     - insert parsed type into hashmap
///     - if token is none, break loop
///     - return parsed schema
pub fn parse_schema(schema: &str) -> Result<SchemaDefinition, CastleError> {
    let bytes = schema.as_bytes();
    let mut tokenizer = Tokenizer::new(bytes);
    let mut parsed_schema: SchemaDefinition = SchemaDefinition::new();

    loop {
        //function below parses schema types and inserts into parsed_schema
        let at_end_of_schema = check_token_and_parse_schema_or_break(&mut tokenizer, &mut parsed_schema)?;
        if at_end_of_schema { break; }
    }
    return Ok(parsed_schema)
}