use quote::quote_spanned;
use syn::{spanned::Spanned, Fields, ItemStruct};

use crate::{shared_functions::get_input_def_and_initalizations};

pub fn derive_input(item_struct: ItemStruct) -> proc_macro2::TokenStream {
    let name = &item_struct.ident;

    let fields = match &item_struct.fields {
        Fields::Named(fields) => fields,
        _ => panic!("Only structs with named fields are supporte"),
    };

    let (
        input_definitions,
        initializations
    ) = get_input_def_and_initalizations(fields.named.clone());

    let field_conversions = fields.named
        .iter()
        .map(|field| {
            let name = field.ident.as_ref().unwrap();
            let ty = &field.ty;
            quote_spanned!(ty.span()=> #name: inputs.get(stringify!(#name)).unwrap().into())
        });

        

    quote_spanned! {item_struct.span() =>
        #item_struct

        impl ::core::convert::From<&::castle_api::types::Input> for #name {
            fn from(input: &::castle_api::types::Input) -> Self {
                let inputs = input.as_map().unwrap();
                #name {
                    #(
                        #field_conversions,
                    )*
                }
            }
        }

        impl ::castle_api::types::HasKind for &#name {
            fn kind() -> ::castle_api::types::Kind {
                ::castle_api::types::Kind {
                    ident: stringify!(#name).into(),
                    generics: vec![]
                }
            }
        }

        impl ::castle_api::types::SchemaItem for &#name {
            fn initialize_item(schema: &mut ::castle_api::types::SchemaDefinition) {
                if !schema.kind_is_registered(&stringify!(#name)) {
                    let input_def = ::castle_api::types::InputTypeDefinition {
                        ident: stringify!(#name).into(),
                        input_definitions: [
                            #(
                                #input_definitions,
                            )*
                        ].into(),
                        directives: vec![].into(),
                    };

                    schema.register_input_type(input_def);

                    #(
                        #initializations;
                    )*
                }
            }
        }
    }
    .into()
}
