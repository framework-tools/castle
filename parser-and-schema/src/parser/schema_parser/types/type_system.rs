use std::io::Read;

use input_cursor::Cursor;
use shared::CastleError;

use crate::{tokenizer::{tokenizer::{Tokenizer, advance_and_parse_token}, self}, token::token::TokenKind, parser::schema_parser::parse_schema_field::skip_comma};

use super::{primitive_type::PrimitiveType, vec_type::VecType};



#[derive(Debug, PartialEq)]
pub enum Type {
    PrimitiveType(PrimitiveType),
    SchemaTypeOrEnum(Box<str>),
    VecType(VecType),
    OptionType(Box<Type>)
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
    let token = advance_and_parse_token(tokenizer)?;
    println!("token: {:#?}", token);
    match token {
        Some(token) => match token.kind {
            TokenKind::PrimitiveType(primitive_type) => return get_primitive_type(primitive_type, tokenizer),
            TokenKind::Identifier(identifier) => return get_schema_type(identifier.name, tokenizer),
            TokenKind::VecType(vec_type) => return get_vec_type(vec_type, tokenizer),
            _ => Err(CastleError::Schema(format!("Expected type, found: {:?}", token.kind).into(), token.span))
        },
        None => Err(CastleError::AbruptEOF)
    }
}

fn get_primitive_type<R>(primitive_type: PrimitiveType, tokenizer: &mut Tokenizer<R>) -> Result<Type, CastleError> 
where R: Read{
    return Ok(Type::PrimitiveType(primitive_type))
}

fn get_schema_type<R>(identifier: Box<str>, tokenizer: &mut Tokenizer<R>) -> Result<Type, CastleError> 
where R: Read{
    return Ok(Type::SchemaTypeOrEnum(identifier))
}

fn get_vec_type<R>(vec_type: VecType, tokenizer: &mut Tokenizer<R>) -> Result<Type, CastleError>
    where R: Read{
    return Ok(Type::VecType(vec_type))
}