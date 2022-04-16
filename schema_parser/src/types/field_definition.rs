use std::collections::HashMap;

use super::{Kind, ArgDefinition, DirectiveDefinition};


/// Definition of a field resolver, used in field types
/// ```notrust
/// type Query {
///     this_is_a_field(arg: String): String
///     also_a_field: String @lowercase
/// }
/// ```
#[derive(Debug)]
pub struct FieldDefinition {
    pub name: Box<str>,
    pub args: HashMap<Box<str>, ArgDefinition>,
    pub return_kind: Kind,
    pub directives: Vec<DirectiveDefinition>,
}