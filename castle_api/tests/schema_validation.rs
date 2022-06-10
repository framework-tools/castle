
// error if unknown argument provided to directive
// error if directive definition argument with no default is missing in the directive
// error if the directive is allowed on the given directive location

use castle_api::types::Directive;

#[castle_api::castle_macro(Directive {
    foo: String,
})]
struct MockDirective;


impl Directive for MockDirective {
    fn name() -> &'static str {
        "bar"
    }
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
        .add_resolver(&"foo", |_: &Field, _: &()|async { unimplemented!() })
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
        .add_resolver(&"foo", |_: &Field, _: &()|async { unimplemented!() })
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
        .add_resolver(&"foo", |_: &Field, _: &()|async { unimplemented!() })
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
        .add_resolver(&"foo", |_: &Field, _: &()|async { unimplemented!() })
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
        .add_resolver(&"foo", |_: &Field, _: &()|async { unimplemented!() })
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
        .add_resolver(&"foo", |_: &Field, _: &()|async { unimplemented!() })
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
        .add_resolver(&"foo", |_: &Field, _: &()|async { unimplemented!() })
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
        .add_resolver(&"foo", |_: &Field, _: &()|async { unimplemented!() })
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
        .add_resolver(&"foo", |_: &Field, _: &()|async { unimplemented!() })
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
        .add_resolver(&"foo", |_: &Field, _: &()|async { unimplemented!() })
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
        .add_resolver(&"foo", |_: &Field, _: &()|async { unimplemented!() })
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
        .add_resolver(&"foo", |_: &Field, _: &()|async { unimplemented!() })
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
        .add_resolver(&"foo", |_: &Field, _: &()|async { unimplemented!() })
        .add_directive(&"bar", MockDirective)
        .build()
        .unwrap_err();
}


// #[directives("@authenticated(a: b)@sorted(a: b)")]
#[test]
fn directive_with_missing_arg_fails() {
    struct Root;
    // directive @foo(arg: String) on FieldDefinition
    #[castle_macro::castle(Type)]
    impl Root {
        #[directives("@bar")]
        fn foo(&self, ctx: &castle_api::types::Context) -> String {
            unimplemented!()
        }
    }
    CastleBuilder::new(Root)
        .add_directive(MockDirective)
        .build()
        .unwrap_err();
    // initalise castle based on above Root
    // add directive to castle builder
    // build and validate...
}

// todo: test generic types
// todo: enum types
// todo: option type