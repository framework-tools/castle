
use std::{collections::HashMap};
use shared::CastleError;


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
}

#[derive(Debug, PartialEq)]
pub struct ObjectProjection {
    pub identifier:  Box<str>,
    pub arguments: HashMap<Box<str>, IdentifierAndValueArgument>,
    pub fields: FieldsType
}

#[derive(Debug, PartialEq)]
pub enum FieldsType {
    Regular(HashMap<Box<str>, Want>),
    Match(MatchStatement)
}

impl Want {
    pub fn new_single_field(identifier: Box<str>, arguments: HashMap<Box<str>, IdentifierAndValueArgument>) -> Self {
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
        })
    }

    pub fn new_object_projection(identifier: Box<str>, fields: FieldsType, arguments: HashMap<Box<str>, IdentifierAndValueArgument>) -> Self {
        Want::ObjectProjection(ObjectProjection {
            identifier,
            fields,
            arguments,
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
    pub fn new(identifier: Box<str>, arguments: HashMap<Box<str>, IdentifierAndValueArgument>) -> Want {
        Want::SingleField(SingleField {
            identifier,
            arguments,
        })
    }
}

impl ObjectProjection {
    pub fn new(identifier: Box<str>, fields: FieldsType, arguments: HashMap<Box<str>, IdentifierAndValueArgument>) -> Want {
        Want::ObjectProjection(ObjectProjection {
            identifier,
            fields,
            arguments,
        })
    }
}