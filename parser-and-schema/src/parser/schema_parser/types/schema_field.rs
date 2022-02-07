use crate::ast::syntax_definitions::directive_definition::DirectiveDefinition;



#[derive(Debug, PartialEq)]
pub struct SchemaField {
    pub name: Box<str>,
    pub type_: Type,
    pub directives: Option<DirectiveDefinition>
}

#[derive(Debug, PartialEq)]
pub enum Type {
    PrimitiveType(PrimitiveType),
    SchemaType(Box<str>),
    VecType(Box<str>),
}

#[derive(Debug, PartialEq)]
pub enum PrimitiveType {
    String,
    Int,
    Float,
    Bool,
    Uuid
}

impl PrimitiveType {
    //this function should create a new variable from s so we can 
    //match and ignore whether the characters are upper or lowercase
    pub fn from_str_to_option_primitive_type(s: &str) -> Option<Self> {
        let s = s.to_lowercase();
        let s = s.as_str();
        match s {
            "string" => Some(PrimitiveType::String),
            "int" => Some(PrimitiveType::Int),
            "float" => Some(PrimitiveType::Float),
            "bool" => Some(PrimitiveType::Bool),
            "uuid" => Some(PrimitiveType::Uuid),
            _ => None
        }
    }
    
}