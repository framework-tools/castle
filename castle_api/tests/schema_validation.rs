
// error if unknown argument provided to directive
// error if directive definition argument with no default is missing in the directive
// error if the directive is allowed on the given directive location

use castle_api::{types::Directive, castle::CastleBuilder};


#[test]
fn directive_with_definition_and_resolver_succeeds() {

    #[castle_api::castle_macro(Directive @bar())]
    struct MockDirective;
    impl Directive for MockDirective {}

    struct Root;
    #[castle_macro::castle(Type)]
    impl Root {
        #[directives("@bar")]
        fn foo(&self, _ctx: &castle_api::types::State) -> String {
            unimplemented!()
        }
    }
    CastleBuilder::new(Root)
    .add_directive(&"bar", MockDirective)
    .build()
    .unwrap();
}

#[test]
fn directive_with_unspecified_arg_fails() {
    #[castle_api::castle_macro(Directive @bar())]
    struct MockDirective;
    impl Directive for MockDirective {}

    struct Root;
    #[castle_macro::castle(Type)]
    impl Root {
        #[directives("@bar(arg: 123)")]
        fn foo(&self, _ctx: &castle_api::types::State) -> String {
            unimplemented!()
        }
    }

    CastleBuilder::new(Root)
    .add_directive(&"bar", MockDirective)
    .build()
    .unwrap_err();
}

#[test]
fn directive_with_string_input_type_mismatch_fails() {
    #[castle_api::castle_macro(Directive @bar(arg: String))]
    struct MockDirective;
    impl Directive for MockDirective {}

    struct Root;
    #[castle_macro::castle(Type)]
    impl Root {
        #[directives("@bar(arg: 123)")]
        fn foo(&self, _ctx: &castle_api::types::State) -> String {
            unimplemented!()
        }
    }


    CastleBuilder::new(Root)
    .add_directive(&"bar", MockDirective)
    .build()
    .unwrap_err();
}

#[test]
fn directive_with_matching_number_type_succeeds() {
    #[castle_api::castle_macro(Directive @bar(arg: f64))]
    struct MockDirective;
    impl Directive for MockDirective {}

    struct Root;
    #[castle_macro::castle(Type)]
    impl Root {
        #[directives("@bar(arg: -123)")]
        fn foo(&self, _ctx: &castle_api::types::State) -> String {
            unimplemented!()
        }
    }


    CastleBuilder::new(Root)
    .add_directive(&"bar", MockDirective)
    .build()
    .unwrap();
}



#[test]
fn directive_with_number_input_type_mismatch_fails() {
    #[castle_api::castle_macro(Directive @bar(arg: u32))]
    struct MockDirective;
    impl Directive for MockDirective {}

    struct Root;
    #[castle_macro::castle(Type)]
    impl Root {
        #[directives("@bar(string: \"string\")")]
        fn foo(&self, _ctx: &castle_api::types::State) -> String {
            unimplemented!()
        }
    }


    CastleBuilder::new(Root)
    .add_directive(&"bar", MockDirective)
    .build()
    .unwrap_err();
}

#[test]
fn field_with_custom_type_mismatch_fails() {
    #[castle_api::castle_macro(Input)]
    struct Custom {
        _x: u32
    }

    struct Root;
    #[castle_macro::castle(Type)]
    impl Root {
        fn foo(&self, _ctx: &castle_api::types::State, _custom: Custom) -> String {
            unimplemented!()
        }
    }

    CastleBuilder::new(Root)
    .build()
    .unwrap();
}


#[test]
fn directive_with_boolean_type_succeeds() {
    #[castle_api::castle_macro(Directive @bar(arg: bool))]
    struct MockDirective;
    impl Directive for MockDirective {}

    struct Root;
    #[castle_macro::castle(Type)]
    impl Root {
        #[directives("@bar(arg: true)")]
        fn foo(&self, _ctx: &castle_api::types::State) -> String {
            unimplemented!()
        }
    }
    CastleBuilder::new(Root)
    .add_directive(&"bar", MockDirective)
    .build()
    .unwrap();
}

#[test]
fn directive_with_array_type_succeeds() {
    // let schema = "
    // directive @bar(arg: Vec<String>) on FieldDefinition

    // type Root {
    //     foo: String @bar(arg: [\"string\"])
    // }
    // ";
    #[castle_api::castle_macro(Directive @bar(arg: Vec<String>))]
    struct MockDirective;
    impl Directive for MockDirective {}

    struct Root;
    #[castle_macro::castle(Type)]
    impl Root {
        #[directives("@bar(arg: [\"a\", \"b\"])")]
        fn foo(&self, _ctx: &castle_api::types::State) -> String {
            unimplemented!()
        }
    }
    CastleBuilder::new(Root)
    .add_directive(&"bar", MockDirective)
    .build()
    .unwrap();
}

#[test]
fn directive_with_string_array_type() {
    #[castle_api::castle_macro(Directive @bar3(arg: Vec<String>))]
    struct MockDirective3;
    impl Directive for MockDirective3 {}

    struct Root;
    #[castle_macro::castle(Type)]
    impl Root {
        #[directives(r#"@bar3(arg: ["string", "string", "string"])"#)]
        fn foo(&self, _ctx: &castle_api::types::State) -> String {
            unimplemented!()
        }
    }
    CastleBuilder::new(Root)
        .add_directive(&"bar3", MockDirective3)
        .build()
        .unwrap();
}
#[test]
fn directive_with_array_type_mismatch_fails() {
    #[castle_api::castle_macro(Directive @bar3(arg: Vec<String>))]
    struct MockDirective3;
    impl Directive for MockDirective3 {}

    struct Root;
    #[castle_macro::castle(Type)]
    impl Root {
        #[directives(r#"@bar3(arg: [123, "string", "string"])"#)]
        fn foo(&self, _ctx: &castle_api::types::State) -> String {
            unimplemented!()
        }
    }
    CastleBuilder::new(Root)
        .add_directive(&"bar3", MockDirective3)
        .build()
        .unwrap_err();
}




// #[directives("@authenticated(a: b)@sorted(a: b)")]
#[test]
fn testing_directives_with_number_arg() {
    #[castle_api::castle_macro(Directive @bar(arg: u32))]
    struct MockDirective;
    impl Directive for MockDirective {}

    struct Root;
    #[castle_macro::castle(Type)]
    impl Root {
        #[directives("@bar(arg: 1)")]
        fn foo(&self, _ctx: &castle_api::types::State) -> String {
            unimplemented!()
        }
    }
    CastleBuilder::new(Root)
        .add_directive(&"bar", MockDirective)
        .build()
        .unwrap();
    // initalise castle based on above Root
    // add directive to castle builder
    // build and validate...
}

// todo: test generic types
// todo: enum types
// todo: option type