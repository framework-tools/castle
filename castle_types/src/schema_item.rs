use crate::{SchemaDefinition, Kind};




pub trait SchemaItem {
    fn initialize_item(schema: &mut SchemaDefinition);
    fn kind() -> Kind;
}

impl<T, E> SchemaItem for Result<T, E> where T: SchemaItem {
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

macro_rules! impl_schema_item_for_scalars {
    (
        $($ty:ty: $ident:ident,)*
    ) => {
        $(
            impl SchemaItem for $ty {
                fn initialize_item(schema: &mut SchemaDefinition) {
                    unimplemented!()
                }
                fn kind() -> Kind {
                    Kind {
                        ident: stringify!($ident).into(),
                        generics: vec![],
                    }
                }
            }
        )*
    };
}

impl_schema_item_for_scalars! {
    isize: number,
    i64: number,
    i32: number,
    i16: number,
    i8: number,
    usize: number,
    u64: number,
    u32: number,
    u16: number,
    u8: number,
    f64: number,
    f32: number,
    bool: bool,
}