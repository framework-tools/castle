use std::{io::Read, collections::HashMap};

use shared::CastleError;

use crate::{tokenizer::{tokenizer::Tokenizer, tokenizer_utils::get_next_token_and_unwrap}, ast::syntax_definitions::{fn_definition::{FnDefinition}, argument::{IdentifierAndTypeArgument, ArgumentOrTuple}}, parsers::schema_parser::{types::type_system::{Type, parse_type}}, token::token::{Identifier, TokenKind, Punctuator}};

pub fn parse_function<R>(tokenizer: &mut Tokenizer<R>) -> Result<FnDefinition, CastleError>
where R: Read {

    let (name, args, return_type);
    let token = get_next_token_and_unwrap(tokenizer)?;
    match token.kind {
        TokenKind::Identifier(identifier ) => {
            (name, args) = get_fn_name_and_arguments(identifier)?;
            return_type = get_fn_return_type( tokenizer)?;
        },
        _ => return Err(CastleError::Schema(format!("6. Expected identifier, found: {:?}", token.kind).into(), token.span))
    }
    let function_definition = FnDefinition::new(name, args, return_type);
    return Ok(function_definition);
}

fn get_fn_name_and_arguments(identifier: Identifier)
-> Result<(Box<str>, HashMap<Box<str>, IdentifierAndTypeArgument>), CastleError> {
    let name = identifier.name;
    let arguments = ArgumentOrTuple::convert_arguments_to_identifier_and_type_arguments(identifier.arguments)?;
    return Ok((name, arguments))
}

fn get_fn_return_type<R>(tokenizer: &mut Tokenizer<R>)
-> Result<Type, CastleError> where R: Read {
    let token = get_next_token_and_unwrap(tokenizer)?;
    return match token.kind {
        TokenKind::Punctuator(Punctuator::Sub) => { // dash from arrow syntax
            Ok(parse_function_return_type(tokenizer)?)
        }
        _ => return Err(CastleError::Schema(format!("Expected sub operator, found: {:?}", token.kind).into(), token.span))
    }
}

fn parse_function_return_type<R>(tokenizer: &mut Tokenizer<R>) -> Result<Type, CastleError>
where R: Read {
    tokenizer.next(false)?; //skip chevron right from return arrow
    let token = get_next_token_and_unwrap(tokenizer)?;
    let return_type = parse_type(token, tokenizer)?;
    return Ok(return_type)
}