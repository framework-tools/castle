use std::{io::Read};

use shared::CastleError;

use crate::{tokenizer::{tokenizer::Tokenizer}, ast::syntax_definitions::{fn_definition::{FnDefinition, FnStatement}}, parser::schema_parser::{types::type_system::{Type, parse_type}}, token::token::{Identifier, TokenKind, Punctuator}};

pub fn parse_function<R>(tokenizer: &mut Tokenizer<R>) -> Result<FnDefinition, CastleError>
where R: Read {
    let mut function_definition = FnDefinition::new();

    let token = tokenizer.next(true)?;
    match token {
        Some(token) => match token.kind {
            TokenKind::Identifier(identifier ) => {
                get_fn_name_and_arguments(&mut function_definition, identifier)?;
                get_fn_return_type_and_body(&mut function_definition, tokenizer)?;
            },
            _ => return Err(CastleError::Schema(format!("6. Expected identifier, found: {:?}", token.kind).into(), token.span))
        },
        None => return Err(CastleError::AbruptEOF("Error found in 'parse_function'".into()))
    }

    return Ok(function_definition);
}

fn get_fn_name_and_arguments(function_definition:  &mut FnDefinition, identifier: Identifier)
-> Result<(), CastleError> {
    function_definition.name = identifier.name;
    match identifier.arguments {
        Some(arguments) => { function_definition.args = Some(arguments); }
        None => {}
    };
    return Ok(());
}

fn get_fn_return_type_and_body<R>(function_definition: &mut FnDefinition, tokenizer: &mut Tokenizer<R>)
-> Result<(), CastleError> where R: Read {
    let token = tokenizer.next(true)?;
    match token {
        Some(token) => match token.kind {
            TokenKind::Punctuator(Punctuator::Sub) => { // dash from arrow syntax
                function_definition.return_type = Some(parse_function_return_type(tokenizer)?);
                function_definition.body = parse_block(tokenizer)?;
            }
            TokenKind::Punctuator(Punctuator::OpenBlock) => { // return type is None
                function_definition.body = parse_block(tokenizer)?;
            }
            _ => return Err(CastleError::Schema(format!("Expected open block, found: {:?}", token.kind).into(), token.span))
        }
        None => return Err(CastleError::AbruptEOF("Error found in 'get_fn_return_type_and_body'".into())),
    }
    return Ok(())
}

fn parse_function_return_type<R>(tokenizer: &mut Tokenizer<R>) -> Result<Type, CastleError>
where R: Read {
    tokenizer.next(false)?; //skip chevron right from return arrow
    let return_type = parse_type(tokenizer)?;
    tokenizer.next(false)?; //skip open block
    return Ok(return_type)
}

fn parse_block<R>(tokenizer: &mut Tokenizer<R>) -> Result<Vec<FnStatement>, CastleError>
where R: Read {
    tokenizer.next(true)?; //skip close block
    return Ok(Vec::new())
}