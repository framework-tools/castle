use super::{Kind, AppliedDirective, InputDefinitions};

/// Definition of a field resolver, used in field types
/// ```notrust
/// type Root {
///     this_is_a_field(arg: String): String
///     also_a_field: String @lowercase
/// }
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct FieldDefinition {
    pub ident: Box<str>,
    pub input_definitions: InputDefinitions,
    pub return_kind: Kind,
    pub directives: Vec<AppliedDirective>,
}