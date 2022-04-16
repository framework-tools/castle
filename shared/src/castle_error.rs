use std::fmt;

use input_cursor::{Position, Span};
use serde::{Deserialize, Serialize};

extern crate rmp_serde as rmps;

#[derive(Debug, Deserialize, Serialize)]
pub enum CastleError {
    IO(Box<str>),
    AbruptEOF(Box<str>),
    Syntax(Box<str>, Position),
    Parser(Box<str>, Span),
    Other(Box<str>),
    Schema(Box<str>, Span),
    Query(Box<str>, Span),
}

impl From<std::io::Error> for CastleError {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err.to_string().into())
    }
}


impl CastleError {
    pub fn syntax<Msg, Pos>(msg: Msg, pos: Pos) -> Self
    where
        Msg: Into<Box<str>>,
        Pos: Into<Position>,
    {
        Self::Syntax(msg.into(), pos.into())
    }

    pub fn parse<Msg, S>(msg: Msg, span: S) -> Self
    where
        Msg: Into<Box<str>>,
        S: Into<Span>,
    {
        Self::Parser(msg.into(), span.into())
    }
}

impl fmt::Display for CastleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IO(msg) => write!(f, "IO error: {}", msg),
            Self::AbruptEOF(msg) => write!(f, "Unexpected EOF: {}", msg),
            Self::Syntax(msg, pos) => write!(f, "Syntax error: {} at {}", msg, pos),
            Self::Parser(msg, span) => write!(f, "Parser error: {} at {}", msg, span),
            Self::Other(msg) => write!(f, "Error: {}", msg),
            Self::Schema(msg, span) => write!(f, "Schema error: {} at {}", msg, span),
            Self::Query(msg, span) => write!(f, "Query error: {} at {}", msg, span),
        }
    }
}

trait ExtendedErrorDisplay {
    fn extended_error(&self, src: &str) -> String;
}

impl ExtendedErrorDisplay for CastleError {
    fn extended_error(&self, src: &str) -> String {
        match self {
            Self::IO(msg) => msg.to_string(),
            Self::AbruptEOF(msg) =>  msg.to_string(),
            Self::Other(msg) =>  msg.to_string(),
            Self::Syntax(msg, pos) => pretty_print_lexer_error(msg, pos, src),
            Self::Parser(msg, span) => pretty_print_parser_error(msg, span, src),
            Self::Schema(msg, span) => pretty_print_parser_error(msg, span, src),
            Self::Query(msg, span) => pretty_print_parser_error(msg, span, src),
        }
    }
}


/// ## Get pretty errors that look like this
/// ```text
/// error: expected valid identifier, found keyword: type
///
/// 45 | type type {
///    |      ^^^^ expected valid identifier, found keyword: type
/// 46 |    first_name: String
/// 47 |    last_name: String
/// ```
///
/// or
///
/// ```text
/// error: expected valid identifier, found string literal: "hello\n\nhello"
///
/// 45 | type "hello
/// 46 | hello"
///    | ^^^^^^^^^^^^ expected valid identifier, found string literal: "hello\n\nhello"
/// 47 |     first_name: String
fn pretty_print_parser_error(msg: &str, span: &Span, src: &str) -> String {
    let src_lines: Vec<String> = src.lines().map(|line| line.to_string()).collect();
    let mut result_lines = vec![];

    result_lines.push(format!("error: {}\n", msg));

    let start_line_index = span.start().line_number() - 1;
    let end_line_index = span.end().line_number() - 1;
    let is_multiple_lines = start_line_index != end_line_index;

    for line_index in start_line_index..=end_line_index {
        result_lines.push(get_line_text(&line_index.to_string(), src_lines.get(line_index as usize).unwrap()));
    }

    let space_before_arrow = match is_multiple_lines {
        true => 0,
        false => span.start().column_number(),
    } as usize;

    let spaces = " ".repeat(space_before_arrow);
    let arrows = "^".repeat(span.end().column_number() as usize - space_before_arrow);

    result_lines.push(get_line_text(&"", &format!("{spaces}{arrows} {msg}")));

    for line_index in end_line_index+1..end_line_index+3 {
        if let Some(line) = src_lines.get(line_index as usize) {
            result_lines.push(get_line_text(&line_index.to_string(), line));
        }
    }

    result_lines.join("\n")
}

fn get_line_text(index: &str, line: &str) -> String {
    format!("{:>4} | {}", index, line)
}


fn pretty_print_lexer_error(msg: &str, pos: &Position, src: &str) -> String {
    let src_lines: Vec<String> = src.lines().map(|line| line.to_string()).collect();
    let mut result_lines = vec![];

    result_lines.push(format!("ERROR: {}\n", msg));

    let start_line_index = pos.line_number() - 1;

    result_lines.push(get_line_text(&start_line_index.to_string(), src_lines.get(start_line_index as usize).unwrap()));

    let space_before_arrow = " ".repeat(pos.column_number() as usize);

    result_lines.push(get_line_text(&"", &format!("{space_before_arrow}^ {msg}")));

    for line_index in start_line_index+1..start_line_index+3 {
        if let Some(line) = src_lines.get(line_index as usize)  {
            result_lines.push(get_line_text(&line_index.to_string(), line));
        }
    }

    result_lines.join("\n")
}


#[ignore]
#[test]
fn test_pretty_print_lexer_error() {
    let src = r#"
    type {

    }

    "#;
    let err = CastleError::syntax("got unexpected string", Position::new(2, 5));

    let result = err.extended_error(src);
    println!("{}", result);
}

#[ignore]
#[test]
fn test_pretty_print_parser_error() {
    let src = r#"
type "sdsdds
sdsd" {
    first_name: String
    last_name: String
}

    "#;
    let err = CastleError::parse("got unexpected string", Span::new(Position::new(2, 5), Position::new(3, 5)));

    let result = err.extended_error(src);
    println!("{}", result);
}

#[ignore]
#[test]
fn test_pretty_print_parser_error_with_one_line() {
    let src = r#"
type type {
    first_name: String
    last_name: String
}

    "#;
    let err = CastleError::parse("got unexpected keyword 'type', but expected identifier", Span::new(Position::new(2, 5), Position::new(2, 5 + 4)));

    let result = err.extended_error(src);
    println!("{}", result);
}