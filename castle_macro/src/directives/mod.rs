use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{Ident, Result, parse::{Parse, ParseStream}, ItemStruct, FieldsNamed, spanned::Spanned};

use crate::{shared_functions::get_input_def_and_initalizations};

pub struct CustomAttribute {
    pub ident: Ident,
    pub tokens: TokenStream,
}

pub struct DirectiveDefAst {
    pub brace_token: syn::token::Brace,
    pub name: Field<syn::LitStr>,
    pub comma_token: syn::Token![,],
    pub args: Field<FieldsNamed>,
}

pub struct Field<T> {
    pub ident: Ident,
    pub equals_token: syn::Token![=],
    pub tokens: T,
}

impl Parse for CustomAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(CustomAttribute {
            ident: input.parse()?,
            tokens: input.parse::<TokenStream>()?
        })
    }
}

impl Parse for DirectiveDefAst {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(DirectiveDefAst {
            brace_token: syn::braced!(content in input),
            name: content.parse()?,
            comma_token: content.parse()?,
            args: content.parse()?
        })
    }
}

impl<T: Parse> Parse for Field<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Field {
            ident: input.parse()?,
            equals_token: input.parse()?,
            tokens: input.parse()?
        })
    }
}

pub fn derive_directive(item_struct: ItemStruct, directives: DirectiveDefAst) -> proc_macro2::TokenStream {
    let directive_name = &item_struct.ident;
    let directive_str_name = directives.name.tokens;
    let directive_args = directives.args.tokens.named;
    let (
        input_definitions,
        initializations
    ) = get_input_def_and_initalizations(directive_args);

    return quote_spanned!{item_struct.span() =>
        #item_struct

        impl ::castle_api::types::SchemaItem for #directive_name {
            fn initialize_item(schema: &mut ::castle_api::types::SchemaDefinition) {
                if !schema.kind_is_registered(&#directive_str_name) {
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
