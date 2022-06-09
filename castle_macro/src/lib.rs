#![feature(proc_macro_diagnostic)]
use crate::directives::DirectiveDefAst;
extern crate proc_macro;
mod inputs;
mod types;
mod directives;
mod shared_functions;
// Allows you to unzip Vec<(A, B, C)> into (Vec<A>, Vec<B>, Vec<C>)
unzip_n::unzip_n!(3);
unzip_n::unzip_n!(2);




#[proc_macro_attribute]
pub fn castle(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = syn::parse_macro_input!(attr as directives::CustomAttribute);
    let attr_tokens = attr.tokens.into();
    match &*attr.ident.to_string() {
        "Input" => inputs::derive_input(syn::parse_macro_input!(item as syn::ItemStruct)).into(),
        "Type" => types::derive_type(syn::parse_macro_input!(item as syn::ItemImpl)).into(),
        "Directive" => directives::derive_directive(
            syn::parse_macro_input!(item as syn::ItemStruct),
            syn::parse_macro_input!(attr_tokens as DirectiveDefAst),
        ).into(),
        attribute => panic!(
            "attribute {} is not supported. Use Input, Type or Directive",
            attribute
        ),
    }
}
