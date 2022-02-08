use std::io::Read;

use input_cursor::Cursor;
use shared::CastleError;

use crate::{tokenizer::{tokenizer::{Tokenizer, advance_and_parse_token}, self}, token::token::TokenKind, parser::schema_parser::parse_schema_field::skip_comma, ast::syntax_definitions::keyword::Keyword};

use super::{primitive_type::PrimitiveType, vec_type::VecType, option_type::OptionType, parse_directive::parse_directive,};



#[derive(Debug, PartialEq)]
pub enum Type {
    PrimitiveType(PrimitiveType),
    SchemaTypeOrEnum(Box<str>),
    VecType(VecType),
    OptionType(OptionType)
}

impl Type {
    pub fn new(s: String) -> Self {
        let option_primitive = PrimitiveType::from_str_to_option_primitive_type(&s);
        match option_primitive {
            Some(primitive) => Type::PrimitiveType(primitive),
            None => {
                let option_vec = VecType::new(&s);
                match option_vec {
                    Some(type_) => {
                        let vec_type = VecType::get_vec_type_struct(type_);
                        return Type::VecType(vec_type)
                    },
                    None => Type::SchemaTypeOrEnum(s.into())
                }
            }
        }
    }
}

/// takes in tokenizer and returns parsed type
///  - get next token
///  - match token kind to a Type
///  - else if token kind identifier parse identifier as schematype
///  - return parsed type
///  
pub fn parse_type<R>(tokenizer: &mut Tokenizer<R>) -> Result<Type, CastleError> 
where R: Read{
    let token = tokenizer.next(true)?;
    match token {
        Some(token) => match token.kind {
            TokenKind::PrimitiveType(primitive_type) => return Ok(Type::PrimitiveType(primitive_type)),
            TokenKind::Identifier(identifier) => return Ok(Type::SchemaTypeOrEnum(identifier.name)),
            TokenKind::VecType(vec_type) => return Ok(Type::VecType(vec_type)),
            TokenKind::OptionType(option_type) => return Ok(Type::OptionType(option_type)),
            _ => Err(CastleError::Schema(format!("Expected type, found: {:?}", token.kind).into(), token.span))
        },
        None => Err(CastleError::AbruptEOF("Error found in 'parse_type'".into())),
    }
}



