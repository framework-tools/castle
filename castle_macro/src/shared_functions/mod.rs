use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{spanned::Spanned, punctuated::Punctuated, Field, token::Comma};

use crate::Unzip2;




pub fn get_input_def_and_initalizations(fields: &Punctuated<Field, Comma>) -> (Vec<TokenStream>, Vec<TokenStream>) {
    fields
        .iter()
        .map(|field| {
            let name = field.ident.as_ref().unwrap();
            let ty = &field.ty;
            (
                quote_spanned!(ty.span()=> (
                    stringify!(#name).into(), ::castle_api::types::InputDefinition {
                        ident: stringify!(#name).into(),
                        input_kind: <#ty as ::castle_api::types::HasKind>::kind(),
                        default: ::core::option::Option::None,
                        directives: vec![],
                    }
                )),
                quote_spanned!(ty.span()=> <#ty as ::castle_api::types::SchemaItem>::initialize_item(schema)),
            )
        })
        .unzip_n::<Vec<_>, Vec<_>>()
}