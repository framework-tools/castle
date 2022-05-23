extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, DeriveInput, Item, ItemStruct, Fields, spanned::Spanned};
/// Implements `From<Input>` for the given struct. eg:
/// ```
/// #[castle_macro::castle(input)]
/// struct CreateUser {
///     email: String,
///     password: String,
/// }
/// ```
/// will generate:
/// ```ignore
/// impl From<&::castle_api::Inputs> for CreateUser {
///     fn from(inputs: &::castle_api::Inputs) -> Self {
///         CreateUser {
///             email: inputs.get("email").into(),
///             password: inputs.get("password").into()
///         }
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn castle(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let struct_ = parse_macro_input!(item as ItemStruct);
    let name = &struct_.ident;
    let fields = match &struct_.fields {
        Fields::Named(fields) => fields,
        _ => panic!("Only named fields are supported"),
    };

    let field_names = fields.named.iter().map(|field| field.ident.as_ref().unwrap());
    let conversions = fields.named.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        let ty = &field.ty;

        quote_spanned!(ty.span() => inputs.get(stringify!(#name)).unwrap().into())
    });
    match attr.to_string() {
        _ => panic!("Only `castle(input)` is supported"),
    }
    let expanded = quote! {
        #struct_
        
        impl From<&::castle_api::Inputs> for #name {
            fn from(inputs: &::castle_api::Inputs) -> Self {
                #name {
                    #(#field_names: #conversions),*
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}
