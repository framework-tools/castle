#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Punctuator {


    Default, // = - may be used for default value
    Or, // |
    Neg, // -

    // Generics
    GenericOpen, // <
    GenericClose, // >

    // Symbols
    Colon, // :
    SemiColon, // ;
    Dot, // .
    Comma, // ,
    Spread, // ...
    At, // @ - Used for directives
    DoubleColon, // ::

    // Brackets, Parenthesis, Blocks
    OpenBlock, // {
    CloseBlock, // }
    OpenParen, // (
    CloseParen, // )
    OpenBracket, // [
    CloseBracket, // ]
}

impl Punctuator {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "=" => Some(Punctuator::Default),
            "|" => Some(Punctuator::Or),
            "-" => Some(Punctuator::Neg),

            "<" => Some(Punctuator::GenericOpen),
            ">" => Some(Punctuator::GenericClose),

            ":" => Some(Punctuator::Colon),
            ";" => Some(Punctuator::SemiColon),
            "." => Some(Punctuator::Dot),
            "," => Some(Punctuator::Comma),
            "..." => Some(Punctuator::Spread),
            "@" => Some(Punctuator::At),
            "::" => Some(Punctuator::DoubleColon),

            "{" => Some(Punctuator::OpenBlock),
            "}" => Some(Punctuator::CloseBlock),
            "(" => Some(Punctuator::OpenParen),
            ")" => Some(Punctuator::CloseParen),
            "[" => Some(Punctuator::OpenBracket),
            "]" => Some(Punctuator::CloseBracket),
            _ => None,
        }
    }
}