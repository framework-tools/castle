#[derive(Debug, PartialEq)]
pub struct SchemaField {
    pub name: Box<str>,
    pub schema_type: Type,
}
#[derive(Debug, PartialEq)]
pub enum Type {
    PrimitiveType(PrimitiveType),
}
#[derive(Debug, PartialEq)]
pub enum PrimitiveType {
    String,
    Int,
    Float,
    Bool,
    Date,
    DateTime,
    Time,
    Timestamp,
    Uuid,
    Json,
    Jsonb,
    Array,
    Enum,
    Set,
    Range,
    UserDefined,
}

impl PrimitiveType {
    pub fn from_str_to_option_primitive_type(s: &str) -> Option<Self> {
        match s {
            "string" => Some(PrimitiveType::String),
            "int" => Some(PrimitiveType::Int),
            "float" => Some(PrimitiveType::Float),
            "bool" => Some(PrimitiveType::Bool),
            "date" => Some(PrimitiveType::Date),
            "datetime" => Some(PrimitiveType::DateTime),
            "time" => Some(PrimitiveType::Time),
            "timestamp" => Some(PrimitiveType::Timestamp),
            "uuid" => Some(PrimitiveType::Uuid),
            "json" => Some(PrimitiveType::Json),
            "jsonb" => Some(PrimitiveType::Jsonb),
            "array" => Some(PrimitiveType::Array),
            "enum" => Some(PrimitiveType::Enum),
            "set" => Some(PrimitiveType::Set),
            "range" => Some(PrimitiveType::Range),
            _ => None
        }
    }
    
}