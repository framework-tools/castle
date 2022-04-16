use std::collections::HashMap;

use super::{field_definition::FieldDefinition, directives::Directive};

/// Definition of a type in the schema.
///
/// ```notrust
/// @docs(doc: "A user")
/// type User {
///     user_name: String,
///     email: String,
/// }
/// ```
#[derive(Debug)]
pub struct TypeDefinition {
    pub directives: Vec<Directive>,
    pub identifier: Box<str>,
    pub fields: HashMap<Box<str>, FieldDefinition>,
}