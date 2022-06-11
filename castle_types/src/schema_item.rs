use crate::{SchemaDefinition};




pub trait SchemaItem {
    fn initialize_item(schema: &mut SchemaDefinition);
}

impl<T, E> SchemaItem for Result<T, E> where T: SchemaItem {
    fn initialize_item(schema: &mut SchemaDefinition) {
        T::initialize_item(schema);
    }
}
impl<G> SchemaItem for Vec<G> where G: SchemaItem {
    fn initialize_item(schema: &mut SchemaDefinition) {
        G::initialize_item(schema);
    }
}
impl<G> SchemaItem for Option<G> where G: SchemaItem {
    fn initialize_item(schema: &mut SchemaDefinition) {
        G::initialize_item(schema);
    }
}


macro_rules! impl_schema_item_for_scalars {
    (
        $($ty:ty: $ident:ident,)*
    ) => {
        $(
            impl SchemaItem for $ty {
                fn initialize_item(schema: &mut SchemaDefinition) {
                    if !schema.kind_is_registered(stringify!($ident)) {
                        schema.register_scalar(stringify!($ident).into());
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
    String: String,
    (): void,
}