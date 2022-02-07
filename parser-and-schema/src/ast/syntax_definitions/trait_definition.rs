use std::collections::HashMap;
use std::collections::HashSet;

use linked_hash_map::LinkedHashMap;

use super::field_definition::FieldDefinition;
use super::directive_definition::DirectiveDefinition;
use super::Mark;
use super::Type;

#[derive(Debug, PartialEq)]
pub struct TraitDefinition {
    pub name: Type,
    pub marks: HashSet<Mark>,
    pub fields: HashMap<String, FieldDefinition>,
    pub directives: LinkedHashMap<String, DirectiveDefinition>,
}

