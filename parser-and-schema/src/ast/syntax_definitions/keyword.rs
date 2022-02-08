
#[derive(Debug, PartialEq)]
pub enum Keyword {
    As,
    True,
    False,
    None,
    Some,
    Match,
    Type,
    Enum,
    Fn,
    Into
}

impl Keyword {
    pub fn from_str_to_option_keyword(s: &str) -> Option<Self> {
        match s {
            "as" => Some(Keyword::As),
            "true" => Some(Keyword::True),
            "false" => Some(Keyword::False),
            "none" => Some(Keyword::None),
            "some" => Some(Keyword::Some),
            "match" => Some(Keyword::Match),
            "type" => Some(Keyword::Type),
            "enum" => Some(Keyword::Enum),
            "fn" => Some(Keyword::Fn),
            "into" => Some(Keyword::Into),
            _ => None
        }
    }
}