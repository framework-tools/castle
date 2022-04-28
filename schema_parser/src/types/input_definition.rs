


use std::collections::HashMap;

use shared_parser::Input;

use super::{Kind, Directive};
/// Argument Definition
///
/// Not to be confused with input arguments which are for the actual provided
/// inputs, whereas argument definitions are for defining the type of inputs.
///
/// Args can be either comma separated or newline separated.
///
/// Eg:
/// ```text
/// type Query {
///     hello(
///         this_is_an_arg: String @lowercase
///         and_another: Number
///     ): String
///     height(unit: Unit = METER): Float
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct InputDefinition {
    pub ident: Box<str>,
    pub input_kind: Kind,
    pub default: Option<Input>,
    pub directives: Vec<Directive>,
}

pub type InputDefinitions = HashMap<Box<str>, InputDefinition>;


#[derive(Debug, PartialEq)]
pub struct InputTypeDefinition {
    pub ident: Box<str>,
    pub input_definitions: InputDefinitions,
    pub directives: Vec<Directive>,
}