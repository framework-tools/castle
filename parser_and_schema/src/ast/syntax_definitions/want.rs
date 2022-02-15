
use std::{collections::HashMap};
use shared::CastleError;


use super::{match_statement::{MatchStatement}, argument::IdentifierAndValueArgument};


#[derive(Debug, PartialEq)]
pub enum Want {
    SingleField(WantArguments),
    ObjectProjection(Wants, WantArguments),
    Match(MatchStatement),
}

pub type Wants = HashMap<Box<str>, Want>;
pub type WantArguments = HashMap<Box<str>, IdentifierAndValueArgument>;

impl Want {
    pub fn new_single_field(arguments: HashMap<Box<str>, IdentifierAndValueArgument>) -> Self {
        Want::SingleField(arguments)
    }

    pub fn new_object_projection(identifier: Box<str>, fields: Wants, arguments: WantArguments) -> Self {
        Want::ObjectProjection(
            fields,
            arguments,
        )
    }
}