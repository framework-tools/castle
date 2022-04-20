
use std::{collections::HashMap};



use super::{match_statement::{MatchStatement}};


#[derive(Debug, PartialEq)]
pub enum Want {
    SingleField(WantArguments),
    ObjectProjection(Wants, WantArguments),
    Match(MatchStatement),
}

pub type Wants = HashMap<Box<str>, Want>;
pub type WantArguments = HashMap<Box<str>, PrimitiveValue>;

impl Want {
    pub fn new_single_field(arguments: HashMap<Box<str>, PrimitiveValue>) -> Self {
        Want::SingleField(arguments)
    }

    pub fn new_object_projection(fields: Wants, arguments: WantArguments) -> Self {
        Want::ObjectProjection(
            fields,
            arguments,
        )
    }

    pub fn new_match(match_statements: MatchStatement) -> Self {
        Want::Match(
            match_statements
        )
    }
}