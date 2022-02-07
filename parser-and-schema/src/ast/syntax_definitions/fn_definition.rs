use linked_hash_map::LinkedHashMap;

use super::Type;



#[derive(Debug, PartialEq)]
pub struct FnDefinition {
    pub name: String,
    pub args: LinkedHashMap<String, Type>,
    pub return_type: Type,
    pub body: Vec<FnStatement>
}


#[derive(Debug, PartialEq)]
pub struct FnStatement; // TODO: statements
