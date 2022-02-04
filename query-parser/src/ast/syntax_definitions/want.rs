
use std::collections::HashMap;

use super::expressions::PrimitiveValue;

#[derive(Debug, PartialEq, Clone)]
pub enum Want {
    SingleField(SingleField),
    Projection(ObjectProjection)
}

#[derive(Debug, PartialEq, Clone)]
pub struct SingleField {
    pub identifier: Box<str>,
    pub arguments: Option<Vec<PrimitiveValue>>
}


#[derive(Debug, PartialEq, Clone)]
pub struct ObjectProjection {
    pub identifier: Box<str>,
    pub fields: Option<Vec<Box<Want>>>,
    pub match_statements: Option<Vec<Box<Want>>>
}


impl Want {
    pub fn new_single_field(identifier: Box<str>, arguments: Option<Vec<PrimitiveValue>>) -> Self {
        Want::SingleField(SingleField {
            identifier,
            arguments
        })
    }

    pub fn new_projection(identifier: Box<str>, fields: Option<Vec<Box<Want>>>, match_statements: Option<Vec<Box<Want>>>) -> Self {
        Want::Projection(ObjectProjection {
            identifier,
            fields,
            match_statements
        })
    }

    pub fn get_identifier(&self) -> Box<str> {
        return match self {
            Want::SingleField(single_field) => single_field.identifier.clone(),
            Want::Projection(projection) => projection.identifier.clone()
        }
    }
}

impl SingleField {
    pub fn new(identifier: Box<str>, arguments: Option<Vec<PrimitiveValue>>) -> Want {
        Want::SingleField(SingleField {
            identifier,
            arguments
        })
    }
}

impl ObjectProjection {
    pub fn new(identifier: Box<str>, fields: Option<Vec<Box<Want>>>, match_statements: Option<Vec<Box<Want>>>) -> Want {
        Want::Projection(ObjectProjection {
            identifier,
            fields,
            match_statements
        })
    }
}