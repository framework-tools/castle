
use std::{collections::HashMap};
use shared::CastleError;

use crate::ast::syntax_definitions::argument::ArgumentOrTuple;

use super::{match_statement::{MatchStatement}, argument::IdentifierAndValueArgument};


#[derive(Debug, PartialEq)]
pub enum Want {
    SingleField(SingleField),
    ObjectProjection(ObjectProjection),
}

#[derive(Debug, PartialEq)]
pub struct SingleField {
    pub identifier: Box<str>,
    pub arguments: HashMap<Box<str>, IdentifierAndValueArgument>,
    pub match_statement: Option<MatchStatement>
}

#[derive(Debug, PartialEq)]
pub struct ObjectProjection {
    pub identifier:  Box<str>,
    pub arguments: HashMap<Box<str>, IdentifierAndValueArgument>,
    pub fields: Option<HashMap<Box<str>, Want>>,
    pub match_statement: Option<MatchStatement>
}

impl Want {
    pub fn new_single_field(identifier: Box<str>, arguments: HashMap<Box<str>, IdentifierAndValueArgument>, match_statement: Option<MatchStatement>) -> Self {
        // let arguments;
        // if args.is_some() {
        //     let args = args.unwrap();
        //     if args.len() == 0 {
        //         arguments = None;
        //     } else {
        //         arguments = Some(args);
        //     }
        // } else {
        //     arguments = args;
        // }
        Want::SingleField(SingleField {
            identifier,
            arguments,
            match_statement
        })
    }

    pub fn new_object_projection(identifier: Box<str>, fields: Option<HashMap<Box<str>, Want>>, match_statement: Option<MatchStatement>, arguments: HashMap<Box<str>, IdentifierAndValueArgument>) -> Self {
        Want::ObjectProjection(ObjectProjection {
            identifier,
            fields,
            arguments,
            match_statement
        })
    }

    pub fn get_identifier(&self) -> Result<Box<str>, CastleError> {
        return match self {
            Want::SingleField(single_field) => Ok(single_field.identifier.clone()),
            Want::ObjectProjection(projection) => Ok(projection.identifier.clone()),
        }
    }
}

impl SingleField {
    pub fn new(identifier: Box<str>, arguments: HashMap<Box<str>, IdentifierAndValueArgument>, match_statement: Option<MatchStatement>) -> Want {
        Want::SingleField(SingleField {
            identifier,
            arguments,
            match_statement
        })
    }
}

impl ObjectProjection {
    pub fn new(identifier: Box<str>, fields: Option<HashMap<Box<str>, Want>>,match_statement: Option<MatchStatement>, arguments: HashMap<Box<str>, IdentifierAndValueArgument>) -> Want {
        Want::ObjectProjection(ObjectProjection {
            identifier,
            fields,
            arguments,
            match_statement
        })
    }
}