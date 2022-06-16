use castle_api::{castle::Castle};
use castle_types::{CastleError, State};


async fn create_castle() -> Castle {

    struct Root;
    #[castle_macro::castle(Type)]
    struct SomeThing {
        hello: String,
        sigma: f64,
        thing_is_true: bool
    }

    #[castle_macro::castle(Input)]
    struct Xyz {
        _abc: f64
    }

    #[castle_macro::castle(Type)]
    struct HighLevelObj {
        some_thing: SomeThing,
    }


    #[castle_macro::castle(Type)]
    impl Root {
        fn hello(&self, _ctx: &castle_api::types::State) -> String {
            return "world".to_string()
        }
        fn foo(&self, _ctx: &castle_api::types::State, _bar: f64) -> String {
            unimplemented!()
        }
        fn sigma(&self, _ctx: &castle_api::types::State) -> f64 {
            return 69.0
        }
        fn baz(&self, _ctx: &castle_api::types::State, _arg: Xyz) -> String {
            unimplemented!()
        }
        fn list(&self, _ctx: &castle_api::types::State, _arg: Vec<String>) -> String {
            unimplemented!()
        }
        fn list2(&self, _ctx: &castle_api::types::State, _arg: Vec<Xyz>) -> String {
            unimplemented!()
        }
        fn foobar(&self, _ctx: &castle_api::types::State, _arg1: f64, _arg2: String) -> String {
            unimplemented!()
        }
        fn oogabooga(&self, _ctx: &castle_api::types::State, _is_true: bool) -> String {
            unimplemented!()
        }
        fn some_thing(&self, _ctx: &castle_api::types::State) -> SomeThing {
            SomeThing { hello: self.hello(_ctx), sigma: self.sigma(_ctx), thing_is_true: self.thing_is_true(_ctx) }
        }
        fn thing_is_true(&self, _ctx: &castle_api::types::State) -> bool {
            return true
        }
        fn high_level_obj(&self, _ctx: &castle_api::types::State) -> HighLevelObj {
            unimplemented!()
        }
        fn list_of_some_things(&self, _ctx: &castle_api::types::State) -> Vec<SomeThing> {
            unimplemented!()
        }
        fn list_of_high_level_obj(&self, _ctx: &castle_api::types::State) -> Vec<HighLevelObj> {
            unimplemented!()
        }
    }




    castle_api::castle::CastleBuilder::new(Root)
        .build()
        .unwrap()
}

#[tokio::test]
async fn basic_projection_validates() {
    let ctx = State::new();
    let msg = r#"
    message {
        hello
    }"#;
    let a = create_castle().await
    .run_message(msg, &ctx).await
    .unwrap();
    println!("{:?}", a);
}

#[tokio::test]
async fn basic_projection_validates_breaks_with_mismatch() -> Result<(), CastleError> {
    let msg = r#"
    message {
        world
    }"#;
    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap_err();
    Ok(())
}

#[tokio::test]
async fn projection_with_unknown_args_fails() {
    let msg = "
    message {
        hello(arg: 123)
    }
    ";

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap_err();
}

#[tokio::test]
async fn projection_with_missing_args_fails() {
    let msg = "
    message {
        foo()
    }
    ";

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap_err();
}

#[tokio::test]
async fn nested_input_args_validates() {
    let msg = "
    message {
        baz(arg: {
            abc: 123
        })
    }
    ";

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap();
}

#[tokio::test]
async fn nested_input_with_unknown_args_fails() {
    let msg = "
    message {
        baz(arg: {
            abc: 123,
            def: 123
        })
    }
    ";

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap_err();
}

#[tokio::test]
async fn list_items_without_correct_type_fails() {
    let msg = "
    message {
        list(arg: [\"abc\", 123])
    }
    ";

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap_err();
}

#[tokio::test]
async fn list_item_with_input_type_validates() {
    let msg = "
    message {
        list2(arg: [
            {
                abc: 123
            },
            {
                abc: 123
            }
        ])
    }
    ";

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap();
}

#[tokio::test]
async fn list_item_with_input_type_mismatch_fails() {
    let msg = "
    message {
        list2(arg: [
            {
                abc: 123
            },
            {
                abc: false
            }
        ])
    }
    ";

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap_err();
}

#[tokio::test]
async fn item_with_multiple_args_fails_if_wrong_type() {
    let msg = "
    message {
        foobar(arg1: 123 arg2: 5.5)
    }
    ";

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap_err();
}

#[tokio::test]
async fn item_with_too_many_args_fails() {
    let msg = "
    message {
        foobar(arg1: 123 arg2: 5.5, arg3: 4)
    }
    ";

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap_err();
}

#[tokio::test]
async fn item_with_too_little_args_fails() {
    let msg = "
    message {
        foobar(arg1: 123)
    }
    ";

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap_err();
}

#[tokio::test]
async fn item_with_matching_args_should_pass() {
    let msg = r#"
    message {
        foobar(arg1: 123, arg2: "Hello World")
    }
    "#;

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap();
}

#[tokio::test]
async fn item_with_correct_return_type_should_pass() {
    let msg = r#"
    message {
        foobar(arg1: 123, arg2: "Hello World")
    }
    "#;

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap();
}

#[tokio::test]
async fn fails_if_number_mismatch_on_argument() {
    let msg = r#"
    message {
        foobar(arg1: "Hello World", arg2: "Hello World")
    }
    "#;

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap_err();
}

#[tokio::test]
async fn fails_if_string_mismatch_on_argument() {
    let msg = r#"
    message {
        foobar(arg1: 123, arg2: 55)
    }
    "#;

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap_err();
}

#[tokio::test]
async fn passes_validating_bool_input() {
    let msg = r#"
    message {
        oogabooga(is_true: true)
    }
    "#;

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap();
}

#[tokio::test]
async fn fails_if_bool_mismatch_on_arg() {
    let msg = r#"
    message {
        oogabooga(is_true: "this should fail")
    }
    "#;

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap_err();
}

#[tokio::test]
async fn fails_if_type_mismatch_for_user_defined_type() {
    let msg = r#"
    message {
        baz(arg: true)
    }
    "#;

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap_err();

    let msg = r#"
    message {
        baz(arg: {
            abc: "Hello World"
        })
    }
    "#;

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap_err();
}

#[tokio::test]
async fn validates_vec_that_is_correctly_typed(){
    let msg = r#"
    message {
        list(arg: ["Hello", "World"])
    }
    "#;

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap();
}

#[tokio::test]
async fn can_validate_valid_object_projection(){
    let ctx = State::new();
    let msg = r#"
    message {
        some_thing {
            hello
            sigma
            thing_is_true
        }
    }
    "#;

    let a = create_castle()
        .await.
        run_message(msg, &ctx).await
        .unwrap();
    println!("{:#?}", a);
}

#[tokio::test]
async fn fails_if_field_is_not_defined_on_type(){
    let msg = r#"
    message {
        some_thing {
            hello
            sigma
            doesnt_exist
        }
    }
    "#;

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap_err();
}

#[tokio::test]
async fn empty_projection_succeeds(){
    let msg = r#"
    message {
        high_level_obj {

        }
    }
    "#;

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap();
}

#[tokio::test]
async fn fails_if_invalid_nested_obj(){
    let msg = r#"
    message {
        high_level_obj {
            some_thing
        }
    }
    "#;

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap_err();
}

#[tokio::test]
async fn fails_if_nested_obj_has_undefined_field(){
    let msg = r#"
    message {
        high_level_obj {
            some_thing {
                hello
                sigma
                doesnt_exist
            }
        }
    }
    "#;

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap_err();
}

#[tokio::test]
async fn passes_for_valid_array_projection(){
    let msg = r#"
    message {
        list_of_some_things [
            hello
        ]
    }
    "#;

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap();
}

#[tokio::test]
async fn fails_array_projection_with_invalid_field(){
    let msg = r#"
    message {
        list_of_some_things [
            hello,
            omega
        ]
    }
    "#;

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap_err();
}

#[tokio::test]
async fn passes_valid_mesage_with_multiple_layers_of_nesting() {
    let msg = r#"
    message {
        list_of_high_level_obj [
            some_thing {
                hello
                sigma
                thing_is_true
            }
        ]

        list_of_some_things [
            hello
        ]
    }
    "#;

    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap();
}

#[tokio::test]
async fn fails_for_invalid_field_multiple_layers_of_nesting() {
    let msg = r#"
    message {
        list_of_high_level_obj [
            some_thing {
                hello
                sigma
                doesnt_exist
            }
        ]
        list_of_some_things [
            hello
        ]
    }
    "#;
    create_castle()
        .await.parse_and_validate_message(msg)
        .unwrap_err();
}

