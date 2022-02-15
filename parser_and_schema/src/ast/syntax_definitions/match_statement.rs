
use super::{expressions::Expression, want::WantType};


pub type MatchStatement = Vec<MatchArm>;

#[derive(Debug, PartialEq)]
pub struct MatchArm {
    pub condition: Expression,
    pub object: WantType
}

impl MatchArm{
    pub fn new(condition: Expression, object: WantType) -> Self {
        MatchArm {
            condition,
            object
        }
    }
}