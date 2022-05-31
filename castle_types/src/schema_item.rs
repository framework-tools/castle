use crate::{SchemaDefinition, Kind};




pub trait SchemaItem {
    fn initialize_item(schema: &mut SchemaDefinition);
    fn kind() -> Kind;
}

impl<T> SchemaItem for Result<T, super::CastleError> where T: SchemaItem {
    fn initialize_item(schema: &mut SchemaDefinition) {
        unimplemented!()
    }
    fn kind() -> Kind {
        T::kind()
    }
}

impl SchemaItem for String {
    fn initialize_item(schema: &mut SchemaDefinition) {
        unimplemented!()
    }
    fn kind() -> Kind {
        Kind {
            ident: "String".into(),
            generics: vec![],
        }
    }
}

impl SchemaItem for () {
    fn initialize_item(schema: &mut SchemaDefinition) {
        unimplemented!()
    }
    fn kind() -> Kind {
        Kind {
            ident: "void".into(),
            generics: vec![],
        }
    }
}