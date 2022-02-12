

use super::type_system::{Type, parse_type, get_type_from_string};


#[derive(Debug, PartialEq)]
pub struct VecType {
    pub inner_type: Box<Type>,
}

impl VecType {
    pub fn new(type_: &str) -> Type {
        let inner_type_as_string: Box<str> = type_[4..type_.len() - 1].into();
        let inner_type= get_type_from_string(&inner_type_as_string);

        return Type::VecType( VecType {
            inner_type: inner_type.into()
        })
    }
}