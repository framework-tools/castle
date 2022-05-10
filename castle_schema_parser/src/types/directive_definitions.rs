
use std::{collections::{HashMap, HashSet}, fmt::Display};

use castle_shared_parser::Input;

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
    pub locations: HashSet<DirectiveLocation>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

    /// InputFieldDefinition
    InputFieldDefinition,
}

impl Display for DirectiveLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectiveLocation::FieldDefinition => write!(f, "FieldDefinition"),
            DirectiveLocation::EnumDefinition => write!(f, "EnumDefinition"),
            DirectiveLocation::VariantDefinition => write!(f, "VariantDefinition"),
            DirectiveLocation::InputDefinition => write!(f, "InputDefinition"),
            DirectiveLocation::TypeDefinition => write!(f, "TypeDefinition"),
            DirectiveLocation::InputFieldDefinition => write!(f, "InputFieldDefinition"),
        }
    }
}

/// ### Directive
/// A directive being used on a field, type, enum, enum value, or input arg.
///
/// Named below for convenience.
/// ```notrust
/// @type_directive(arg: 123)
/// type Root {
///     this_is_a_field(arg: String): String @FieldDefinition(an_arg: 123)
/// }
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Directive {
    pub ident: Box<str>,
    pub inputs: HashMap<Box<str>, Input>,
}