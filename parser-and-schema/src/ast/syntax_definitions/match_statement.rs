use std::collections::HashMap;

use super::{want::Want, expressions::Expression};


#[derive(Debug, PartialEq)]
pub struct MatchStatement {
    pub statements: Vec<MatchArm>
}

impl MatchStatement {
    pub fn new(statements: Vec<MatchArm>) -> Self {
        MatchStatement {
            statements
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MatchArm {
    pub condition: Expression,
    pub object: Want
}

impl MatchArm{
    pub fn new(condition: Expression, object: Want) -> Self {
        MatchArm {
            condition,
            object
        }
    }
}