use std::collections::HashMap;

use super::{Kind, InputDefinition, Directive};


/// Definition of a field resolver, used in field types
/// ```notrust
/// type Query {
///     this_is_a_field(arg: String): String
///     also_a_field: String @lowercase
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct FieldDefinition {
    pub name: Box<str>,
    pub input_definitions: HashMap<Box<str>, InputDefinition>,
    pub return_kind: Kind,
    pub directives: Vec<Directive>,
}