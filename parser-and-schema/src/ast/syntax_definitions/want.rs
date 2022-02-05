
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
    pub fields: Option<HashMap<Box<str>, Want>>,
    pub match_statements: Option<HashMap<Box<str>, Want>>
}

impl Want {
    pub fn new_single_field(identifier: Box<str>, arguments: Option<Vec<PrimitiveValue>>) -> Self {
        Want::SingleField(SingleField {
            identifier,
            arguments
        })
    }

    pub fn new_projection(identifier: Box<str>, fields: Option<HashMap<Box<str>, Want>>, match_statements: Option<HashMap<Box<str>, Want>>) -> Self {
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
    pub fn new(identifier: Box<str>, fields: Option<HashMap<Box<str>, Want>>, match_statements: Option<HashMap<Box<str>, Want>>) -> Want {
        Want::Projection(ObjectProjection {
            identifier,
            fields,
            match_statements
        })
    }
}