
#[derive(Debug, PartialEq)]
pub enum PrimitiveType {
    String,
    Int,
    UInt,
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
            "uint" => Some(PrimitiveType::UInt),
            "float" => Some(PrimitiveType::Float),
            "bool" => Some(PrimitiveType::Bool),
            "uuid" => Some(PrimitiveType::Uuid),
            _ => None
        }
    }
}