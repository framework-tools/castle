extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, ItemStruct, Fields, spanned::Spanned, ItemImpl, ImplItem, ReturnType, Type, FnArg, Pat};
/// For #[castle_macro::castle(Input)]
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
    let expanded = match &attr.to_string()[..] {
        "Input" => derive_input(item),
        "Type" => derive_type(item),
        attribute => panic!("attribute {} is not supported", attribute),
    };
    TokenStream::from(expanded)
}


fn derive_input(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
        quote_spanned!(ty.span()=> inputs.get(stringify!(#name)).unwrap().into())
    });

    quote_spanned! {name.span()=>
        #struct_
        impl From<&::castle_api::Inputs> for #name {
            fn from(inputs: &::castle_api::Inputs) -> Self {
                #name {
                    #(#field_names: #conversions),*
                }
            }
        }
    }.into()
}

fn derive_type(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    
    let struct_ = parse_macro_input!(item as ItemImpl);
    let mut types_used:Vec<syn::TypePath> = vec![];
    push_return_types_used_to_vec(&struct_, &mut types_used);

    let name = &struct_.self_ty;
    let field_definitions = struct_.items.iter().filter_map(|field| match field {
        ImplItem::Method(method) => {
            let name = &method.sig.ident;
            let return_kind = match &method.sig.output {
                ReturnType::Type(.., return_type) => return_type,
                ReturnType::Default => panic!("Default return types are not supported"),
            };
            let kind = &method.sig.inputs;
            let mut input_definitions = vec![];
            for type_values in kind.iter() {
                match type_values {
                    FnArg::Typed(type_path) => {
                        let kind = match &*type_path.ty {
                            Type::Path(type_path_kind) => {
                                types_used.push(type_path_kind.clone());
                                type_path_kind
                            },
                            _ => { panic!("only type path current supported") }
                        };
                        match &*type_path.pat {
                            Pat::Ident(ident) => {
                                let ident = &ident.ident;
                                let span = type_path.span();
                                input_definitions.push(
                                    quote_spanned!(span=>
                                        stringify!(#ident).into(), 
                                        castle_schema_parser::types::InputDefinition {
                                            ident: stringify!(#ident).into(),
                                            input_kind: castle_schema_parser::types::Kind { ident: stringify!(#kind).into(), generics: vec![] },
                                            default: None,
                                            directives: vec![],
                                        }
                                    )
                                );
                            },
                            _ => {panic!("only ident patterns current supported")},
                        }
                    },
                    _ => panic!("Only type patterns are supported"),
                }
            }
            let field_definition = quote_spanned!(method.sig.ident.span() => 
                (stringify!(#name).into(), FieldDefinition {
                    ident: stringify!(#name).into(),
                    input_definitions: [#( #input_definitions ),*].into(),
                    return_kind: <#return_kind as ::castle_api::types::schema_item::SchemaItem>::name(),
                    directives: [].into(),
                })
            );

            Some(field_definition)
        }
        _ => None,
    });

    let span = struct_.self_ty.span();
    quote_spanned! {span=>
        #struct_
        impl ::castle_api::types::schema_item::SchemaItem for #name {
            fn name() -> ::castle_schema_parser::types::Kind {
                ::castle_schema_parser::types::Kind {
                    ident: stringify!(#name).into(),
                    generics: vec![]
                }
            }
            fn initialize_item(schema: &mut ::castle_schema_parser::types::SchemaDefinition) {
                if(!schema.is_type_registered(&stringify!(#name))) {
                    schema.register_type(::castle_schema_parser::types::TypeDefinition {
                        ident: stringify!(#name).into(),
                        fields: [#( #field_definitions ),*].into(),
                        directives: vec![].into(),
                    });
                    #(
                        <#types_used as ::castle_api::types::schema_item::SchemaItem>::initialize_item(schema);
                    )*
                }
            }
        }
    }.into()
}

fn push_return_types_used_to_vec(struct_: &ItemImpl, types_used: &mut Vec<syn::TypePath>) {
    for type_used_item in struct_.items.iter() {
        match type_used_item {
            ImplItem::Method(method) => match &method.sig.output {
                ReturnType::Type(.., return_type) => match &**return_type {
                    Type::Path(return_type) => types_used.push(return_type.clone()),
                    _ => panic!("Only type paths are supported"),
                },
                ReturnType::Default => {},
            },
            _ => {},
        }
    };
}
