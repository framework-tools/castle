use std::{collections::HashMap};

use shared::CastleError;

use crate::{tokenizer::tokenizer::Tokenizer, ast::syntax_definitions::schema_definition::SchemaDefinition};

use super::{types::schema_type::SchemaType, parse_schema_type::check_token_and_parse_schema_type_or_break, handle_schema_errors::check_for_undefined_schema_types};

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
        let token = tokenizer.next(true)?;
        //function below parses schema types and inserts into parsed_schema
        let at_end_of_schema = check_token_and_parse_schema_type_or_break(token, &mut tokenizer, &mut parsed_schema)?;
        if at_end_of_schema { break; }
    }
    check_for_undefined_schema_types(&parsed_schema)?;
    return Ok(parsed_schema)
}