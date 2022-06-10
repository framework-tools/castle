use quote::quote_spanned;
use syn::{Ident, Result, parse::{Parse, ParseStream}, ItemStruct, spanned::Spanned};

use crate::{shared_functions::get_input_def_and_initalizations};

#[derive(Debug)]
pub struct DirectiveDefinitionAttribute {
    pub at: syn::Token![@],
    pub ident: Ident,
    pub paren_token: syn::token::Paren,
    pub inputs: syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
}

impl Parse for DirectiveDefinitionAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            at: input.parse()?,
            ident: input.parse()?,
            paren_token: syn::parenthesized!(content in input),
            inputs: content.parse_terminated(syn::Field::parse_named)?,
        })
    }
}

pub fn derive_directive(directive_attribute: DirectiveDefinitionAttribute, item_struct: ItemStruct) -> proc_macro2::TokenStream {
    let directive_name = &item_struct.ident;
    let directive_str_name = directive_attribute.ident;
    let (
        input_definitions,
        initializations
    ) = get_input_def_and_initalizations(&directive_attribute.inputs);

    return quote_spanned!{item_struct.span() =>
        #item_struct

        impl ::castle_api::types::SchemaItem for #directive_name {
            fn initialize_item(schema: &mut ::castle_api::types::SchemaDefinition) {
                if !schema.kind_is_registered(&stringify!(#directive_str_name)) {
                    let type_def = ::castle_api::types::DirectiveDefinition {
                        ident: stringify!(#directive_str_name).into(),
                        input_definitions: [
                            #(
                                #input_definitions,
                            )*
                        ].into(),
                        locations: [].into(),
                    };
                    schema.register_directive_definition(type_def);
                    #(
                        #initializations;
                    )*
                }
            }
        }
    }
}
