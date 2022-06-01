use quote::quote_spanned;
use syn::{
    spanned::Spanned, FnArg, ImplItem, ItemImpl, PatType, ReturnType
};
use crate::{Unzip3, Unzip2};

pub fn derive_type(item_impl: ItemImpl) -> proc_macro2::TokenStream {
    let self_name = &item_impl.self_ty;
    let mut types_used = vec![];

    let (
        matched_fns,
        field_definitions,
    ) = item_impl.items.iter().map(|impl_item| match impl_item {
        ImplItem::Method(method) => {
            let fn_name = &method.sig.ident;
            let fn_return_type = match &method.sig.output {
                ReturnType::Type(_, ty) => *ty.clone(),
                ReturnType::Default => syn::parse_quote_spanned! { fn_name.span() => ()}
            };

            types_used.push(fn_return_type.clone());

            let (input_definitions, input_conversion) = match method.sig.inputs.iter()
                .skip(2)
                .collect::<Vec<_>>()
                .get(0) {
                    Some(FnArg::Typed(PatType { 
                        ty,
                        pat,
                        ..
                    })) => Some((
                        quote_spanned!(ty.span()=>{
                            (stringify!(#pat).into(), ::castle_api::types::InputDefinition {
                                ident: stringify!(#pat).into(),
                                input_kind: <#ty as ::castle_api::types::SchemaItem>::kind(),
                                default: ::core::option::Option::None,
                                directives: vec![],
                            })
                        }),
                        quote_spanned!(ty.span() => , <#ty as ::castle_api::types::FromInputs>::from_inputs(&field.inputs))
                    )),
                    _ => None,
                }
                .into_iter()
                .unzip_n::<Vec<_>, Vec<_>>();

            (
                quote_spanned!(fn_name.span() => stringify!(#fn_name) => ::castle_api::types::ValueToResult::value_to_result(
                    self.#fn_name(ctx #( #input_conversion )*)
                )),
                quote_spanned!(fn_name.span() =>
                    (stringify!(#fn_name).into(), castle_api::types::FieldDefinition {
                        ident: stringify!(#fn_name).into(),
                        input_definitions: [#( #input_definitions, )*].into(),
                        return_kind: <#fn_return_type as ::castle_api::types::SchemaItem>::kind(),
                        directives: [].into(),
                    })),
            )
        }
        _ => panic!("Only methods are supported"),
    }).unzip_n::<Vec<_>, Vec<_>>();

    let initializations = types_used.iter()
        .map(|ty| quote_spanned!(ty.span() => <#ty as ::castle_api::types::SchemaItem>::initialize_item(schema)))
        .collect::<Vec<_>>();
    
    quote_spanned!{ item_impl.span() =>
        #item_impl

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
                        #initializations
                    )*
                }
            }
        }

        impl ::castle_api::types::ResolvesFields for #self_name {
            fn resolve(&self, field: &::castle_api::types::Field, ctx: &::castle_api::types::Context) -> Result<::castle_api::types::Value, ::castle_api::Error> {
                match &*field.ident {
                    #(
                        #matched_fns,
                    )*
                    _ => unreachable!("Should not reachable if property validated")
                }
            }
        }
    }.into()
}
