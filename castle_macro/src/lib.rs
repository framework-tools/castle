extern crate proc_macro;

use quote::{quote_spanned};
use syn::{parse_macro_input, ItemStruct, Fields, spanned::Spanned, ItemImpl, ImplItem, ReturnType, Type, FnArg, Pat, ImplItemMethod, Signature, PatType, PatIdent};

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
        "Input" => derive_input(parse_macro_input!(item as ItemStruct)),
        "Type" => derive_type(parse_macro_input!(item as ItemImpl)),
        attribute => panic!("attribute {} is not supported", attribute),
    };
    proc_macro::TokenStream::from(expanded)
}

fn derive_input(struct_: ItemStruct) -> proc_macro2::TokenStream {
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
        impl ::From<&::castle_api::types::Inputs> for #name {
            fn from(inputs: &::castle_api::types::Inputs) -> Self {
                #name {
                    #(#field_names: #conversions),*
                }
            }
        }
    }.into()
}

fn get_field_definitions_from_impl(item_impl: &ItemImpl, types_used: &mut Vec<Type>) -> Vec<proc_macro2::TokenStream> {
    item_impl.items.iter().filter_map(|field| match field {
        ImplItem::Method(ImplItemMethod {
            sig: Signature {
                ident: fn_name,
                inputs,
                output,
                ..
            },
            ..
        }) => {
            let return_kind = match output {
                ReturnType::Type(.., return_type) => *return_type.clone(),
                ReturnType::Default => syn::parse_quote!(())
            };

            types_used.push(return_kind.clone());

            let input_definitions = inputs.iter().filter_map(|input| match input {
                FnArg::Typed(PatType {
                    pat,
                    ty,
                    ..
                }) => {
                    types_used.push(*ty.clone());
                    match &**pat {
                        Pat::Ident(PatIdent {
                            ident,
                            ..
                        }) => Some(quote_spanned!(ty.span() =>
                            (stringify!(#ident).into(), ::castle_api::types::InputDefinition {
                                ident: stringify!(#ident).into(),
                                input_kind: <#ty as ::castle_api::types::schema_item::SchemaItem>::kind(),
                                default: None,
                                directives: vec![],
                            })
                        )),
                        _ => panic!("Only named args are supported, eg: `arg: i32`"),
                    }
                },
                _ => panic!("self args are not supported"),
            });

            Some(quote_spanned!(fn_name.span() =>
                (stringify!(#fn_name).into(), FieldDefinition {
                    ident: stringify!(#fn_name).into(),
                    input_definitions: [#( #input_definitions ),*].into(),
                    return_kind: <#return_kind as ::castle_api::types::schema_item::SchemaItem>::kind(),
                    directives: [].into(),
                })
            ))
        }
        _ => None,
    }).collect()
}

fn derive_type(item_impl: ItemImpl) -> proc_macro2::TokenStream {
    let mut types_used = vec![];
    let self_name = &item_impl.self_ty;

    let field_definitions = get_field_definitions_from_impl(&item_impl, &mut types_used);

    quote_spanned!{ item_impl.self_ty.span() =>
        #item_impl

        impl ::castle_api::types::schema_item::SchemaItem for #self_name {
            fn kind() -> ::castle_schema_parser::types::Kind {
                ::castle_schema_parser::types::Kind {
                    ident: stringify!(#self_name).into(),
                    generics: vec![]
                }
            }
            fn initialize_item(schema: &mut ::castle_schema_parser::types::SchemaDefinition) {
                if !schema.is_type_registered(&stringify!(#self_name)) {
                    let type_def = ::castle_schema_parser::types::TypeDefinition {
                        ident: stringify!(#self_name).into(),
                        fields: [
                            #(
                                #field_definitions,
                            )*
                        ].into(),
                        directives: vec![].into(),
                    };

                    schema.register_type(type_def);

                    #(
                        <#types_used as ::castle_api::types::schema_item::SchemaItem>::initialize_item(schema);
                    )*
                }
            }
        }
    }.into()
}