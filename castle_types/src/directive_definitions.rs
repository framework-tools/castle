
use std::{collections::{HashMap, HashSet}, fmt::Display};


use crate::Input;

use super::InputDefinition;



/// ### Directive Definition
///
/// Directives are used to modify the behavior of a resolver, types.
/// and provide additional information and metadata.
///
/// Directives can be applied on a field, type, enum, enum value, or input args.
///
/// They can also have arguments, which can also be optional by setting a default.
///
/// ```notrust
/// directive @lowercase on FieldDefinition
/// directive @deprecated(
///     reason: String = "No longer supported"
/// )
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct DirectiveDefinition {
    pub ident: Box<str>,
    pub input_definitions: HashMap<Box<str>, InputDefinition>,
}

/// ### AppliedDirective
/// A directive being applied or used on a field, type, enum, enum value, or input arg.
///
/// Named below for convenience.
/// ```notrust
/// @type_directive(arg: 123)
/// type Root {
///     this_is_a_field(arg: String): String @FieldDefinition(an_arg: 123)
/// }
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct AppliedDirective {
    pub ident: Box<str>,
    pub inputs: HashMap<Box<str>, Input>,
}