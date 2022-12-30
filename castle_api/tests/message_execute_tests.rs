use castle_api::Castle;
use castle_types::{State, Directive, Field, Inputs, Next, Value};



// #[tokio::test]
// fn testing_message_execution() {

// }
//tests are in query validation, can move over and seperate test
async fn create_castle() -> Castle {

    struct Root;

    #[derive(Clone)]
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

    #[castle_macro::castle(Directive @test_directive(string: String))]
    struct TestDirective;

    #[async_trait::async_trait]
    impl Directive for TestDirective {
        async fn field_visitor(
            &self,
            field: Field,
            directive_args: &Inputs,
            next: Next,
            context: &State,
        ) -> Result<Value, anyhow::Error> {
            let string = directive_args.get("string").unwrap().to_string();
            assert_eq!(string, "hello world");
            next.resolve(field).await
        }
    }

    struct HighLevelObj {
        some_thing: SomeThing,
    }

    #[castle_macro::castle(Type)]
    impl HighLevelObj {
        fn some_thing(&self, _ctx: &castle_api::types::State) -> SomeThing {
            self.some_thing.clone()
        }
    }


    #[castle_macro::castle(Type)]
    impl Root {

        #[directives(@test_directive(string: "hello world"))]
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
        fn list(&self, _ctx: &castle_api::types::State, arg: Vec<String>) -> String {
            return arg.concat()
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
            return [self.some_thing(_ctx), self.some_thing(_ctx), self.some_thing(_ctx)].into()
        }
        fn list_of_high_level_obj(&self, _ctx: &castle_api::types::State) -> Vec<HighLevelObj> {
            return vec![
                HighLevelObj {
                    some_thing: SomeThing {
                        hello: String::from("hello"),
                        sigma: 1.0,
                        thing_is_true: true
                    }
                },
                HighLevelObj {
                    some_thing: SomeThing {
                        hello: String::from("hello"),
                        sigma: 1.0,
                        thing_is_true: true
                    }
                }
            ]
        }
        fn variant(&self, _ctx: &castle_api::types::State, foo: Option<i32>) -> i32 {
            foo.unwrap()
        }
        fn option_return(&self, _ctx: &castle_api::types::State) -> Option<i32> {
            Some(1)
        }
    }




    castle_api::castle::CastleBuilder::new(Root)
        .add_directive("test_directive", TestDirective)
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_hello_world() {
    let ctx = State::new();
    let msg = "
    message {
        hello
    }
    ";

    let a = create_castle().await
    .run_message(msg, &ctx).await
    .unwrap();

    assert_eq!(
        a.0,
        Value::Object([
            ("hello".into(), Value::String("world".to_string()))
        ].into())
    )
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
async fn list_items() {
    let ctx = State::new();
    let msg = "
    message {
        list(arg: [\"hello\", \"world\"])
    }
    ";

    let a = create_castle().await
    .run_message(msg, &ctx).await
    .unwrap();
    println!("{:#?}", a);
}

#[tokio::test]
async fn list_of_some_things_test() {
    let ctx = State::new();
    let msg = "
    message {
        list_of_some_things() [

        ]
    }
    ";

    let a = create_castle().await
    .run_message(msg, &ctx).await
    .unwrap();
    println!("{:#?}", a);
}

#[tokio::test]
async fn message_with_enum_variant() {
    let ctx = State::new();
    let msg = "
    message {
        variant(foo: Some(123))
    }
    ";

    let a = create_castle().await
        .run_message(msg, &ctx).await
        .unwrap();
    println!("{:#?}", a);
}

#[tokio::test]
async fn message_with_list_of_high_level_obj() {
    let ctx = State::new();
    let msg = "
    message {
        list_of_high_level_obj [
            some_thing {
                hello
            }
        ]
    }
    ";

    let a = create_castle().await
        .run_message(msg, &ctx).await
        .unwrap();
    println!("{:#?}", a);
}

#[tokio::test]
async fn can_return_option() {
    let ctx = State::new();
    let msg = "
    message {
        option_return
    }
    ";

    let a = create_castle().await
        .run_message(msg, &ctx).await
        .unwrap();
    println!("{:#?}", a);
}