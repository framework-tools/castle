
use castle_tokenizer::{Keyword, TokenKind, Tokenizable, Tokenizer};
use castle_types::{SchemaDefinition, CastleError};

use super::{
    parse_directive_definition::parse_directive_definition,
    parse_directives::parse_directives, parse_enum_definition::parse_enum_definition,
    parse_type_definition::parse_type_definition, parse_input_type_definition::parse_input_type_definition,
};

pub fn parse_schema(schema: &str) -> Result<SchemaDefinition, CastleError> {
    let bytes = schema.as_bytes();
    let mut tokenizer = Tokenizer::new(bytes);
    let mut schema_definition = SchemaDefinition::new();
    loop {
        // directive implementations for types and enums come before the type.
        let directives = parse_directives(&mut tokenizer)?;

        let token = if let Some(token) = tokenizer.next(true)? {
            token
        } else {
            // we're done
            if directives.len() != 0 {
                Err(CastleError::Validation(
                    "Cannot have directives at the end of the schema".into(),
                ))?
            }
            return Ok(schema_definition);
        };

        match token.kind {
            TokenKind::Keyword(Keyword::Type) => {
                let type_ = parse_type_definition(&mut tokenizer, directives)?;
                schema_definition.types.insert(type_.ident.clone(), type_);
            }
            TokenKind::Keyword(Keyword::Enum) => {
                let enum_ = parse_enum_definition(&mut tokenizer, directives)?;
                schema_definition.enums.insert(enum_.ident.clone(), enum_);
            }
            TokenKind::Keyword(Keyword::Directive) => {
                if directives.len() != 0 {
                    Err(CastleError::Other(
                        "Directive definitions cannot have directives.".into(),
                    ))?
                }
                let directive_definition = parse_directive_definition(&mut tokenizer)?;
                schema_definition.directives.insert(directive_definition.ident.clone(), directive_definition);
            },
            TokenKind::Keyword(Keyword::Input) => {
                let input_type_definition = parse_input_type_definition(&mut tokenizer, directives)?;
                schema_definition.input_types.insert(input_type_definition.ident.clone(), input_type_definition);
            },
            _ => Err(CastleError::Schema(
                format!("Expected item, found: {:?}", token.kind).into(),
                token.span,
            ))?
        }
    }
}

