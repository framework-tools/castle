use castle_error::CastleError;
use tokenizer::{Keyword, TokenKind, Tokenizable, Tokenizer};

use crate::types::SchemaDefinition;

use super::{
    parse_directive_definition::parse_directive_definition,
    parse_directives::parse_directives, parse_enum_definition::parse_enum_definition,
    parse_type_definition::parse_type_definition,
};

pub fn parse_schema(schema: &str) -> Result<SchemaDefinition, CastleError> {
    let bytes = schema.as_bytes();
    let mut tokenizer = Tokenizer::new(bytes);
    let mut schema_definition = SchemaDefinition::new();
    loop {
        // directive implementations for types and enums come before the type.
        let (kind, span) = if let Some(token) = tokenizer.next(true)? {
            (token.kind, token.span)
        } else {
            // we're done
            return Ok(schema_definition);
        };
        let directives = parse_directives(&mut tokenizer)?;
        match kind {
            TokenKind::Keyword(Keyword::Type) => {
                let type_ = parse_type_definition(&mut tokenizer, directives)?;
                schema_definition
                    .types
                    .insert(type_.ident.clone(), type_);
            }
            TokenKind::Keyword(Keyword::Enum) => {
                let enum_ = parse_enum_definition(&mut tokenizer, directives)?;
                schema_definition.enums.insert(enum_.ident.clone(), enum_);
            }
            TokenKind::Keyword(Keyword::Directive) => {
                if directives.len() != 0 {
                    return Err(CastleError::Other(
                        "Directive definitions cannot have directives.".into(),
                    ));
                }
                let directives = parse_directive_definition(&mut tokenizer)?;
                schema_definition
                    .directives
                    .insert(directives.name.clone(), directives);
            }
            _ => {
                return Err(CastleError::Schema(
                    format!("Expected item, found: {:?}", kind).into(),
                    span,
                ))
            }
        }
    }
}
