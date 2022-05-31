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
        "CastleObject" => derive_castle_object(parse_macro_input!(item as ItemImpl)),
        attribute => panic!("attribute {} is not supported", attribute),
    };
    proc_macro::TokenStream::from(expanded)
}

fn derive_input(struct_: ItemStruct) -> proc_macro2::TokenStream {
    let name = &struct_.ident;

    let mut types_used = vec![];

    let fields = match &struct_.fields {
        Fields::Named(fields) => fields,
        _ => panic!("Only named fields are supported"),
    };

    let field_conversions = fields.named.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        quote_spanned!(ty.span()=> #name: inputs.get(stringify!(#name)).unwrap().into())
    });

    let input_definitions = fields.named.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        types_used.push(ty.clone());
        quote_spanned!(ty.span()=> (
            stringify!(#name).into(), ::castle_api::types::InputDefinition {
                ident: stringify!(#name).into(),
                input_kind: <#ty as ::castle_api::types::SchemaItem>::kind(),
                default: ::core::option::Option::None,
                directives: vec![],
            }
        ))
    });
    
    quote_spanned! {name.span()=>
        #struct_

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
                        <#types_used as ::castle_api::types::SchemaItem>::initialize_item(schema);
                    )*
                }
            }
        }
    }.into()
}

fn derive_type(item_impl: ItemImpl) -> proc_macro2::TokenStream {
    let mut types_used = vec![];
    let self_name = &item_impl.self_ty;

    let field_definitions = get_field_definitions_from_impl(&item_impl, &mut types_used);

    quote_spanned!{ item_impl.self_ty.span() =>
        #item_impl

        impl ::castle_api::types::ResolvesFields for #self_name {
            fn resolve(&self, field: &::castle_api::types::Field, ctx: &::castle_api::types::Context) -> Result<::castle_api::types::Value, ::castle_api::Error> {
                match &*field.ident {
                    
                    _ => unreachable!("Should not reachable if property validated")
                }
            }
        }

        impl ::castle_api::types::SchemaItem for #self_name {
            fn kind() -> ::castle_api::types::Kind {
                ::castle_api::types::Kind {
                    ident: stringify!(#self_name).into(),
                    generics: vec![]
                }
            }
            fn initialize_item(schema: &mut ::castle_api::types::SchemaDefinition) {
                if !schema.is_type_registered(&stringify!(#self_name)) {
                    let type_def = ::castle_api::types::TypeDefinition {
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
                        <#types_used as ::castle_api::types::SchemaItem>::initialize_item(schema);
                    )*
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
                                input_kind: <#ty as ::castle_api::types::SchemaItem>::kind(),
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
                    input_definitions: [#( #input_definitions, )*].into(),
                    return_kind: <#return_kind as ::castle_api::types::SchemaItem>::kind(),
                    directives: [].into(),
                })
            ))
        }
        _ => None,
    }).collect()
}



/// For #[castle_macro::castle(CastleObject)]
/// implement resolve for the type
/// ```
/// #[castle_macro::castle(input)]
///impl User {
///    async fn first_name(&self, ctx: Context) -> Result<String, Error> {
///      
///    }
///}
/// ```
/// will generate:
/// ```ignore
///impl Resolve for User {
///    fn resolve(&self, field: &Field, ctx: &Context, errors: &mut Vec<Error>) {
///      let map = field.as_map();
///      let result_map = HashMap::new();
///      
///      for field in map {
///        match field.ident {
///          "first_name" => result_map.insert("first_name", Resolver::resolve(self.first_name(ctx), ctx, errors))
///        }
///      }
///    }
///  }
/// ```
fn derive_castle_object(item_impl: ItemImpl) -> proc_macro2::TokenStream {
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