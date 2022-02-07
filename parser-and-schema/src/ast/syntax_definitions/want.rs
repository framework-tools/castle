
use std::{collections::HashMap};
use crate::ast::syntax_definitions::argument::Argument;

#[derive(Debug, PartialEq)]
pub enum Want {
    SingleField(SingleField),
    Projection(ObjectProjection)
}

#[derive(Debug, PartialEq)]
pub struct SingleField {
    pub identifier: Box<str>,
    pub arguments: Option<Vec<Argument>>
}

#[derive(Debug, PartialEq)]
pub struct ObjectProjection {
    pub identifier: Box<str>,
    pub fields: Option<HashMap<Box<str>, Want>>,
    pub match_statements: Option<HashMap<Box<str>, Want>>
}

impl Want {
    pub fn new_single_field(identifier: Box<str>, arguments: Option<Vec<Argument>>) -> Self {
        Want::SingleField(SingleField {
            identifier,
            arguments
        })
    }

    pub fn new_object_projection(identifier: Box<str>, fields: Option<HashMap<Box<str>, Want>>, match_statements: Option<HashMap<Box<str>, Want>>) -> Self {
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
    pub fn new(identifier: Box<str>, arguments: Option<Vec<Argument>>) -> Want {
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