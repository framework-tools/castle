use std::collections::HashMap;

use super::{field_definition::FieldDefinition, directive_definitions::Directive};

/// Definition of a type in the schema.
///
/// ```notrust
/// @docs(doc: "A user")
/// type User {
///     user_name: String,
///     email: String,
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct TypeDefinition {
    pub ident: Box<str>,
    pub fields: HashMap<Box<str>, FieldDefinition>,
    pub directives: Vec<Directive>,
}