
use shared::CastleError;

use crate::{token::{token::TokenKind, Token}};

use super::{primitive_type::PrimitiveType, vec_type::VecType, option_type::OptionType, };


#[derive(Debug, PartialEq)]
pub enum Type {
    PrimitiveType(PrimitiveType),
    SchemaTypeOrEnum(Box<str>),
    VecType(VecType),
    HashMapType(Box<Type>),
    OptionType(OptionType),
    Void //Needs to be removed
}

impl Type {
    pub fn new_primitve_or_schema_or_enum_type(s: String) -> Self {
        let option_primitive = PrimitiveType::from_str_to_option_primitive_type(&s);
        match option_primitive {
            Some(primitive) => Type::PrimitiveType(primitive),
            None => return Type::SchemaTypeOrEnum(s.into())
        }
    }
}

/// takes in tokenizer and returns parsed type
///  - get next token
///  - match token kind to a Type
///  - else if token kind identifier parse identifier as schematype
///  - return parsed type
///  
pub fn parse_type(token: Token) -> Result<Type, CastleError> {
    match token.kind {
        TokenKind::PrimitiveType(primitive_type) => return Ok(Type::PrimitiveType(primitive_type)),
        TokenKind::Identifier(identifier) => return Ok(Type::SchemaTypeOrEnum(identifier.name)),
        TokenKind::VecType(vec_type) => return Ok(Type::VecType(vec_type)),
        TokenKind::OptionType(option_type) => return Ok(Type::OptionType(option_type)),
        TokenKind::HashMapType(value_type) => return Ok(Type::HashMapType(value_type.into())),
        _ => Err(CastleError::Schema(format!("Expected type, found: {:?}", token.kind).into(), token.span))
    }
}

pub fn get_type_from_string(type_as_str: &str) -> Type {
    if type_as_str.len() > 5 {
        if &type_as_str[..4] == "Vec<" { 
            println!("working");
            return VecType::new(&type_as_str)
        }

        if type_as_str.len() > 9 {
            if &type_as_str[..7] == "Option<" { 
                return OptionType::new(&type_as_str)
            }
        }

        if type_as_str.len() > 10 {
            if &type_as_str[..8] == "HashMap<" { 
                return Type::HashMapType(Box::new(get_type_from_string(&type_as_str[8..type_as_str.len() - 1])))
            }
        }
    }
    return Type::new_primitve_or_schema_or_enum_type(type_as_str.to_string()) 
}