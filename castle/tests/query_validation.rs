use castle::castle::Castle;
use castle_error::CastleError;




fn create_castle() -> Castle<()> {

    let schema = r#"
        input Xyz {
            abc: number
        }

        type Query {
            hello: String,
            foo(bar: number): String,
            baz(arg: Xyz): String,
            list(arg: Vec<String>): String,
            list2(arg: Vec<Xyz>): String,
        }
    "#;
    castle::castle::CastleBuilder::new(schema)
        .add_resolver("hello", |_, _|unimplemented!())
        .add_resolver("foo", |_, _|unimplemented!())
        .add_resolver("baz", |_, _|unimplemented!())
        .add_resolver("list", |_, _|unimplemented!())
        .add_resolver("list2", |_, _|unimplemented!())
        .build()
        .unwrap()
}

#[test]
fn basic_projection_validates() {
    let msg = r#"
    message {
        hello
    }"#;
    create_castle()
        .validate_message(msg)
        .unwrap();
}

#[test]
fn basic_projection_validates_breaks_with_mismatch() -> Result<(), CastleError> {
    let msg = r#"
    message {
        world
    }"#;
    create_castle()
        .validate_message(msg)
        .expect_err("schema should fail but didn't");
    Ok(())
}

#[test]
fn projection_with_unknown_args_fails() {
    let msg = "
    message {
        hello(arg: 123)
    }
    ";

    create_castle()
        .validate_message(msg)
        .expect_err("schema should fail but didn't");
}

#[test]
fn projection_with_missing_args_fails() {
    let msg = "
    message {
        foo()
    }
    ";

    create_castle()
        .validate_message(msg)
        .expect_err("schema should fail but didn't");
}

#[test]
fn nested_input_args_validates() {
    let msg = "
    message {
        baz(arg: {
            abc: 123
        })
    }
    ";

    create_castle()
        .validate_message(msg)
        .unwrap();
}

#[test]
fn nested_input_with_unknown_args_fails() {
    let msg = "
    message {
        baz(arg: {
            abc: 123,
            def: 123
        })
    }
    ";

    create_castle()
        .validate_message(msg)
        .unwrap_err();
}

#[test]
fn list_items_without_correct_type_fails() {
    let msg = "
    message {
        list(arg: [\"abc\", 123]) 
    }
    ";

    create_castle()
        .validate_message(msg)
        .unwrap_err();
}

#[test]
fn list_item_with_input_type_validates() {
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
        .validate_message(msg)
        .unwrap();
}

#[test]
fn list_item_with_input_type_mismatch_fails() {
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
        .validate_message(msg)
        .unwrap_err();
}
