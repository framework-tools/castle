use std::fmt::Display;


/// Definition of a type as a return value;
/// e.g. `Vec<User>`, `User` or `String`
#[derive(Debug, PartialEq)]
pub struct Kind {
    pub ident: Box<str>,
    pub generics: Vec<Kind>,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ident)?;
        if !self.generics.is_empty() {
            write!(f, "<")?;
            for (i, kind) in self.generics.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", kind)?;
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}