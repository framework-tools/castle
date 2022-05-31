use quote::quote_spanned;
use syn::{
    spanned::Spanned, FnArg, Ident, ImplItem, ImplItemMethod, ItemImpl, Pat, PatIdent, PatType,
    ReturnType, Signature, Type,
};

pub fn derive_type(item_impl: ItemImpl) -> proc_macro2::TokenStream {
    let mut types_used = vec![];
    let mut fn_names = vec![];
    let self_name = &item_impl.self_ty;

    let field_definitions =
        get_field_definitions_from_impl(&item_impl, &mut types_used, &mut fn_names);

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
                        <#types_used as ::castle_api::types::SchemaItem>::initialize_item(schema);
                    )*
                }
            }
        }

        impl ::castle_api::types::ResolvesFields for #self_name {
            fn resolve(&self, field: &::castle_api::types::Field, ctx: &::castle_api::types::Context) -> Result<::castle_api::types::Value, ::castle_api::Error> {
                match &*field.ident {
                    #(
                        stringify!(#fn_names) => self.#fn_names(ctx, <_ as ::castle_api::types::FromInputs>::from_inputs(&field.inputs)).into(),
                    )*
                    _ => unreachable!("Should not reachable if property validated")
                }
            }
        }
    }.into()
}

fn get_field_definitions_from_impl(
    item_impl: &ItemImpl,
    types_used: &mut Vec<Type>,
    fn_names: &mut Vec<Ident>,
) -> Vec<proc_macro2::TokenStream> {
    item_impl
        .items
        .iter()
        .filter_map(|field| match field {
            ImplItem::Method(ImplItemMethod {
                sig:
                    Signature {
                        ident: fn_name,
                        inputs,
                        output,
                        ..
                    },
                ..
            }) => {
                let return_kind = match output {
                    ReturnType::Type(.., return_type) => *return_type.clone(),
                    ReturnType::Default => syn::parse_quote!(()),
                };

                fn_names.push(fn_name.clone());
                types_used.push(return_kind.clone());

                let input_definitions = inputs.iter().skip(2).filter_map(|input| match input {
                    FnArg::Typed(PatType { pat, ty, .. }) => {
                        types_used.push(*ty.clone());

                        match &**pat {
                            Pat::Ident(PatIdent { ident, .. }) => Some(quote_spanned!(ty.span() =>
                                (stringify!(#ident).into(), ::castle_api::types::InputDefinition {
                                    ident: stringify!(#ident).into(),
                                    input_kind: <#ty as ::castle_api::types::SchemaItem>::kind(),
                                    default: None,
                                    directives: vec![],
                                })
                            )),
                            _ => panic!("Only named args are supported, eg: `arg: i32`"),
                        }
                    }
                    FnArg::Receiver(_) => None,
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
        })
        .collect()
}
