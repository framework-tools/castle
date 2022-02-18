use std::io::Read;

use input_cursor::{Position, Cursor};
use shared::castle_error::CastleError;


use crate::{tokenizer::{tokenizer::Tokenizer, tokenizer_utils::{peek_next_token_and_unwrap, get_next_token_and_unwrap}, }, ast::syntax_definitions::{directive_definition::{Directive, DirectiveOnValue, }, argument::ArgumentOrTuple, }, token::{token::{TokenKind, Punctuator, Identifier}, Token},};

/// takes in tokenizer and returns parsed directive
///     - get next token
///     - match token
///     - if token is into keyword, get next token and parse directive and return
///     - else return none 
pub fn parse_directives<R>(tokenizer: &mut Tokenizer<R>) -> Result<Vec<Directive>, CastleError> 
where R: Read{
    let token = peek_next_token_and_unwrap(tokenizer)?;
    match token.kind {
        TokenKind::Punctuator(Punctuator::At) => {
            let directives = get_all_directives(tokenizer)?;
            return Ok(directives);
        },
        _ => return Ok(Vec::new())
    }
}

fn get_all_directives<R>(tokenizer: &mut Tokenizer<R>) -> Result<Vec<Directive>, CastleError>
where R: Read {
    let mut directives: Vec<Directive> = Vec::new();
    loop {
        let next_token_is_at_symbol = check_for_at_symbol(tokenizer)?;
        if !next_token_is_at_symbol {
            break;
        }
        tokenizer.next(true)?; // skip @
        let directive = get_directive(tokenizer)?;
        directives.push(directive);
        
    }
    return Ok(directives)
}

fn get_directive<R>(tokenizer: &mut Tokenizer<R>) -> Result<Directive, CastleError> 
where R: Read{
    let token = get_next_token_and_unwrap(tokenizer)?; // should be identifier
    return match token.kind { 
        TokenKind::Identifier(Identifier {name, arguments}) => {
            let arguments = ArgumentOrTuple::convert_arguments_to_identifier_and_type_arguments(arguments)?;
            Ok(Directive::new(name, arguments))
        },
        _ => Err(CastleError::UndefinedTypeOrEnumInSchema("Expected identifier for directive".into()))
    }
}

pub fn check_for_at_symbol<R>(tokenizer: &mut Tokenizer<R>) -> Result<bool, CastleError>
where R: Read{
    let option_peeked_token = tokenizer.peek(true)?;
    return match option_peeked_token {
        Some(token) => { 
            if token.kind == TokenKind::Punctuator(Punctuator::At) {
                return Ok(true)
            }
            else {
                return Ok(false)
            }
        },
        None => Err(CastleError::AbruptEOF("Error found in 'check_for_at_symbol'".into()))
    };
    
}

pub fn parse_directive_on_value<R>(cursor: &mut Cursor<R>, word: &str, start: Position) -> Result<Token, CastleError>
where R: Read{
    
    match word {
        "FIELD" => Ok(Token::new(TokenKind::DirectiveOnValue(DirectiveOnValue::Field), input_cursor::Span { start, end: cursor.pos() })),
        "ENUM_VARIANT" => Ok(Token::new(TokenKind::DirectiveOnValue(DirectiveOnValue::EnumVariant), input_cursor::Span { start, end: cursor.pos() })),
        _ => Err(CastleError::Lexer(format!("Expected directive on value, found: {}", word).into(), cursor.pos() ))
    }
}