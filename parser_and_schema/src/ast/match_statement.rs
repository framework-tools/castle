
use super::{want::{Want}, enum_definition::EnumValue};


pub type MatchStatement = Vec<MatchArm>;

#[derive(Debug, PartialEq)]
pub struct MatchArm {
    pub condition: EnumValue,
    pub object_identifier: Box<str>,
    pub object: Want,
}

impl MatchArm{
    pub fn new(condition: EnumValue, object_identifier: Box<str>, object: Want) -> Self {
        MatchArm {
            object_identifier,
            condition,
            object
        }
    }
}