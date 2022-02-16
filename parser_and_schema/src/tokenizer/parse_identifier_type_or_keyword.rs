use std::{io::Read};

use input_cursor::{Cursor, Position};
use shared::CastleError;

use crate::{token::{Token}, parsers::schema_parser::types::parse_directive::parse_directive_on_value};

use super::{parse_keyword::{get_keyword_or_return_none}, parse_vec_type::get_vec_type_from_word, tokenizer::Tokenizer, parse_option_type::get_option_type_from_word, parse_enum_value::{parse_enum_value, peek_next_char_and_unwrap}, parse_identifier::parse_identifier_token, parse_primitive_type::get_primitive_type_or_return_none, parse_hashmap_type::get_hashmap_type_from_word};

pub fn parse_identifier_or_keyword_or_type<R>(tokenizer: &mut Tokenizer<R>, start: Position) -> Result<Token, CastleError> 
where R: Read {
    let (word, field_has_arguments) = get_word_from_chars(&mut tokenizer.cursor)?;
    if word == "Vec" { return get_vec_type_from_word(&mut tokenizer.cursor, word, start) }
    else if word == "Option" { return get_option_type_from_word(&mut tokenizer.cursor, word, start) }
    else if word == "HashMap" {return get_hashmap_type_from_word(&mut tokenizer.cursor, word, start)}
    else if word.contains("::") { return parse_enum_value(tokenizer, word, start)}
    else if word == "FIELD" || word == "ENUM_VARIANT" {return parse_directive_on_value(&mut tokenizer.cursor, &word, start)}


    //if field has arguments it must be an identifier (enums are already handled)
    if field_has_arguments { return parse_identifier_token(tokenizer, word, start, true) }

    // function below will check every case of word
    // and will return a keyword, type, or identifier token
    let token = get_keyword_or_return_none(tokenizer, &word, start)?;
    if token.is_some() { return Ok(token.unwrap()) }
    else { 
        let token = get_primitive_type_or_return_none(tokenizer, &word, start)?;
        if token.is_some() { return Ok(token.unwrap()) }
        else { return parse_identifier_token(tokenizer, word, start, false) }
    }
}

fn get_word_from_chars<R>(cursor: &mut Cursor<R>) -> Result<(String, bool), CastleError> where R: Read {
    let mut word = String::new();
    let mut gap_has_been_found = false;
    loop {
        let ch = peek_next_char_and_unwrap(cursor);
        if ch.is_err() { break } //if is end of file, break
        let ch = ch.unwrap();
        
        if ch == '(' { //start of arguments
            cursor.next_char()?;
            return Ok((word, true))
        } else if ch == '_' || (ch.is_ascii_alphanumeric() && !gap_has_been_found) {
            word.push(ch);
            cursor.next_char()?;
        }
        // if first letter uppercase & colon we can assume it's an enum value (For now)
        else if ch == ':' && word.chars().nth(0).unwrap().is_uppercase() {
            return Ok((get_enum_identifier_from_chars(cursor, word)?, false))
        } 
        else if ch == ' ' || ch == '\n' || ch == '\t' {
            gap_has_been_found = true;
            cursor.next_char()?;
        }
        else {
            break;
        }
    }
    return Ok((word, false))
}

fn get_enum_identifier_from_chars<R>(cursor: &mut Cursor<R>, word: String) -> Result<String, CastleError> 
where R: Read {
    let mut word = word;
    cursor.next_char()?; // skip first colon
    let ch = get_next_char_and_unwrap(cursor)?; //skip second colon
    if ch == ':' { //keep pushing for enum_value
        word.push(':');
        word.push(':');
        let (variant, _) = get_word_from_chars(cursor)?;
        word.push_str(variant.as_str());
        return Ok(word)
    } else {
        return Err(CastleError::Lexer("wrong syntax while parsing word from chars".into(), cursor.pos()))
    }
}

pub fn get_next_char_and_unwrap<R>(cursor: &mut Cursor<R>) -> Result<char, CastleError> 
where R: Read {
    let c = cursor.next_char(); 
    match c {
        Ok(c) => {
            match c {
                Some(c) => {
                    if let Ok(ch) = char::try_from(c) { return Ok(ch) }
                    else { return Err(CastleError::AbruptEOF("parsing word from chars".into())) }
                },
                None => return Err(CastleError::AbruptEOF("parsing word from chars".into()))
            }
        },
        Err(_e) => return Err(CastleError::AbruptEOF("get word from chars".into()))
    }
}