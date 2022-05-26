use castle_schema_parser::types::{SchemaDefinition, Kind};


pub trait SchemaItem {
    fn initialize_item(schema: &mut SchemaDefinition);
    fn name() -> Kind;
}

impl<T> SchemaItem for Result<T, crate::Error> where T: SchemaItem {
    fn initialize_item(schema: &mut SchemaDefinition) {
        unimplemented!()
    }
    fn name() -> Kind {
        T::name()
    }
}

impl SchemaItem for String {
    fn initialize_item(schema: &mut SchemaDefinition) {
        unimplemented!()
    }
    fn name() -> Kind {
        Kind {
            ident: "String".into(),
            generics: vec![],
        }
    }
}