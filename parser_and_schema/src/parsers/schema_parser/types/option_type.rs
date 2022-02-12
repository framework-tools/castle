

use super::type_system::{Type, get_type_from_string};


#[derive(Debug, PartialEq)]
pub struct OptionType {
    pub inner_type: Box<Type>,
}

impl OptionType {
    pub fn new(type_: &str) -> Type {
        let inner_type_as_string: Box<str> = type_[7..type_.len() - 1].into();
        let inner_type= get_type_from_string(&inner_type_as_string);

        return Type::OptionType( OptionType {
            inner_type: inner_type.into()
        })
    }

    pub fn get_option_type_struct(type_: Type) -> OptionType {
        match type_ {
            Type::OptionType(option_type) => option_type,
            _ => panic!("Type is not a OptionType")
        }
    }
}