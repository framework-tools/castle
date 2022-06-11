use quote::quote_spanned;
use syn::{Fields, spanned::Spanned};


pub fn get_from_conversion(item_struct: syn::ItemStruct) -> proc_macro2::TokenStream {
    let name = &item_struct.ident;

    let fields = match &item_struct.fields {
        Fields::Named(fields) => fields,
        _ => panic!("Only structs with named fields are supporte"),
    };

    let field_conversions = fields.named
        .iter()
        .map(|field| {
            let name = field.ident.as_ref().unwrap();
            let ty = &field.ty;
            quote_spanned!(ty.span()=> (stringify!(#name).into(), item.#name.into()))
        });

    quote_spanned!(item_struct.span() => {
        #item_struct
        
        impl From<#name> for ::castle_api::types::Value {
            fn from(item: #name) -> Self {
                [
                    #(
                        #field_conversions,
                    )*
                ].into()
            }
        }
    })
}