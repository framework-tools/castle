use std::{io::Read};



use shared::castle_error::CastleError;

use crate::{tokenizer::{tokenizer::Tokenizer, tokenizer_utils::get_next_token_and_unwrap}, ast::syntax_definitions::{directive_definition::{DirectiveDefinition, DirectiveOnValue}, keyword::Keyword, fn_definition::FnDefinition, argument::ArgumentOrTuple}, token::token::{Punctuator, TokenKind, Identifier}, parsers::schema_parser::types::type_system::Type};



/// example: directive @test(ar: String) on FIELD
///  - directive keyword is consumed in previous function
///  - get next token and unwrap
///  - match token.kind to punctuator::At else throw error
///  - let token = get next token and unwrap
///  - let identifier_and_arguments be match token.kind to identifier and return the inner value
///  - get next token and unwrap
///  - match token.kind to punctuator::On else throw error
///  - let token = get next token and unwrap
///  - match token.kind to On enum and insert into directive.on
///  - return directive_definition
pub fn parse_directive_definition<R>(tokenizer: &mut Tokenizer<R>) -> Result<DirectiveDefinition, CastleError>
where R: Read{
    parse_token_and_consume_at_token(tokenizer)?;
    let (identifier, arguments) = parse_and_match_identifier_and_arguments(tokenizer)?;
    let arguments = ArgumentOrTuple::convert_arguments_to_identifier_and_type_arguments(arguments)?;
    parse_token_and_consume_on_token(tokenizer)?;
    let on = get_on_value(tokenizer)?;
    let function = FnDefinition::new(identifier, arguments, Type::Void);
    let directive_definition = DirectiveDefinition::new(function, on);
    Ok(directive_definition)
}

///  - get next token and unwrap
///  - match token.kind to punctuator::At else throw error
fn parse_token_and_consume_at_token<R>(tokenizer: &mut Tokenizer<R>) -> Result<(), CastleError>
where R: Read{
    let token = get_next_token_and_unwrap(tokenizer)?;
    match token.kind{
        TokenKind::Punctuator(Punctuator::At) => {},
        _ => return Err(CastleError::Schema(format!("Unexpected token while parsing directive , expected @ got: {:?}", token.kind).into(), token.span))
    };
    Ok(())
}

///  - let identifier_and_arguments be match token.kind to identifier and return the inner value
fn parse_and_match_identifier_and_arguments<R>(tokenizer: &mut Tokenizer<R>) -> Result<(Box<str>, Option<Vec<ArgumentOrTuple>>), CastleError>
where R: Read{
    let token = get_next_token_and_unwrap(tokenizer)?;
    match token.kind {
        TokenKind::Identifier(Identifier {name, arguments}) => Ok((name, arguments)),
        _ => return Err(CastleError::Schema(format!("Unexpected token, expected Identifier got: {:?}", token.kind).into(), token.span))
    }
}

///  - get next token and unwrap
///  - match token.kind to punctuator::On else throw error
fn parse_token_and_consume_on_token<R>(tokenizer: &mut Tokenizer<R>) -> Result<(), CastleError>
where R: Read{
    let token = get_next_token_and_unwrap(tokenizer)?;
    match token.kind{
        TokenKind::Keyword(Keyword::On) => { },
        _ => return Err(CastleError::Schema(format!("Unexpected token, expected On got: {:?}", token.kind).into(), token.span))
    };
    Ok(())
}

///  - match token.kind to On enum and insert into directive.on
fn get_on_value<R>(tokenizer: &mut Tokenizer<R>) -> Result<DirectiveOnValue, CastleError>
where R: Read{
    let token = get_next_token_and_unwrap(tokenizer)?;
    return match token.kind{
        TokenKind::DirectiveOnValue(on) => {Ok(on)},
        _ => return Err(CastleError::Schema(format!("Unexpected token, expected Identifier got: {:?}", token.kind).into(), token.span))
    };
}