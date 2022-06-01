#![feature(proc_macro_diagnostic)]

extern crate proc_macro;
mod inputs;
mod types;

// Allows you to unzip Vec<(A, B, C)> into (Vec<A>, Vec<B>, Vec<C>)
unzip_n::unzip_n!(3);
unzip_n::unzip_n!(2);

#[proc_macro_attribute]
pub fn castle(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match &*attr.to_string() {
        "Input" => inputs::derive_input(syn::parse_macro_input!(item as syn::ItemStruct)).into(),
        "Type" => types::derive_type(syn::parse_macro_input!(item as syn::ItemImpl)).into(),
        attribute => panic!(
            "attribute {} is not supported. Use Input or Type",
            attribute
        ),
    }
}
