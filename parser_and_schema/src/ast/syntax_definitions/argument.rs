use std::{io::Read, collections::HashMap};


use shared::castle_error::CastleError;

use crate::{token::{Token, token::{TokenKind, Identifier, Punctuator}}, parsers::schema_parser::types::type_system::{Type, parse_type}, tokenizer::{tokenizer::{Tokenizer}, tokenizer_utils::{get_next_token_and_unwrap}}};

use super::expressions::PrimitiveValue;

//For Schema Resolvers/Functions
pub type IdentifierAndTypeArgument = (Box<str>, Type);
//For Query Object Projections/Resolvers
pub type IdentifierAndValueArgument = (Box<str>, PrimitiveValue);

#[derive(Debug, PartialEq)]
pub enum ArgumentOrTuple {
    PrimitiveValue(PrimitiveValue),
    IdentifierAndType(IdentifierAndTypeArgument),
    IdentifierAndValue(IdentifierAndValueArgument),
}

impl ArgumentOrTuple {
    pub fn new<R>(token: Token, tokenizer: &mut Tokenizer<R>) -> Result<Self, CastleError> 
    where R: Read {
        let argument = match token.kind {
            TokenKind::Identifier(Identifier { name, ..}) => parse_identifier_argument(name, tokenizer)?, //can be ident, type, enum or a combo
            //parse option argument
            _ => parse_primitive_value_argument(token)?
        };
        return Ok(argument)
    }
    pub fn convert_arguments_to_identifier_and_value_arguments(arguments: Option<Vec<ArgumentOrTuple>>) -> Result<HashMap<Box<str>, IdentifierAndValueArgument>, CastleError>{
        let mut arguments_for_query_object = HashMap::new();
        if arguments.is_none() {
            return Ok(arguments_for_query_object)
        } else {
            let arguments = arguments.unwrap();
            for argument in arguments {
                match argument {
                    ArgumentOrTuple::IdentifierAndValue(identifier_and_value) => {
                        let (identifier, primitive_value) = identifier_and_value;
                        arguments_for_query_object.insert(identifier.clone(), (identifier, primitive_value));
                    },
                    _ => return Err(CastleError::IncorrectArgumentType(format!("Expected identifier and value got different argument type, found: {:?}", argument).into()))
                };
            }
        }
        return Ok(arguments_for_query_object)
    }
    
    pub fn convert_arguments_to_identifier_and_type_arguments(arguments: Option<Vec<ArgumentOrTuple>>) -> Result<HashMap<Box<str>, IdentifierAndTypeArgument>, CastleError>{
        let mut arguments_for_query_object = HashMap::new();
        if arguments.is_none() {
            return Ok(arguments_for_query_object)
        } else {
            let arguments = arguments.unwrap();
            for argument in arguments {
                match argument {
                    ArgumentOrTuple::IdentifierAndType(identifier_and_type) => {
                        let (identifier, primitive_value) = identifier_and_type;
                        arguments_for_query_object.insert(identifier.clone(), (identifier, primitive_value));
                    },
                    _ => return Err(CastleError::IncorrectArgumentType(format!("Expected identifier and value got different argument type, found: {:?}", argument).into()))
                };
            }
        }
        return Ok(arguments_for_query_object)
    }

    pub fn convert_arguments_as_hashmaps_to_identifier_and_type_arguments(arguments: HashMap<Box<str>, ArgumentOrTuple>) -> Result<HashMap<Box<str>, IdentifierAndTypeArgument>, CastleError>{
        let mut arguments_for_query_object = HashMap::new();
        for (_identifier, argument) in arguments {
            match argument {
                ArgumentOrTuple::IdentifierAndType(identifier_and_type) => {
                    let (identifier, primitive_value) = identifier_and_type;
                    arguments_for_query_object.insert(identifier.clone(), (identifier, primitive_value));
                },
                _ => return Err(CastleError::IncorrectArgumentType(format!("Expected identifier and value got different argument type, found: {:?}", argument).into()))
            };
        }
        return Ok(arguments_for_query_object)
    }
}

fn parse_identifier_argument<R>(name: Box<str>, tokenizer: &mut Tokenizer<R>) -> Result<ArgumentOrTuple, CastleError>
where R: Read {
    let token = tokenizer.next(true)?;
    match token {
        Some(token) => match_token_to_parse_argument(token, tokenizer, name),
        None => return Err(CastleError::AbruptEOF("Error found in 'parse_identifier_argument'".into()))
    }
}

pub fn match_token_to_parse_argument<R>(token: Token, tokenizer:&mut Tokenizer<R>, name: Box<str>) -> Result<ArgumentOrTuple, CastleError> 
where R: Read {
    match token.kind {
        TokenKind::Punctuator(Punctuator::Colon) => { //Identifier and Type Argument
            let token = get_next_token_and_unwrap(tokenizer)?;
            let is_primitive_value = PrimitiveValue::check_if_primitive_value(&token.kind);
            if is_primitive_value {
                let primitive_value = PrimitiveValue::new_from_token_kind(token)?;
                let ident_and_type: IdentifierAndValueArgument = (name, primitive_value);
                return Ok(ArgumentOrTuple::IdentifierAndValue(ident_and_type));
            } else {
                let type_ = parse_type(token)?;
                let ident_and_type: IdentifierAndTypeArgument = (name, type_);
                return Ok(ArgumentOrTuple::IdentifierAndType(ident_and_type));
            }
        },
        _ => return Err(CastleError::Schema(format!("Expected ':' after identifier '{}'", name).into(), token.span))
    }
}

fn parse_primitive_value_argument(token: Token) -> Result<ArgumentOrTuple, CastleError> {
    let primitive_value = PrimitiveValue::new_from_token_kind(token)?;
    return Ok(ArgumentOrTuple::PrimitiveValue(primitive_value))
}