use std::{io::Read, collections::HashMap};

use shared::CastleError;
use token::Token;

use crate::{tokenizer::{tokenizer::{Tokenizer}, tokenizer_utils::{peek_next_token_and_unwrap, get_next_token_and_unwrap}}, ast::syntax_definitions::{match_statement::{MatchStatement, MatchArm}, expressions::{Expression, PrimitiveValue}, enum_definition::EnumValue, keyword::Keyword, want::{Want, FieldsType}}, token::{token::{TokenKind, Punctuator, Identifier, Numeric, self},}};

use super::{parse_query::match_peeked_token_to_want, parse_object_projection::parse_object_projection};

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
    let mut match_statement = MatchStatement::new(Vec::new());
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
                    match_statement.statements.push(match_arm);
                },
                _ => break
            }
            None => break
        };
    }
    return Ok(match_statement)
}

fn get_match_arm<R>(tokenizer: &mut Tokenizer<R>, token: Token) -> Result<MatchArm, CastleError>
where R: Read {
    let condition = get_condition(tokenizer, token)?;
    skip_arrow_syntax(tokenizer)?;
    let mut match_arms = HashMap::new();
    loop {
        let end_of_match_arms = insert_arm_into_match_arm(tokenizer, &mut match_arms)?;
        if end_of_match_arms{
            break;
        }
    }
    let identifier = condition.get_identifier();
    return Ok(MatchArm::new(condition, Want::new_object_projection(identifier, FieldsType::Regular(match_arms), HashMap::new()))); // empty used hashmap new here
}

/// Peek next token
/// If token is an ident - continue
/// Peek next token
///If next token is a colon
///  - call parse_object_projection
///  - Insert obj into hashmap
/// Else: 
///  - let end_of_arm = insert_field_into_match_arm(tokenizer, &mut fields)?;
///  - if end_of_arm { break; }
fn insert_arm_into_match_arm<R>(tokenizer: &mut Tokenizer<R>, match_arms: &mut HashMap<Box<str>, Want>) -> Result<bool, CastleError> 
where R: Read {
    let peeked_token = peek_next_token_and_unwrap(tokenizer)?;
    return match &peeked_token.kind{
        TokenKind::Identifier(identifier) => {
            return insert_arm(tokenizer, match_arms);
        },
        TokenKind::Punctuator(Punctuator::Comma) => {
            tokenizer.next(true)?; // consume the comma
            Ok(false)
        },
        _ => Ok(true)
        
    };
}

fn insert_arm<R>(tokenizer: &mut Tokenizer<R>, match_arms: &mut HashMap<Box<str>, Want>) -> Result<bool, CastleError>
where R: Read {
    let token_after = tokenizer.peek_n(1, true)?.unwrap();
    if token_after.kind == TokenKind::Punctuator(Punctuator::Colon) {
        //if token after colon is match keyword
        // - need to parse match statements
        // - tokenizer.next(true)?; // consume the identifier
        // - tokenizer.next(true)?; // consume the colon
        // - tokenizer.next(true)?; // consume the match keyword
        // - parse match statements
        //else run insert_object_into_match_arm to insert object into hashmap

        let token_after_colon = tokenizer.peek_n(2, true)?.unwrap();
        if token_after_colon.kind == TokenKind::Keyword(Keyword::Match) {
            let identifier_token = get_next_token_and_unwrap(tokenizer)?; // consume the identifier
            tokenizer.next(true)?; // consume the colon
            tokenizer.next(true)?; // consume the match keyword
            let identifier = match identifier_token.kind {
                TokenKind::Identifier(identifier) => identifier,
                _ => return Err(CastleError::Parser("Expected identifier after colon in match arm".into(), identifier_token.span))
            };
            let match_statements = parse_match_statements(tokenizer)?;
            match_arms.insert(identifier.name.clone(), Want::new_object_projection(identifier.name, FieldsType::Match(match_statements), HashMap::new())); // empty used hashmap new here
            let peeked_token = peek_next_token_and_unwrap(tokenizer)?;
            match &peeked_token.kind{
                TokenKind::Punctuator(Punctuator::CloseBlock) => {
                    
                    return Ok(true)
                },
                _ => return Ok(false)
            }
        }
        else {
            let identifier_token = get_next_token_and_unwrap(tokenizer)?; // consume the identifier
            let identifier = match identifier_token.kind {
                TokenKind::Identifier(identifier) => identifier,
                _ => return Err(CastleError::Parser("Expected identifier after colon in match arm".into(), identifier_token.span))
            };
            return insert_object_into_match_arm(tokenizer, match_arms, identifier)
        }
        
    } 
    else { return insert_single_field_into_match_arm(tokenizer, match_arms) }
}

fn insert_object_into_match_arm<R>(tokenizer: &mut Tokenizer<R>, match_arms: &mut HashMap<Box<str>, Want>, identifier: Identifier) -> Result<bool, CastleError> 
where R: Read {
    let name = identifier.name.clone();
    let match_arm = parse_object_projection(identifier, tokenizer, false)?;
    match_arms.insert(name, match_arm);
    return Ok(false)
}

fn insert_single_field_into_match_arm<R>(tokenizer: &mut Tokenizer<R>, match_arms: &mut HashMap<Box<str>, Want>) -> Result<bool, CastleError> 
where R: Read {
    let end_of_arm = insert_field_into_match_arm(tokenizer, match_arms)?;
    if end_of_arm {
        Ok(true)
    }
    else {
        Ok(false)
    }
}

fn get_condition<R>(tokenizer: &mut Tokenizer<R>, token: Token) -> Result<Expression, CastleError>
where R: Read {
    let condition = match token.kind {
        TokenKind::EnumValue(EnumValue { enum_parent, variant, data_type, identifier }) => Some(Expression::EnumValue(EnumValue::new(enum_parent, variant, data_type))),
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

fn insert_field_into_match_arm<R>(tokenizer: &mut Tokenizer<R>, fields: &mut HashMap<Box<str>, Want>) -> Result<bool, CastleError> 
where R: Read {
    let token = get_next_token_and_unwrap(tokenizer)?;
    match token.kind {
        TokenKind::Identifier(identifier) => {
            let name = identifier.name.clone();
            let want = match_peeked_token_to_want(identifier, tokenizer)?;
            fields.insert(name,want);
            let peeked_token = peek_next_token_and_unwrap(tokenizer)?;
            match peeked_token.kind{
                TokenKind::Punctuator(Punctuator::CloseBlock) => {
                    tokenizer.next(true)?; // consume the close block
                    let peeked_token = peek_next_token_and_unwrap(tokenizer)?;
                    if peeked_token.kind == TokenKind::Punctuator(Punctuator::Comma) {
                        tokenizer.next(true)?; // consume the comma
                    }
                    return Ok(true)
                },
                _ => return Ok(false)
            }
        },
        
        _ => return Err(CastleError::AbruptEOF("token is not valid condition".into()))
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