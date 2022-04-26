
use std::collections::{HashMap, HashSet};



use shared_parser::Input;

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
/// directive @lowercase on FIELD_DEFINITION
/// directive @deprecated(
///     reason: String = "No longer supported"
/// )
/// ```
#[derive(Debug, PartialEq)]
pub struct DirectiveDefinition {
    pub name: Box<str>,
    pub input_definitions: HashMap<Box<str>, InputDefinition>,
    pub locations: HashSet<DirectiveLocation>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DirectiveLocation {
    /// FieldDefinition
    FieldDefinition,

    /// EnumDefinition
    EnumDefinition,

    /// VariantDefinition
    VariantDefinition,

    /// InputDefinition
    InputDefinition,

    /// TypeDefinition
    TypeDefinition,
}

/// ### Directive
/// A directive being used on a field, type, enum, enum value, or input arg.
///
/// Named below for convenience.
/// ```notrust
/// @type_directive(arg: 123)
/// type Query {
///     this_is_a_field(arg: String): String @FieldDefinition(an_arg: 123)
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct Directive {
    pub name: Box<str>,
    pub inputs: HashMap<Box<str>, Input>,
}