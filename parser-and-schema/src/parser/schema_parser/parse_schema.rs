use std::{collections::HashMap};

use shared::CastleError;

use crate::tokenizer::tokenizer::Tokenizer;

use super::{types::schema_type::SchemaType, parse_schema_type::check_token_and_parse_schema_type_or_break, handle_schema_errors::get_parsed_schema_or_err};

/// takes in schema as string and returns parsed schema as hashmap
///     - creates tokenizer
///     - loops through tokens
///     - if token is type keyword, parse type
///     - insert parsed type into hashmap
///     - if token is none, break loop
///     - return parsed schema
pub fn parse_schema(schema: &str) -> Result<HashMap<Box<str>, SchemaType>, CastleError> {
    let bytes = schema.as_bytes();
    let mut tokenizer = Tokenizer::new(bytes);
    let mut parsed_schema: HashMap<Box<str>, SchemaType> = HashMap::new();

    loop {
        let token = tokenizer.next(true)?;
        //function below parses schema types and inserts into parsed_schema
        let at_end_of_schema = check_token_and_parse_schema_type_or_break(token, &mut tokenizer, &mut parsed_schema)?;
        if at_end_of_schema { break; }
    }
    return get_parsed_schema_or_err(parsed_schema)
}