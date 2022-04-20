
/// Definition of a type as a return value;
/// e.g. `Vec<User>`, `User` or `String`
#[derive(Debug, PartialEq)]
pub struct Kind {
    pub name: Box<str>,
    pub generics: Vec<Kind>,
}