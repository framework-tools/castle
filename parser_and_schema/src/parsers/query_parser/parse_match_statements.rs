use std::{io::Read};
use shared::CastleError;
use token::Token;

use crate::{tokenizer::{tokenizer::{Tokenizer}, tokenizer_utils::{peek_next_token_and_unwrap, get_next_token_and_unwrap}}, ast::syntax_definitions::{match_statement::{MatchStatement, MatchArm}, expressions::{Expression, PrimitiveValue}, enum_definition::EnumValue, keyword::Keyword, want::{Want}}, token::{token::{TokenKind, Punctuator, Identifier, Numeric, self},}};

use super::{ parse_object_projection::{loop_through_tokens_and_parse_fields}};

pub fn parse_match_statements<R>(tokenizer: &mut Tokenizer<R>) -> Result<MatchStatement, CastleError> 
where R: Read {
    tokenizer.next(true)?; //consume the '{' 
    let match_statement = get_all_match_arms(tokenizer)?;
    return Ok(match_statement)
}

/// Parses a match statement
/// loop through tokens
/// parse object projection for each possible match statement
fn get_all_match_arms<R>(tokenizer: &mut Tokenizer<R>) -> Result<MatchStatement, CastleError>
where R: Read{
    let mut err = None;
    let mut match_statement = Vec::new();
    loop {
        let token = tokenizer.peek(true)?;
        match token {
            Some(token) => match &token.kind {
                TokenKind::Punctuator(Punctuator::CloseBlock) => {
                    tokenizer.next(true)?; // consume the close block
                    break;
                },
                TokenKind::EnumValue(_) | TokenKind::Identifier(_) | 
                TokenKind::BooleanLiteral(_) |
                TokenKind::StringLiteral(_) |
                TokenKind::NumericLiteral(_)
                => {
                    let token = get_next_token_and_unwrap(tokenizer)?; // consume the identifier
                    let match_arm = get_match_arm(tokenizer, token)?;
                    match_statement.push(match_arm);
                },
                TokenKind::Punctuator(Punctuator::Comma) => {
                    tokenizer.next(true)?; // consume the comma
                },
                _ => err = Some(Err(CastleError::Parser(format!("expected close block, value, or comma got : {:#?}", token).into(), token.span )))
            }
            None => err = Some(Err(CastleError::AbruptEOF(format!("expected close block, value, or comma got : None").into() )))
        };
    }
    if err.is_some() {
        return err.unwrap();
    } else {
        return Ok(match_statement)
    }
}

fn get_match_arm<R>(tokenizer: &mut Tokenizer<R>, token: Token) -> Result<MatchArm, CastleError>
where R: Read {
    let condition = get_condition(token)?;
    let identifier = condition.get_identifier();
    skip_arrow_syntax(tokenizer)?;
    let match_arm_fields = loop_through_tokens_and_parse_fields(tokenizer)?;
    return Ok(MatchArm::new(condition, match_arm_fields)); // empty used hashmap new here
}

fn get_condition(token: Token) -> Result<Expression, CastleError>{
    let condition = match token.kind {
        TokenKind::EnumValue(EnumValue { enum_parent, variant, data_type, .. }) => Some(Expression::EnumValue(EnumValue::new(enum_parent, variant, data_type))),
        TokenKind::Identifier(Identifier { name, arguments }) => Some(Expression::Identifier(Identifier::new(name, arguments))),
        TokenKind::StringLiteral(string_literal) => Some(Expression::PrimitiveValue(PrimitiveValue::String(string_literal))),
        TokenKind::NumericLiteral(numeric_literal) => Some(convert_numeric_token_to_expression(numeric_literal)),
        TokenKind::Keyword(Keyword::True) => Some(Expression::PrimitiveValue(PrimitiveValue::Boolean(true))),
        TokenKind::Keyword(Keyword::False) => Some(Expression::PrimitiveValue(PrimitiveValue::Boolean(false))),
        _ => None
    };
    if condition.is_some() {
        return Ok(condition.unwrap())
    }
    else{
        return Err(CastleError::AbruptEOF("token is not valid condition".into()))
    }
}

pub fn skip_arrow_syntax<R>(tokenizer: &mut Tokenizer<R>) -> Result<(), CastleError>
where R: Read {
    let peeked_token = peek_next_token_and_unwrap(tokenizer)?;
    return match peeked_token.kind {
        TokenKind::Punctuator(Punctuator::Assign) => {
            tokenizer.next(true)?; // consume the equal
            tokenizer.next(true)?; // consume the chevron
            tokenizer.next(true)?; // consume the open block
            Ok(())
        },
        _ => Err(CastleError::AbruptEOF("unexpected end of file in skip_arrow_syntax".into()))
    }
}

pub fn convert_numeric_token_to_expression(numeric_literal: Numeric) -> Expression {
    return match numeric_literal {
        Numeric::Integer(integer) => Expression::PrimitiveValue(PrimitiveValue::Int(integer)),
        Numeric::Float(float) => Expression::PrimitiveValue(PrimitiveValue::Float(float)),
        Numeric::UnsignedInteger(unsigned_integer) => Expression::PrimitiveValue(PrimitiveValue::UInt(unsigned_integer)),
    }
}