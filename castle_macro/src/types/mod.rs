
mod get_from_conversion;
use std::collections::HashSet;

use crate::types::get_from_conversion::get_from_conversion;
use quote::quote_spanned;
use syn::{
    spanned::Spanned, FnArg, ImplItem, PatType, ReturnType, Item
};
use crate::{Unzip2, directives::AppliedDirective};

pub fn derive_type(item_impl: Item) -> proc_macro2::TokenStream {
    let mut types_used = HashSet::new();
    let (self_name, field_definitions, items): (syn::Type, Vec<syn::Expr>, Vec<syn::Item>) = match item_impl {
        Item::Impl(item_impl) => get_item_impl_conversion(item_impl, &mut types_used),
        Item::Struct(item_struct) => get_from_conversion(item_struct, &mut types_used),
        _ => panic!("Only impls and structs are supported"),
    };

    let initializations = types_used.iter()
        .map(|ty| quote_spanned!(ty.span() => <#ty as ::castle_api::types::SchemaItem>::initialize_item(schema)))
        .collect::<Vec<_>>();

    quote_spanned!{ self_name.span() =>
        impl ::castle_api::types::HasKind for #self_name{
            fn kind() -> ::castle_api::types::Kind {
                ::castle_api::types::Kind {
                    ident: stringify!(#self_name).into(),
                    generics: vec![]
                }
            }
        }

        impl ::castle_api::types::SchemaItem for #self_name {
            fn initialize_item(schema: &mut ::castle_api::types::SchemaDefinition) {
                if !schema.kind_is_registered(&stringify!(#self_name)) {
                    let type_def = ::castle_api::types::TypeDefinition {
                        ident: stringify!(#self_name).into(),
                        fields: [
                            #(
                                #field_definitions,
                            )*
                        ].into(),
                        directives: [].into(),
                    };
                    schema.register_type(type_def);
                    #(
                        #initializations;
                    )*
                }
            }
        }

        #(
            #items
        )*
    }.into()

}

fn get_item_impl_conversion(mut item_impl: syn::ItemImpl, types_used: &mut HashSet<syn::Type>) -> (syn::Type, Vec<syn::Expr>, Vec<syn::Item>) {
    let self_name = item_impl.self_ty.clone();
    let (
        matched_fns,
        field_definitions,
    ) = item_impl.items.iter_mut().map(|impl_item| match impl_item {
        ImplItem::Method(method) => {
            let fn_name = &method.sig.ident;
            let fn_return_type = match &method.sig.output {
                ReturnType::Type(_, ty) => *ty.clone(),
                ReturnType::Default => syn::parse_quote_spanned! { fn_name.span() => () }
            };
            let await_suffix = match method.sig.asyncness {
                Some(_) => quote_spanned! { fn_name.span() => .await },
                None => quote_spanned!(fn_name.span() => ),
            };

            let directives: String = method.attrs.drain_filter(|attr| attr.path.is_ident("directives"))
                .map(|attr| {
                    let tokens = attr.tokens.into();
                    syn::parse::<AppliedDirective>(tokens).unwrap().string.value()
                }).collect::<Vec<String>>().join(" ");

            types_used.insert(fn_return_type.clone());

            let (input_definitions, input_conversion) = method.sig.inputs
                .iter_mut()
                .skip(2)
                .filter_map(|arg| match arg {
                    FnArg::Typed(PatType {
                        ty,
                        pat,
                        ..
                    }) => {
                        types_used.insert(*ty.clone());
                        Some((
                            quote_spanned!(ty.span()=>{
                                (stringify!(#pat).into(), ::castle_api::types::InputDefinition {
                                    ident: stringify!(#pat).into(),
                                    input_kind: <#ty as ::castle_api::types::HasKind>::kind(),
                                    default: ::core::option::Option::None,
                                    directives: vec![],
                                })
                            }),
                            quote_spanned!(ty.span() => field.inputs.get(stringify!(#pat)).unwrap().into()),
                        ))
                    }
                    _ => panic!("unexpected argument type"),
                })
                .into_iter()
                .unzip_n::<Vec<_>, Vec<_>>();

            (
                quote_spanned!(fn_name.span() => stringify!(#fn_name) => ::castle_api::types::ConvertFrom::from(self.#fn_name(ctx #( , #input_conversion )*)#await_suffix)),
                syn::parse_quote_spanned!(fn_name.span() =>
                    (stringify!(#fn_name).into(), ::castle_api::types::FieldDefinition {
                        ident: stringify!(#fn_name).into(),
                        input_definitions: [#( #input_definitions, )*].into(),
                        return_kind: <#fn_return_type as ::castle_api::types::HasKind>::kind(),
                        directives: castle_api::parse_directives_from_str(#directives),
                    })),
            )
        }
        _ => panic!("Only methods are supported"),
    }).unzip_n::<Vec<_>, Vec<_>>();


    (
        *self_name.clone(),
        field_definitions,
        [
            syn::Item::Impl(item_impl),
            syn::parse_quote_spanned!{ self_name.span() =>
                #[castle_api::async_trait]
                impl ::castle_api::types::ResolvesFields for #self_name {
                    async fn resolve(&self, field: &::castle_api::types::Field, ctx: &::castle_api::types::State) -> ::core::result::Result<::castle_api::types::Value, ::castle_api::Error> {
                        match &*field.ident {
                            #(
                                #matched_fns,
                            )*
                            _ => unreachable!("Should not reachable if property validated")
                        }
                    }
                }
            }
        ].into()
    )
}