
use super::{expressions::Expression, want::{Want, Wants}};


pub type MatchStatement = Vec<MatchArm>;

#[derive(Debug, PartialEq)]
pub struct MatchArm {
    pub condition: Expression,
    pub fields: Wants
}

impl MatchArm{
    pub fn new(condition: Expression, fields: Wants) -> Self {
        MatchArm {
            condition,
            fields
        }
    }
}