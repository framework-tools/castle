use std::io::Read;

use shared::CastleError;

use crate::{tokenizer::{tokenizer::Tokenizer, tokenizer_utils::get_next_token_and_unwrap}, ast::syntax_definitions::{directive_definition::{DirectiveDefinition, self}, keyword::Keyword, fn_definition::FnDefinition}, token::token::{Punctuator, TokenKind, Identifier}};



/// example: directive @test(ar: String) on FIELD
///  - directive keyword is consumed in previous function
///  - get next token and unwrap
///  - match token.kind to punctuator::At else throw error
///  - let token = get next token and unwrap
///  - let identifier_and_arguments be match token.kind to identifier and return the inner value
///  
///  - get next token and unwrap
///  - match token.kind to punctuator::On else throw error
///  - let token = get next token and unwrap
///  - match token.kind to On enum and insert into directive.on
///  - return directive_definition
pub fn parse_directive_definition<R>(tokenizer: &mut Tokenizer<R>) -> Result<DirectiveDefinition, CastleError>
where R: Read{
    let token = get_next_token_and_unwrap(tokenizer)?;
    match token.kind{
        TokenKind::Punctuator(Punctuator::At) => { },
        _ => return Err(CastleError::Schema(format!("Unexpected token while parsing directive , expected @ got: {:?}", token.kind).into(), token.span))
    };

    let token = get_next_token_and_unwrap(tokenizer)?;
    let (identifier, arguments) = match token.kind{
        TokenKind::Identifier(Identifier {name, arguments}) => {(name, arguments)},
        _ => return Err(CastleError::Schema(format!("Unexpected token, expected Identifier got: {:?}", token.kind).into(), token.span))
    };

    let token = get_next_token_and_unwrap(tokenizer)?;
    match token.kind{
        TokenKind::Keyword(Keyword::On) => { },
        _ => return Err(CastleError::Schema(format!("Unexpected token, expected On got: {:?}", token.kind).into(), token.span))
    };

    let token = get_next_token_and_unwrap(tokenizer)?;
    let on = match token.kind{
        TokenKind::DirectiveOnValue(on) => {on},
        _ => return Err(CastleError::Schema(format!("Unexpected token, expected Identifier got: {:?}", token.kind).into(), token.span))
    };
    
    let function = FnDefinition::new(identifier, arguments, None);
    let directive_definition = DirectiveDefinition::new(function, on);
    Ok(directive_definition)
}