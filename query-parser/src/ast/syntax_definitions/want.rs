
use std::collections::{HashMap, HashSet};

use super::expressions::PrimitiveValue;

#[derive(Debug, PartialEq, Hash, Clone, Eq)]
pub enum Want {
    SingleField(SingleField),
    Projection(ObjectProjection)
}

#[derive(Debug, PartialEq, Hash, Clone, Eq)]
pub struct SingleField {
    pub identifier: Box<str>,
    pub arguments: Option<Vec<PrimitiveValue>>
}


#[derive(Debug, PartialEq, Hash, Clone, Eq)]
pub struct ObjectProjection {
    pub identifier: Box<str>,
    pub fields: Vec<Box<Want>>
}

impl Want {
    pub fn new_single_field(identifier: Box<str>, arguments: Option<Vec<PrimitiveValue>>) -> Self {
        Want::SingleField(SingleField {
            identifier,
            arguments
        })
    }

    pub fn new_projection(identifier: Box<str>, fields: Vec<Box<Want>>) -> Self {
        Want::Projection(ObjectProjection {
            identifier,
            fields
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
    pub fn new(identifier: Box<str>, fields: Vec<Box<Want>>) -> Want {
        Want::Projection(ObjectProjection {
            identifier,
            fields
        })
    }
}