
#[derive(Debug, PartialEq)]
pub enum Keyword {
    As,
    True,
    False,
    Match,
    Type,
    Enum,
    Fn,
    Into,
    Directive,
    On,
}

impl Keyword {
    pub fn from_str_to_option_keyword(s: &str) -> Option<Self> {
        match s {
            "as" => Some(Keyword::As),
            "true" => Some(Keyword::True),
            "false" => Some(Keyword::False),
            "match" => Some(Keyword::Match),
            "type" => Some(Keyword::Type),
            "enum" => Some(Keyword::Enum),
            "fn" => Some(Keyword::Fn),
            "into" => Some(Keyword::Into),
            "directive" => Some(Keyword::Directive),
            "on" => Some(Keyword::On),
            _ => None
        }
    }
}