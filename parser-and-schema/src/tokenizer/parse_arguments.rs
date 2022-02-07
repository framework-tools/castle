use std::io::Read;

use input_cursor::Cursor;
use shared::CastleError;

use crate::ast::syntax_definitions::argument::Argument;

use super::tokenizer::advance_and_parse_token;

/// Takes in Cursor returns arguments token
///  - The '(' is already consumed
///  - if ')' return token
///  - else if, ',' create token from argument, then push token to arguments
///  - else push character to current argument
pub fn get_arguments<R>(cursor: &mut Cursor<R> ) -> Result<Vec<Argument>, CastleError> 
where R: Read {
    let mut arguments = Vec::new();
    loop {
        let end_of_arguments = unwrap_char_parse_argument_or_end(&mut arguments, cursor)?;
        if end_of_arguments { break; }
    }
    return Ok(arguments)
}

fn unwrap_char_parse_argument_or_end<R>(arguments: &mut Vec<Argument>, cursor: &mut Cursor<R>) -> Result<bool, CastleError>
where R: Read {
    let c = cursor.peek()?;
    match c {
        Some(ch) => {
            let ch = char::try_from(ch).ok().ok_or(CastleError::lex("invalid character", cursor.pos()))?;
            let end_of_arguments = check_for_end_or_parse_argument(arguments, cursor, ch);
            return end_of_arguments
        }
        None =>return Err(CastleError::AbruptEOF)
    }
}

fn check_for_end_or_parse_argument<R>(arguments: &mut Vec<Argument>, cursor: &mut Cursor<R>, ch: char) -> Result<bool, CastleError>
where R: Read {
    if ch == ')' {
        cursor.next_char()?; // skip close paren
        return Ok(true)
    } else if ch == ','{
        cursor.next_char()?; // skip comma
        get_argument_insert_into_arguments(arguments, cursor)?;
        return Ok(false)
    } else if ch == ' ' || ch == '\n'{ 
        cursor.next_char()?;
        return Ok(false)
    } else {
        get_argument_insert_into_arguments(arguments, cursor)?;
        return Ok(false)
    }
}

fn get_argument_insert_into_arguments<R>(arguments: &mut Vec<Argument>, cursor: &mut Cursor<R>) -> Result<(), CastleError> 
where R: Read {
    let token = advance_and_parse_token(cursor)?;
    println!("token before insert {:#?}", token);
    match token {
        Some(token) => {
            let argument = Argument::new(token, cursor)?;
            arguments.push(argument)
        },
        None => return Err(CastleError::AbruptEOF)
    };
    return Ok(())
}