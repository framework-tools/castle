use std::{io::Read, collections::HashMap};



use shared::castle_error::CastleError;

use crate::{tokenizer::{tokenizer::Tokenizer}, ast::syntax_definitions::{field_definition::{FieldDefinition}, argument::{ArgumentOrTuple}, directive_definition::Directive}, parsers::schema_parser::{types::{parse_type::{Type, parse_type}, parse_directive::parse_directives}}, token::token::{Identifier, TokenKind, Punctuator}};

pub fn parse_function<R: Read>(tokenizer: &mut Tokenizer<R>) -> Result<FieldDefinition, CastleError> {
    let (name, args, return_type, directives);
    let token = tokenizer.expect_next(true)?;
    match token.kind {
        TokenKind::Identifier(identifier ) => {
            (name, args) = get_fn_name_and_arguments(identifier)?;
            return_type = get_fn_return_type( tokenizer)?;
            directives = get_fn_directives(tokenizer)?;
        },
        _ => return Err(CastleError::Schema(format!("6. Expected identifier, found: {:?}", token.kind).into(), token.span))
    }
    let function_definition = FieldDefinition::new(name, args, return_type, directives);
    return Ok(function_definition);
}

fn get_fn_name_and_arguments(identifier: Identifier)
-> Result<(Box<str>, HashMap<Box<str>, Type>), CastleError> {
    let name = identifier.name;
    let arguments = ArgumentOrTuple::convert_arguments_to_identifier_and_type_arguments(identifier.arguments)?;
    return Ok((name, arguments))
}

fn get_fn_return_type<R>(tokenizer: &mut Tokenizer<R>)
-> Result<Type, CastleError> where R: Read {
    tokenizer.expect_punctuator(Punctuator::Colon, true)?;
    Ok(parse_function_return_type(tokenizer)?)
}

fn parse_function_return_type<R>(tokenizer: &mut Tokenizer<R>) -> Result<Type, CastleError>
where R: Read {
    tokenizer.expect_punctuator(Punctuator::Colon, true)?;
    let token = tokenizer.expect_next(true)?;
    let return_type = parse_type(tokenizer)?;
    return Ok(return_type)
}

fn get_fn_directives<R: Read>(tokenizer: &mut Tokenizer<R>) -> Result<Vec<Directive>, CastleError> {
    let peeked_token = tokenizer.peek(true)?;
    match peeked_token {
        Some(token) => {
            match token.kind{
                TokenKind::Punctuator(Punctuator::At) => {
                    return parse_directives(tokenizer);
                },
                _ => return Ok(Vec::new())
            }
        }
        None => return Ok(Vec::new())
    }
}
