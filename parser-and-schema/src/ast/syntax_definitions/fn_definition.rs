use crate::parser::schema_parser::types::schema_field::Type;





#[derive(Debug, PartialEq)]
pub struct FnDefinition {
    pub name: Box<str>,
    pub args: Vec<Type>,
    pub return_type: Type,
    pub body: Vec<FnStatement>
}


#[derive(Debug, PartialEq)]
pub struct FnStatement; // TODO: statements
