



use std::collections::HashMap;

use shared::castle_error::CastleError;

use crate::{tokenizer::tokenizer::Tokenizer, ast::syntax_definitions::{schema_definition::SchemaDefinition, keyword::Keyword}, token::token::TokenKind};

use super::{parse_schema_type::parse_schema_type, enums::parse_enum::parse_enum_definition, functions::{parse_function::parse_function, parse_directive_definition::parse_directive_definition}};

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
    let mut parsed_schema: SchemaDefinition = HashMap::new();

    loop {
        let token = tokenizer.next(true)?;
        match token {
            Some(token) => match token.kind {
                TokenKind::Keyword(Keyword::Type) => {
                    let schema_type = parse_schema_type(&mut tokenizer)?;
                    parsed_schema.schema_types.insert(schema_type.name.clone(), schema_type);
                },
                TokenKind::Keyword(Keyword::Enum) => {
                    let enum_definition = parse_enum_definition(&mut tokenizer)?;
                    parsed_schema.enums.insert(enum_definition.name.clone(), enum_definition);
                },  
                TokenKind::Keyword(Keyword::Fn) => {
                    let function_definition = parse_function(&mut tokenizer)?;
                    parsed_schema.functions.insert(function_definition.name.clone(), function_definition);
                },
                TokenKind::Keyword(Keyword::Directive) => {
                    let directive_definition = parse_directive_definition(&mut tokenizer)?;
                    parsed_schema.directives.insert(directive_definition.name.clone(), directive_definition);
                },
                _ => return Err(CastleError::Schema(format!("Expected item, found: {:?}", token.kind).into(), token.span))
            },
            None => break
        }
        
    }
    return Ok(parsed_schema)
}