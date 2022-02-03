
#[derive(Debug, PartialEq)]
pub enum Keyword {
    As,
    True,
    False,
    None,
    Some,
    Match
}

impl From<&str> for Keyword {
    fn from(s: &str) -> Self {
        match s {
            "as" => Keyword::As,
            "true" => Keyword::True,
            "false" => Keyword::False,
            "none" => Keyword::None,
            "some" => Keyword::Some,
            "match" => Keyword::Match,
            _ => panic!("Unknown keyword: {}", s)
        }
    }
}