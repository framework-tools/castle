use castle_api::{castle::Castle};
use castle_error::CastleError;
use castle_query_parser::Field;
use castle_api::castle::CastleBuilder;

fn create_castle() -> Castle<(), ()> {
    let schema = r#"
        input Xyz {
            abc: number
        }

        type Root {
            hello: String,
            foo(bar: number): String,
            sigma(): number,
            baz(arg: Xyz): String,
            list(arg: Vec<String>): String,
            list2(arg: Vec<Xyz>): String,
            foobar(arg1: number arg2: String): String,
            oogabooga(is_true: bool): String,
            some_thing: SomeThing,
            thing_is_true: bool,
            high_level_obj: HighLevelObj,
            list_of_some_things: Vec<SomeThing>,
            list_of_high_level_obj: Vec<HighLevelObj>,
        }

        type SomeThing {
            hello: String
            sigma: number
            thing_is_true: bool
        }

        type HighLevelObj {
            some_thing: SomeThing
        }
    "#;
    CastleBuilder::new(schema)
        .add_resolver("hello", |_: &Field, _: &()|unimplemented!())
        .add_resolver("foo", |_: &Field, _: &()|unimplemented!())
        .add_resolver("baz", |_: &Field, _: &()|unimplemented!())
        .add_resolver("list", |_: &Field, _: &()|unimplemented!())
        .add_resolver("list2", |_: &Field, _: &()|unimplemented!())
        .add_resolver("foobar", |_: &Field, _: &()|unimplemented!())
        .add_resolver("oogabooga", |_: &Field, _: &()|unimplemented!())
        .add_resolver("some_thing", |_: &Field, _: &()|unimplemented!())
        .add_resolver("sigma", |_: &Field, _: &()|unimplemented!())
        .add_resolver("thing_is_true", |_: &Field, _: &()|unimplemented!())
        .add_resolver("high_level_obj", |_: &Field, _: &()|unimplemented!())
        .add_resolver("list_of_some_things", |_: &Field, _: &()|unimplemented!())
        .add_resolver("list_of_high_level_obj", |_: &Field, _: &()|unimplemented!())
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
        .unwrap_err();
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
        .unwrap_err();
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
        .unwrap_err();
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

#[test]
fn item_with_multiple_args_fails_if_wrong_type() {
    let msg = "
    message {
        foobar(arg1: 123 arg2: 5.5)
    }
    ";

    create_castle()
        .validate_message(msg)
        .unwrap_err();
}

#[test]
fn item_with_too_many_args_fails() {
    let msg = "
    message {
        foobar(arg1: 123 arg2: 5.5, arg3: 4)
    }
    ";

    create_castle()
        .validate_message(msg)
        .unwrap_err();
}

#[test]
fn item_with_too_little_args_fails() {
    let msg = "
    message {
        foobar(arg1: 123)
    }
    ";

    create_castle()
        .validate_message(msg)
        .unwrap_err();
}

#[test]
fn item_with_matching_args_should_pass() {
    let msg = r#"
    message {
        foobar(arg1: 123, arg2: "Hello World")
    }
    "#;

    create_castle()
        .validate_message(msg)
        .unwrap();
}

#[test]
fn item_with_correct_return_type_should_pass() {
    let msg = r#"
    message {
        foobar(arg1: 123, arg2: "Hello World")
    }
    "#;

    create_castle()
        .validate_message(msg)
        .unwrap();
}

#[test]
fn fails_if_number_mismatch_on_argument() {
    let msg = r#"
    message {
        foobar(arg1: "Hello World", arg2: "Hello World")
    }
    "#;

    create_castle()
        .validate_message(msg)
        .unwrap_err();
}

#[test]
fn fails_if_string_mismatch_on_argument() {
    let msg = r#"
    message {
        foobar(arg1: 123, arg2: 55)
    }
    "#;

    create_castle()
        .validate_message(msg)
        .unwrap_err();
}

#[test]
fn passes_validating_bool_input() {
    let msg = r#"
    message {
        oogabooga(is_true: true)
    }
    "#;

    create_castle()
        .validate_message(msg)
        .unwrap();
}

#[test]
fn fails_if_bool_mismatch_on_arg() {
    let msg = r#"
    message {
        oogabooga(is_true: "this should fail")
    }
    "#;

    create_castle()
        .validate_message(msg)
        .unwrap_err();
}

#[test]
fn fails_if_type_mismatch_for_user_defined_type() {
    let msg = r#"
    message {
        baz(arg: true)
    }
    "#;

    create_castle()
        .validate_message(msg)
        .unwrap_err();

    let msg = r#"
    message {
        baz(arg: {
            abc: "Hello World"
        })
    }
    "#;

    create_castle()
        .validate_message(msg)
        .unwrap_err();
}

#[test]
fn validates_vec_that_is_correctly_typed(){
    let msg = r#"
    message {
        list(arg: ["Hello", "World"])
    }
    "#;

    create_castle()
        .validate_message(msg)
        .unwrap();
}

#[test]
fn can_validate_valid_object_projection(){
    let msg = r#"
    message {
        some_thing {
            hello
            sigma
            thing_is_true
        }
    }
    "#;

    create_castle()
        .validate_message(msg)
        .unwrap();
}

#[test]
fn fails_if_field_is_not_defined_on_type(){
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
        .validate_message(msg)
        .unwrap_err();
}

#[test]
fn empty_projection_succeeds(){
    let msg = r#"
    message {
        high_level_obj {

        }
    }
    "#;

    create_castle()
        .validate_message(msg)
        .unwrap();
}

#[test]
fn fails_if_invalid_nested_obj(){
    let msg = r#"
    message {
        high_level_obj {
            some_thing
        }
    }
    "#;

    create_castle()
        .validate_message(msg)
        .unwrap_err();
}

#[test]
fn fails_if_nested_obj_has_undefined_field(){
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
        .validate_message(msg)
        .unwrap_err();
}

#[test]
fn passes_for_valid_array_projection(){
    let msg = r#"
    message {
        list_of_some_things [
            hello
        ]
    }
    "#;

    create_castle()
        .validate_message(msg)
        .unwrap();
}

#[test]
fn fails_array_projection_with_invalid_field(){
    let msg = r#"
    message {
        list_of_some_things [
            hello,
            omega
        ]
    }
    "#;

    create_castle()
        .validate_message(msg)
        .unwrap_err();
}

#[test]
fn passes_valid_mesage_with_multiple_layers_of_nesting() {
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
        .validate_message(msg)
        .unwrap();
}

#[test]
fn fails_for_invalid_field_multiple_layers_of_nesting() {
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
        .validate_message(msg)
        .unwrap_err();
}

