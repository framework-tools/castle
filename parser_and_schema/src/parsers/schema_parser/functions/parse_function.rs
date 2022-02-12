use std::{io::Read};

use shared::CastleError;

use crate::{tokenizer::{tokenizer::Tokenizer, tokenizer_utils::get_next_token_and_unwrap}, ast::syntax_definitions::{fn_definition::{FnDefinition, FnStatement}}, parsers::schema_parser::{types::type_system::{Type, parse_type}}, token::token::{Identifier, TokenKind, Punctuator}};

pub fn parse_function<R>(tokenizer: &mut Tokenizer<R>) -> Result<FnDefinition, CastleError>
where R: Read {
    let mut function_definition = FnDefinition::initalise();

    let token = get_next_token_and_unwrap(tokenizer)?;
    match token.kind {
        TokenKind::Identifier(identifier ) => {
            get_fn_name_and_arguments(&mut function_definition, identifier)?;
            get_fn_return_type(&mut function_definition, tokenizer)?;
        },
        _ => return Err(CastleError::Schema(format!("6. Expected identifier, found: {:?}", token.kind).into(), token.span))
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

fn get_fn_return_type<R>(function_definition: &mut FnDefinition, tokenizer: &mut Tokenizer<R>)
-> Result<(), CastleError> where R: Read {
    let token = get_next_token_and_unwrap(tokenizer)?;
    match token.kind {
        TokenKind::Punctuator(Punctuator::Sub) => { // dash from arrow syntax
            function_definition.return_type = Some(parse_function_return_type(tokenizer)?);
        }
        _ => return Err(CastleError::Schema(format!("Expected sub operator, found: {:?}", token.kind).into(), token.span))
    }
    return Ok(())
}

fn parse_function_return_type<R>(tokenizer: &mut Tokenizer<R>) -> Result<Type, CastleError>
where R: Read {
    tokenizer.next(false)?; //skip chevron right from return arrow
    let return_type = parse_type(tokenizer)?;
    return Ok(return_type)
}