

// these keywords are currently shared between

use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Keyword {
    As, // as
    True, // true
    False, // false
    Match, // match
    Type, // type
    Enum, // enum
    Directive, // directive
    Input, // input
}

impl FromStr for Keyword {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "as" => Ok(Keyword::As), // first_name as email
            "true" => Ok(Keyword::True),
            "false" => Ok(Keyword::False),
            "match" => Ok(Keyword::Match),
            "type" => Ok(Keyword::Type),
            "enum" => Ok(Keyword::Enum),
            "directive" => Ok(Keyword::Directive),
            "input" => Ok(Keyword::Input),
            _ => Err(format!("unexpected keyword: {}", s)),
        }
    }
}