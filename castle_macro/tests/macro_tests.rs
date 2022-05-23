use castle_api::{Inputs, Input, Primitive};


#[test]
fn can_derive_struct() {
    #[castle_macro::castle(input)]
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

    #[castle_macro::castle(type)]
    type Root {
        me: User @authenticated,
        login(email: String, password: String): String
    };
}





