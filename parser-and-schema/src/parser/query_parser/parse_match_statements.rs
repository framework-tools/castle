use std::{io::Read, collections::HashMap};

use shared::CastleError;

use crate::{tokenizer::tokenizer::{Tokenizer}, ast::syntax_definitions::{match_statement::{MatchStatement, MatchArm}, expressions::{Expression, PrimitiveValue}, enum_definition::EnumValue, keyword::Keyword, want::Want}, token::{token::{TokenKind, Punctuator, Identifier, Numeric}, self}, parser::schema_parser::types::parse_directive::unwrap_peeked_token};

use super::{parse_query::match_peeked_token_to_want};

pub fn parse_match_statements<R>(tokenizer: &mut Tokenizer<R>, name: Box<str>) -> Result<Option<MatchStatement>, CastleError> 
where R: Read {
    let mut match_statement = None;
    let is_match_statement = check_colon_and_match(tokenizer)?;
    if is_match_statement{
        match_statement = Some(get_all_match_arms(tokenizer)?);
    }
    return Ok(match_statement)
}

fn check_colon_and_match<R>(tokenizer: &mut Tokenizer<R>) -> Result<bool, CastleError>
where R: Read {
    let option_peeked_token = tokenizer.peek(true)?;
    match option_peeked_token {
        Some(peeked_token) => return match peeked_token.kind {
            TokenKind::Punctuator(Punctuator::Colon) => {
                tokenizer.next(true)?; // skip colon
                tokenizer.next(true)?; // skip match
                tokenizer.next(true)?; // skip open block
                Ok(true)
            },
            _ => Ok(false)
        },
        None => return Ok(false)
    };
}

/// Parses a match statement
/// loop through tokens
/// parse object projection for each possible match statement
fn get_all_match_arms<R>(tokenizer: &mut Tokenizer<R>) -> Result<MatchStatement, CastleError>
where R: Read{
    let mut match_statement = MatchStatement::new(Vec::new());
    loop {
        let token = tokenizer.peek(true)?;
        match token {
            Some(token) => match &token.kind {
                TokenKind::Punctuator(Punctuator::CloseBlock) => {
                    tokenizer.next(true)?; // consume the close block
                    break;
                },
                TokenKind::Identifier(Identifier { name, .. }) => {
                    let match_arm = get_match_arm(tokenizer)?;
                    match_statement.statements.push(match_arm);
                },
                _ => break
            }
            None => break
        };
    }
    return Ok(match_statement)
}

fn get_match_arm<R>(tokenizer: &mut Tokenizer<R>) -> Result<MatchArm, CastleError>
where R: Read {
    let condition = get_condition(tokenizer)?;
    skip_arrow_syntax(tokenizer)?;
    let mut fields = HashMap::new();
    loop {
        let peeked_token = unwrap_peeked_token(tokenizer)?;
        match &peeked_token.kind{
            TokenKind::Identifier(_) => {
                let token = tokenizer.next(true)?;
                let token = token.unwrap();
                match token.kind {
                    TokenKind::Identifier(identifier) => {
                        let name = identifier.name.clone();
                        let want = match_peeked_token_to_want(identifier, tokenizer)?;
                        fields.insert(name,want);
                    },
                    _ => break
                };
                
            },
            _ => break
            
        };
    } 
    return Ok(MatchArm::new(condition, Want::new_inner_object(Some(fields), None)))
}

pub fn skip_arrow_syntax<R>(tokenizer: &mut Tokenizer<R>) -> Result<(), CastleError>
where R: Read {
    let peeked_token = unwrap_peeked_token(tokenizer)?;
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

fn get_condition<R>(tokenizer: &mut Tokenizer<R>) -> Result<Expression, CastleError>
where R: Read {
    let option_token = tokenizer.next(true)?;
    let token = match option_token {
        Some(token) => token,
        None => return Err(CastleError::AbruptEOF("unexpected end of file in get_condition".into()))
    };
    let condition = match token.kind {
        TokenKind::EnumValue(EnumValue { enum_parent, variant, data_type }) => Some(Expression::EnumValue(EnumValue::new(enum_parent, variant, data_type))),
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

pub fn convert_numeric_token_to_expression(numeric_literal: Numeric) -> Expression {
    return match numeric_literal {
        Numeric::Integer(integer) => Expression::PrimitiveValue(PrimitiveValue::Int(integer)),
        Numeric::Float(float) => Expression::PrimitiveValue(PrimitiveValue::Float(float)),
        Numeric::UnsignedInteger(unsigned_integer) => Expression::PrimitiveValue(PrimitiveValue::UInt(unsigned_integer)),
    }
}