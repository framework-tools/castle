


use std::collections::HashMap;

use castle_shared_parser::Input;

use super::{Kind, AppliedDirective};
/// Argument Definition
///
/// Not to be confused with input arguments which are for the actual provided
/// inputs, whereas argument definitions are for defining the type of inputs.
///
/// Args can be either comma separated or newline separated.
///
/// Eg:
/// ```text
/// type Root {
///     hello(
///         this_is_an_arg: String @lowercase
///         and_another: Number
///     ): String
///     height(unit: Unit = METER): Float
/// }
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct InputDefinition {
    pub ident: Box<str>,
    pub input_kind: Kind,
    pub default: Option<Input>,
    pub directives: Vec<AppliedDirective>,
}

pub type InputDefinitions = HashMap<Box<str>, InputDefinition>;


#[derive(Debug, PartialEq, Clone)]
pub struct InputTypeDefinition {
    pub ident: Box<str>,
    pub input_definitions: InputDefinitions,
    pub directives: Vec<AppliedDirective>,
}