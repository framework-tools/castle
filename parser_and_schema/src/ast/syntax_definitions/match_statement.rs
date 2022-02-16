
use super::{expressions::Expression, want::{Want, Wants}, enum_definition::EnumValue};


pub type MatchStatement = Vec<MatchArm>;

#[derive(Debug, PartialEq)]
pub struct MatchArm {
    pub condition: EnumValue,
    pub fields: Wants
}

impl MatchArm{
    pub fn new(condition: EnumValue, fields: Wants) -> Self {
        MatchArm {
            condition,
            fields
        }
    }
}