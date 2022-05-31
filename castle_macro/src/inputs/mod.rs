use quote::quote_spanned;
use syn::{spanned::Spanned, Fields, ItemStruct};

use crate::Unzip3;

pub fn derive_input(item_struct: ItemStruct) -> proc_macro2::TokenStream {
    let name = &item_struct.ident;

    let fields = match &item_struct.fields {
        Fields::Named(fields) => fields,
        _ => panic!("Only structs with named fields are supporte"),
    };

    let (
        field_conversions,
        input_definitions,
        initializations
    ) = fields.named
        .iter()
        .map(|field| {
            let name = field.ident.as_ref().unwrap();
            let ty = &field.ty;
            (
                quote_spanned!(ty.span()=> #name: inputs.get(stringify!(#name)).unwrap().into()),
                quote_spanned!(ty.span()=> (
                    stringify!(#name).into(), ::castle_api::types::InputDefinition {
                        ident: stringify!(#name).into(),
                        input_kind: <#ty as ::castle_api::types::SchemaItem>::kind(),
                        default: ::core::option::Option::None,
                        directives: vec![],
                    }
                )),
                quote_spanned!(ty.span()=> <#ty as ::castle_api::types::SchemaItem>::initialize_item(schema)),
            )
        })
        .unzip_n::<Vec<_>, Vec<_>, Vec<_>>();

    quote_spanned! {item_struct.span() =>
        #item_struct

        impl ::core::convert::From<&::castle_api::types::Inputs> for #name {
            fn from(inputs: &::castle_api::types::Inputs) -> Self {
                #name {
                    #(
                        #field_conversions,
                    )*
                }
            }
        }

        impl ::castle_api::types::SchemaItem for &#name {
            fn kind() -> ::castle_api::types::Kind {
                ::castle_api::types::Kind {
                    ident: stringify!(#name).into(),
                    generics: vec![]
                }
            }

            fn initialize_item(schema: &mut ::castle_api::types::SchemaDefinition) {
                if !schema.is_type_registered(&stringify!(#name)) {
                    let input_def = ::castle_api::types::InputTypeDefinition {
                        ident: stringify!(#name).into(),
                        input_definitions: [
                            #(
                                #input_definitions,
                            )*
                        ].into(),
                        directives: vec![].into(),
                    };

                    schema.register_input(input_def);

                    #(
                        #initializations;
                    )*
                }
            }
        }
    }
    .into()
}
