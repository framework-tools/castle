use std::{io::Read, char, collections::HashMap};

use input_cursor::{Position, Cursor, Span};
use shared::CastleError;

use crate::{token::{Token, token::TokenKind}, ast::syntax_definitions::{enum_definition::{EnumValue, EnumDataType}, argument::Argument}, parsers::schema_parser::parse_schema_type::check_token_and_parse_schema_field_or_break, tokenizer::tokenizer_utils::get_next_token_and_unwrap};

use super::{tokenizer::Tokenizer, parse_arguments::get_arguments};
//
/// Parses enum value as token
/// - push characters to enum_parent until ':' ':' is found (two times in a row)
/// - After this, keep looping but push characters to variant string
/// - End Loop
///
/// - Peek next character
/// - If open parenthesis, parse tuple value
/// - Else if open block, parse object value
/// - Else, enum is a unit variant
/// - place enum value in token and return token

pub fn parse_enum_value<R>(tokenizer: &mut Tokenizer<R>, word: String, start: Position) -> Result<Token, CastleError>
where R: Read {
    let identifier: Box<str> = word.clone().into();
    let (enum_parent, variant) = get_enum_parent_and_variant(tokenizer, word)?;
    let data_type = get_enum_data_type(tokenizer)?;
    let enum_value = EnumValue { identifier, enum_parent, variant, data_type };
    return Ok(Token::new(TokenKind::EnumValue(enum_value), Span::new(start, tokenizer.cursor.pos())));
}


fn get_enum_parent_and_variant<R>(tokenizer: &mut Tokenizer<R>, word: String) -> Result<(Box<str>, Box<str>), CastleError>
where R: Read {
    let mut enum_parent = String::new();
    let mut variant = String::new();
    let mut colon_count = 0;

    for ch in word.chars() {
        if colon_count == 2 { variant.push(ch); }
        else if ch == ':' { colon_count += 1; }
        else if colon_count == 0 { enum_parent.push(ch); }
    }
    return Ok((enum_parent.into(), variant.into()))
}

#[test]
fn test_parse_emum_value(){
    let mut tokenizer = Tokenizer::new("Color::Red ".as_bytes());
    let token = get_next_token_and_unwrap(&mut tokenizer);
    let token = token.unwrap(); 
    let expected_enum = EnumValue {
        identifier: "Color::Red".into(),
        enum_parent: "Color".into(),
        variant: "Red".into(),
        data_type: EnumDataType::EnumUnit
    };
    let expected_token_kind = TokenKind::EnumValue(expected_enum);
    assert_eq!(token.kind, expected_token_kind);
}

fn get_enum_data_type<R>(tokenizer: &mut Tokenizer<R>) -> Result<EnumDataType, CastleError>
where R: Read {
    let peeked_char = peek_next_char_and_unwrap(&mut tokenizer.cursor)?;

    if peeked_char == '(' { return parse_enum_tuple_value(tokenizer) } 
    else if peeked_char == '{' { return parse_enum_object_value(tokenizer) }
    else { return Ok(EnumDataType::EnumUnit) }
}

fn parse_enum_tuple_value<R>(tokenizer: &mut Tokenizer<R>) -> Result<EnumDataType, CastleError>
where R: Read {
    tokenizer.cursor.next_char()?; // skip open parenthesis
    let tuple_values = get_arguments(tokenizer)?;
    return Ok(EnumDataType::EnumTuple(tuple_values))
}

fn parse_enum_object_value<R>(tokenizer: &mut Tokenizer<R>) -> Result<EnumDataType, CastleError>
where R: Read {
    tokenizer.cursor.next_char()?; // skip open block
    let mut object_values = HashMap::new();
    loop {
        let end_of_schema_type = check_token_and_parse_schema_field_or_break(tokenizer, &mut object_values)?;
        if end_of_schema_type { break; }
    }
    return Ok(EnumDataType::EnumObject(object_values))
}


pub fn peek_next_char_and_unwrap<R>(cursor: &mut Cursor<R>) -> Result<char, CastleError> 
where R: Read {
    let c = cursor.peek_char();
    match c {
        Ok(c) => match c {
            //convert c from u32 to char
            Some(c) => match char::try_from(c) {
                Ok(ch) => Ok(ch),
                Err(_) => return Err(CastleError::AbruptEOF("peek_next_char_unwrap_and_convert_to_char".into()))
            },
            None => return Err(CastleError::AbruptEOF("peek_next_char_unwrap_and_convert_to_char".into()))
        },
        Err(e) => return Err(CastleError::AbruptEOF("peek_next_char_unwrap_and_convert_to_char".into()))
    }
}