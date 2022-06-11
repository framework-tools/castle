#![feature(proc_macro_diagnostic)]
#![feature(drain_filter)]
use directives::DirectiveDefinitionAttribute;

extern crate proc_macro;
mod inputs;
mod types;
mod directives;
mod shared_functions;
mod type_struct;
// Allows you to unzip Vec<(A, B, C)> into (Vec<A>, Vec<B>, Vec<C>)
unzip_n::unzip_n!(3);
unzip_n::unzip_n!(2);

struct CustomAttribute {
    ident: syn::Ident,
    tokens: proc_macro2::TokenStream,
}

impl syn::parse::Parse for CustomAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(CustomAttribute {
            ident: input.parse()?,
            tokens: input.parse::<proc_macro2::TokenStream>()?
        })
    }
}

#[proc_macro_attribute]
pub fn castle(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = syn::parse_macro_input!(attr as CustomAttribute);
    let attr_tokens = attr.tokens.into();

    match &*attr.ident.to_string() {
        "Input" => inputs::derive_input(syn::parse_macro_input!(item as syn::ItemStruct)).into(),
        "Type" => types::derive_type(syn::parse_macro_input!(item as syn::ItemImpl)).into(),
        "Directive" => directives::derive_directive(
            syn::parse_macro_input!(attr_tokens as DirectiveDefinitionAttribute),
            syn::parse_macro_input!(item as syn::ItemStruct),
        ).into(),
        "TypeStruct" => type_struct::derive_type_struct(syn::parse_macro_input!(item as syn::Item)).into(),
        attribute => panic!(
            "attribute {} is not supported. Use Input, Type or Directive",
            attribute
        ),
    }
}
