
use std::collections::{HashMap, HashSet};

use shared::args::Input;

use super::{arg_definition::ArgDefinition};

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
#[derive(Debug)]
pub struct DirectiveDefinition {
    pub name: Box<str>,
    pub arguments: HashMap<Box<str>, ArgDefinition>,
    pub locations: HashSet<DirectiveLocation>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DirectiveLocation {
    /// FIELD_DIRECTIVE
    FieldDirective,

    /// ENUM_DIRECTIVE
    EnumDirective,

    /// VARIANT_DIRECTIVE
    VariantDirective,

    /// INPUT_DIRECTIVE
    InputDirective,

    /// TYPE_DIRECTIVE
    TypeDirective,
}

/// ### Directive
/// A directive being used on a field, type, enum, enum value, or input arg.
///
/// Named below for convenience.
/// ```notrust
/// @type_directive
/// type Query {
///     this_is_a_field(arg: String): String @field_directive
/// }
/// ```
#[derive(Debug)]
pub struct Directive {
    pub name: Box<str>,
    pub inputs: HashMap<Box<str>, Input>,
}