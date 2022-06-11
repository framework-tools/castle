use quote::quote_spanned;
use syn::{Fields, spanned::Spanned};

use crate::Unzip2;


pub fn get_from_conversion(item_struct: syn::ItemStruct, types_used: &mut Vec<syn::Type>) -> (syn::Ident, Vec<syn::Expr>, Vec<syn::Item>) {
    let name = &item_struct.ident;

    let fields = match &item_struct.fields {
        Fields::Named(fields) => fields,
        _ => panic!("Only structs with named fields are supporte"),
    };

    let (field_conversions, field_definitions) = fields.named
        .iter()
        .map(|field| {
            let name = field.ident.as_ref().unwrap();
            let ty = &field.ty;
            let directives = "";

            (
                quote_spanned!(ty.span()=> (stringify!(#name).into(), item.#name.into())),
                syn::parse_quote_spanned!(name.span() => (
                    stringify!(#name).into(),
                    ::castle_api::types::FieldDefinition {
                        ident: stringify!(#name).into(),
                        input_definitions: [].into(),
                        return_kind: <#ty as ::castle_api::types::HasKind>::kind(),
                        directives: castle_schema_parser::parse_directives_from_str(#directives),
                    }
                ))
            )
        })
        .unzip_n::<Vec<_>, Vec<_>>();

    (
        name.clone(),
        field_definitions,
        [
            syn::Item::Struct(item_struct),
            syn::parse_quote_spanned!(item_struct.span() => {
                impl From<#name> for ::castle_api::types::Value {
                    fn from(item: #name) -> Self {
                        [
                            #(
                                #field_conversions,
                            )*
                        ].into()
                    }
                }
            }),
        ].into()
    )
}