use castle_api::types::{Input, Primitive, Context};


#[test]
fn can_derive_struct() {
    #[castle_macro::castle(Input)]
    struct Foo {
        bar: i32,
        bar2: usize,
        bar3: String
    }

    let converted: Foo = Foo::from(&Input::Map([
        ("bar".into(), Input::Primitive(Primitive::Number(1u32.into()))),
        ("bar2".into(), Input::Primitive(Primitive::Number(2u32.into()))),
        ("bar3".into(), Input::Primitive(Primitive::String("hello".into()))),
    ].into()));

    assert_eq!(converted.bar, 1);
    assert_eq!(converted.bar2, 2);
    assert_eq!(converted.bar3, "hello".to_string());
}



// #[test]
// fn can_impl_resolve() {
//     struct Root {
//         foo: String
//     }

//     #[castle_macro::castle(Type)]
//     impl Root {
//         async fn first_name(a: String) -> Result<(), anyhow::Error> {
//             unimplemented!()
//         }
//     }
// }

#[test]
fn testing_user_match() {
    struct User;
    struct Profile;

    #[castle_macro::castle(Type)]
    impl Profile {
        fn name(&self, _ctx: &Context) -> String {
            "hello".to_string()
        }
    }

    #[castle_macro::castle(Type)]
    impl User {

        fn first_name(&self, _ctx: &Context) -> Result<String, anyhow::Error> {
            unimplemented!()
        }
        
        fn profile(&self, _ctx: &Context, _arg: String) -> Profile {
            unimplemented!()
        }
    }
}



#[test]
fn testing_directives() {

    #[castle_macro::castle(Directive {
        name = "authenticated",
        args = {
            limit: u32,
        }
    })]
    struct Authenticated;

    struct Root;
    // directives need a identifier or name
    // directives can take args and must match the specification within the
    // SchemaItem trait
    // full eg directive #[directive(type: value)]
    #[castle_macro::castle(Type)]
    impl Root {
        #[directives("@Authenticated(a: b)@sorted(a: b)")]
        fn me(&self, _ctx: &Context) -> String {
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



