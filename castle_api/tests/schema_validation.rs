
// error if unknown argument provided to directive
// error if directive definition argument with no default is missing in the directive
// error if the directive is allowed on the given directive location
use std::fmt::Debug;
use castle_api::{castle::CastleBuilder, Directive};
use castle_query_parser::Field;

struct MockDirective;

impl<Ctx: Send + 'static + Debug, E: Debug + 'static> Directive<Ctx, E> for MockDirective {}

#[tokio::test]
async fn schema_without_type_message_fails() {
    let schema = "

    ";

    CastleBuilder::<(), ()>::new(schema)
        .build()
        .unwrap_err();
}

#[tokio::test]
async fn schema_using_string_primitive_works() {
    let schema = "
    type Root {
        foo: String
    }
    ";

    CastleBuilder::<(), ()>::new(schema)
        .add_resolver(&"foo", |_: &Field, _: &()| unreachable!()).await
        .build()
        .unwrap();
}

#[tokio::test]
async fn schema_using_bool_primitive_works() {
    let schema = "
    type Root {
        foo: bool
    }
    ";

    CastleBuilder::<(), ()>::new(schema)
        .add_resolver(&"foo", |_: &Field, _: &()| unreachable!()).await
        .build()
        .unwrap();
}

#[tokio::test]
async fn schema_using_number_primitive_works() {
    let schema = "
    type Root {
        foo: number
    }
    ";

    CastleBuilder::<(), ()>::new(schema)
        .add_resolver(&"foo", |_: &Field, _: &()| unreachable!()).await
        .build()
        .unwrap();
}

#[tokio::test]
async fn schema_with_non_existent_type_message_fails() {
    let schema = "
    type Root {
        foo: NonExistent
    }
    ";

    CastleBuilder::<(), ()>::new(schema)
        .build()
        .unwrap_err();
}

#[tokio::test]
async fn schema_with_existent_type_succeeds() {
    let schema = "
    type Root {
        foo: Bar
    }

    type Bar {

    }
    ";

    CastleBuilder::<(), ()>::new(schema)
        .add_resolver(&"foo", |_: &Field, _: &()| unreachable!()).await
        .build()
        .unwrap();
}

#[tokio::test]
async fn non_existent_directive_fails() {
    let schema = "
    type Root {
        foo: String @foo
    }
    ";

    CastleBuilder::<(), ()>::new(schema)
        .add_resolver(&"foo", |_: &Field, _: &()| unreachable!()).await
        .build()
        .unwrap_err();
}

#[tokio::test]
async fn directive_with_missing_argument_fails() {
    let schema = "
    directive @foo(arg: String) on FieldDefinition

    type Root {
        foo: String @foo
    }
    ";

    CastleBuilder::<(), ()>::new(schema)
        .add_resolver(&"foo", |_: &Field, _: &()| unreachable!()).await
        .build()
        .unwrap_err();
}

#[tokio::test]
async fn directive_with_definition_and_resolver_succeeds() {
    let schema = "
    directive @bar on FieldDefinition

    type Root {
        foo: String @bar
    }
    ";

    CastleBuilder::<(), ()>::new(schema)
        .add_resolver(&"foo", |_: &Field, _: &()| unreachable!()).await
        .add_directive(&"bar", MockDirective)
        .build()
        .unwrap();
}

#[tokio::test]
async fn directive_on_wrong_location_fails() {
    let schema = "
    directive @bar on VariantDefinition

    type Root {
        foo: String @bar
    }
    ";

    CastleBuilder::<(), ()>::new(schema)
        .add_resolver(&"foo", |_: &Field, _: &()| unreachable!()).await
        .add_directive(&"bar", MockDirective)
        .build()
        .unwrap_err();
}

#[tokio::test]
async fn directive_with_unspecified_arg_fails() {
    let schema = "
    directive @bar on FieldDefinition

    type Root {
        foo: String @bar(arg: 123)
    }
    ";

    CastleBuilder::<(), ()>::new(schema)
        .add_resolver(&"foo", |_: &Field, _: &()| unreachable!()).await
        .add_directive(&"bar", MockDirective)
        .build()
        .unwrap_err();
}

#[tokio::test]
async fn directive_with_string_input_type_mismatch_fails() {
    let schema = "
    directive @bar(arg: String) on FieldDefinition

    type Root {
        foo: String @bar(arg: 123)
    }
    ";

    CastleBuilder::<(), ()>::new(schema)
        .add_resolver(&"foo", |_: &Field, _: &()| unreachable!()).await
        .add_directive(&"bar", MockDirective)
        .build()
        .unwrap_err();
}

#[tokio::test]
async fn directive_with_matching_number_type_succeeds() {
    let schema = "
    directive @bar(arg: number) on FieldDefinition

    type Root {
        foo: String @bar(arg: -123)
    }
    ";

    CastleBuilder::<(), ()>::new(schema)
        .add_resolver(&"foo", |_: &Field, _: &()| unreachable!()).await
        .add_directive(&"bar", MockDirective)
        .build()
        .unwrap();
}

#[tokio::test]
async fn directive_with_matching_number_type_succeeds_with_casting() {
    let schema = "
    directive @bar(arg: number) on FieldDefinition

    type Root {
        foo: String @bar(arg: 123)
    }
    ";

    CastleBuilder::<(), ()>::new(schema)
        .add_resolver(&"foo", |_: &Field, _: &()| unreachable!()).await
        .add_directive(&"bar", MockDirective)
        .build()
        .unwrap();
}


#[tokio::test]
async fn directive_with_number_input_type_mismatch_fails() {
    let schema = "
    directive @bar(arg: number) on FieldDefinition

    type Root {
        foo: String @bar(arg: \"string\")
    }
    ";

    CastleBuilder::<(), ()>::new(schema)
        .add_resolver(&"foo", |_: &Field, _: &()| unreachable!()).await
        .add_directive(&"bar", MockDirective)
        .build()
        .unwrap_err();
}

#[tokio::test]
async fn directive_with_custom_type_mismatch_fails() {
    let schema = "
    directive @bar(arg: Custom) on FieldDefinition

    type Root {
        foo1: String @bar(arg: 123)
        foo2: String @bar(arg: { a: 123 })
    }

    input Custom {

    }
    ";

    CastleBuilder::<(), ()>::new(schema)
        .add_resolver(&"foo", |_: &Field, _: &()| unreachable!()).await
        .add_directive(&"bar", MockDirective)
        .build()
        .unwrap_err();
}

#[tokio::test]
async fn directive_with_custom_type_succeeds() {
    let schema = "
    directive @bar(arg: Custom) on FieldDefinition

    type Root {
        foo: String @bar(arg: { a: 123 })
    }

    input Custom {
        a: number
    }
    ";

    CastleBuilder::<(), ()>::new(schema)
        .add_resolver(&"foo", |_: &Field, _: &()| unreachable!()).await
        .add_directive(&"bar", MockDirective)
        .build()
        .unwrap();
}


#[tokio::test]
async fn directive_with_boolean_type_succeeds() {
    let schema = "
    directive @bar(arg: bool) on FieldDefinition

    type Root {
        foo: String @bar(arg: true)
    }
    ";

    CastleBuilder::<(), ()>::new(schema)
        .add_resolver(&"foo", |_: &Field, _: &()| unreachable!()).await
        .add_directive(&"bar", MockDirective)
        .build()
        .unwrap();
}

#[tokio::test]
async fn directive_with_array_type_succeeds() {
    let schema = "
    directive @bar(arg: Vec<String>) on FieldDefinition

    type Root {
        foo: String @bar(arg: [\"string\"])
    }
    ";

    CastleBuilder::<(), ()>::new(schema)
        .add_resolver(&"foo", |_: &Field, _: &()| unreachable!()).await
        .add_directive(&"bar", MockDirective)
        .build()
        .unwrap();
}

#[tokio::test]
async fn directive_with_array_type_mismatch_fails() {
    let schema = "
    directive @bar(arg: Vec<String>) on FieldDefinition

    type Root {
        foo: String @bar(arg: [123])
    }
    ";

    CastleBuilder::<(), ()>::new(schema)
        .add_resolver(&"foo", |_: &Field, _: &()| unreachable!()).await
        .add_directive(&"bar", MockDirective)
        .build()
        .unwrap_err();
}

#[tokio::test]
async fn directive_with_too_many_generic_params_fails() {
    let schema = "
    directive @bar(arg: Vec<String, String>) on FieldDefinition

    type Root {
        foo: String @bar(arg: [\"string\"])
    }
    ";

    CastleBuilder::<(), ()>::new(schema)
        .add_resolver(&"foo", |_: &Field, _: &()| unreachable!()).await
        .add_directive(&"bar", MockDirective)
        .build()
        .unwrap_err();
}


#[tokio::test]
async fn directive_with_missing_arg_fails() {
    let schema = "
    directive @foo(arg: String) on FieldDefinition

    type Root {
        foo: String @foo
    }
    ";

    CastleBuilder::<(), ()>::new(schema)
        .add_resolver(&"foo", |_: &Field, _: &()| unreachable!()).await
        .add_directive(&"foo", MockDirective)
        .build()
        .unwrap_err();
}


// todo: test generic types
// todo: enum types
// todo: option type