use super::{Kind, Directive, InputDefinitions};

/// Definition of a field resolver, used in field types
/// ```notrust
/// type Query {
///     this_is_a_field(arg: String): String
///     also_a_field: String @lowercase
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct FieldDefinition {
    pub ident: Box<str>,
    pub input_definitions: InputDefinitions,
    pub return_kind: Kind,
    pub directives: Vec<Directive>,
}