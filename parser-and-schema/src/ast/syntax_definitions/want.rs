
use std::{collections::HashMap};
use shared::CastleError;

use crate::ast::syntax_definitions::argument::Argument;

use super::match_statement::{MatchStatement};


#[derive(Debug, PartialEq)]
pub enum Want {
    SingleField(SingleField),
    Projection(ObjectProjection),
    InnerObject(InnerObject)
}

#[derive(Debug, PartialEq)]
pub struct SingleField {
    pub identifier: Box<str>,
    pub arguments: Option<Vec<Argument>>,
    pub match_statement: Option<MatchStatement>
}

#[derive(Debug, PartialEq)]
pub struct ObjectProjection {
    pub identifier: Box<str>,
    pub fields: Option<HashMap<Box<str>, Want>>,
    pub match_statement: Option<MatchStatement>
}

#[derive(Debug, PartialEq)]
pub struct InnerObject {
    pub fields: Option<HashMap<Box<str>, Want>>,
    pub match_statement: Option<MatchStatement>
}

impl Want {
    pub fn new_single_field(identifier: Box<str>, arguments: Option<Vec<Argument>>, match_statement: Option<MatchStatement>) -> Self {
        Want::SingleField(SingleField {
            identifier,
            arguments,
            match_statement
        })
    }

    pub fn new_object_projection(identifier: Box<str>, fields: Option<HashMap<Box<str>, Want>>, match_statement: Option<MatchStatement>) -> Self {
        Want::Projection(ObjectProjection {
            identifier,
            fields,
            match_statement
        })
    }

    pub fn new_inner_object(fields: Option<HashMap<Box<str>, Want>>, match_statement: Option<MatchStatement>) -> Self {
        Want::InnerObject(InnerObject {
            fields,
            match_statement
        })
    }

    pub fn get_identifier(&self) -> Result<Box<str>, CastleError> {
        return match self {
            Want::SingleField(single_field) => Ok(single_field.identifier.clone()),
            Want::Projection(projection) => Ok(projection.identifier.clone()),
            Want::InnerObject(inner_object) => Err(CastleError::MatchError("InnerObject does not have an identifier".into()))
        }
    }
}

impl SingleField {
    pub fn new(identifier: Box<str>, arguments: Option<Vec<Argument>>, match_statement: Option<MatchStatement>) -> Want {
        Want::SingleField(SingleField {
            identifier,
            arguments,
            match_statement
        })
    }
}

impl ObjectProjection {
    pub fn new(identifier: Box<str>, fields: Option<HashMap<Box<str>, Want>>,match_statement: Option<MatchStatement>) -> Want {
        Want::Projection(ObjectProjection {
            identifier,
            fields,
            match_statement
        })
    }
}