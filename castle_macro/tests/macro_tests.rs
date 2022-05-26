use castle_api::{Inputs, Input, Primitive, types::schema_item::SchemaItem};
use castle_schema_parser::types::{SchemaDefinition, FieldDefinition, TypeDefinition};


#[test]
fn can_derive_struct() {
    #[castle_macro::castle(Input)]
    struct Foo {
        bar: i32,
        bar2: usize,
        bar3: String
    }

    let inputs: Inputs = [
        ("bar".into(), Input::Primitive(Primitive::Number(1u32.into()))),
        ("bar2".into(), Input::Primitive(Primitive::Number(2u32.into()))),
        ("bar3".into(), Input::Primitive(Primitive::String("hello".into()))),
    ].into();


    let converted: Foo = Foo::from(&inputs);

    assert_eq!(converted.bar, 1);
    assert_eq!(converted.bar2, 2);
    assert_eq!(converted.bar3, "hello".to_string());
}



#[test]
fn can_impl_resolve() {
    struct Root {
        foo: String
    }

    #[castle_macro::castle(Type)]
    impl Root {
        async fn foo(a: String) -> Result<(), castle_api::Error> {
            unimplemented!()
        }
    }
}

// #[test]
// fn can_impl_resolve_complex() {
//     struct Root {
//         foo: String
//     }

//     #[castle_macro::castle(Type)]
//     impl Root {
//         async fn me(ctx: Ctx) -> Result<User, castle_api::Error> {
//             User {
//                 id: "foo",
//             }
//         }

//         async fn login(input: LoginDetails, ctx: Ctx) -> Result<String, Error> {
//             unimplemented!()
//         }

//         pub async fn signup(input: CreateUser, ctx: Ctx) -> Result<(), Error> {
//             User::create_user(input).await
//         }
//     }
// }



