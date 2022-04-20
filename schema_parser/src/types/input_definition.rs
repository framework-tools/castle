


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
/// ```notrust
/// type Query {
///     hello(
///         this_is_an_arg: String @lowercase
///         and_another: Number
///     ): String
///     height(unit: Unit = METER): Float
/// }
/// ```
#[derive(Debug)]
pub struct InputDefinition {
    pub name: Box<str>,
    pub input_kind: Kind,
    pub default: Option<Input>,
    pub directives: Vec<Directive>,
}