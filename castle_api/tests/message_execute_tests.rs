use castle_api::{castle::CastleBuilder, types::result::CastleResult, Directive, Resolver, Value, Inputs, Next};
use castle_query_parser::Field;

async fn run_schema_with_query<Ctx: Send + Sync + 'static, E: Send + Sync + 'static>(
    schema: &str,
    query: &str,
    resolvers: Vec<(&str, impl Resolver<Ctx, E> + 'static)>,
    ctx: &Ctx,
) -> CastleResult<Ctx, E> {
    let mut castle = CastleBuilder::new(schema);
    for (field_name, resolver) in resolvers {
        castle.add_resolver(field_name, resolver);
    }
    castle
        .build()
        .unwrap()
        .run_message(query, ctx)
        .await
        .unwrap()
}

async fn run_schema_and_directive_with_query<
    Ctx: Send + Sync + 'static,
    E: Send + Sync + 'static,
>(
    schema: &str,
    query: &str,
    resolvers: Vec<(&str, impl Resolver<Ctx, E> + 'static)>,
    directives: Vec<(&str, impl Directive<Ctx, E> + 'static)>,
    ctx: &Ctx,
) -> CastleResult<Ctx, E> {
    let mut castle = CastleBuilder::new(schema);
    for (field_name, resolver) in resolvers {
        castle.add_resolver(field_name, resolver);
    }
    for (field_name, directive) in directives {
        castle.add_directive(field_name, directive);
    }
    castle
        .build()
        .unwrap()
        .run_message(query, ctx)
        .await
        .unwrap()
}

#[tokio::test]
async fn resolver_can_return_string() {
    let schema = "
    type Root {
        bar: String
    }
    ";
    let query = "
        message {
            bar
        }
    ";

    let result: CastleResult<i32, ()> = run_schema_with_query(
        &schema,
        &query,
        vec![(&"bar", |_: &Field, _: &i32| async { Ok("foo".into()) })],
        &123,
    )
    .await;
    let expected = CastleResult {
        data: [("bar".into(), "foo".into())].into(),
        errors: vec![],
    };

    assert_eq!(result, expected)
}

#[tokio::test]
async fn resolver_can_return_number() {
    let schema = "
    type Root {
        bar(arg: String): number
    }
    ";
    let query = "
        message {
            bar(arg: \"world\")
        }
    ";
    let result: CastleResult<(), ()> = run_schema_with_query(
        &schema,
        &query,
        vec![(&"bar", |_: &Field, _: &()| async { Ok(32.into()) })],
        &(),
    )
    .await;
    let expected = CastleResult {
        data: [("bar".into(), 32.into())].into(),
        errors: vec![],
    };
    assert_eq!(result, expected)
}

async fn foo(_: &Field, _: &i32) -> Result<Value<i32, ()>, ()> {
    Ok(32.into())
}

#[tokio::test]
async fn async_resolver_doesnt_complain() {
    let schema = "
    type Root {
        bar(arg: String): number
    }
    ";
    let query = "
        message {
            bar(arg: \"world\")
        }
    ";

    let _: CastleResult<i32, ()> =
        run_schema_with_query(&schema, &query, vec![("bar", foo)], &123).await;
}

#[tokio::test]
async fn testing_void() {
    let query = "
    message {
        foo()
    }
    ";

    let schema = "

    type Root {
        foo: void
    }
    ";

    let result: CastleResult<(), ()> = run_schema_with_query(
        &schema,
        &query,
        vec![(&"foo", |_: &Field, _: &()| async { Ok(Value::Void) })],
        &(),
    )
    .await;
    let expected = CastleResult {
        data: [].into(),
        errors: vec![],
    };
    assert_eq!(result, expected)
}

#[tokio::test]
async fn test_directives() {
    let query = "
    message {
        bar
    }
    ";

    struct ReturnsFooDirective;

    #[async_trait::async_trait]
    impl<Ctx, E> Directive<Ctx, E> for ReturnsFooDirective {
        async fn field_visitor(&self, _field: &Field, _directive_args: &Inputs, _value: Next<Ctx, E>, _context: &Ctx) -> Result<Value<Ctx, E>, E> where 
            Ctx: Send + Sync,
            E: Send + Sync + 'static {
            Ok("foo".into())
        }
    }

    let schema = "
    directive @returns_foo on FieldDefinition
    type Root {
        bar: String @returns_foo
    }
    ";

    let result: CastleResult<(), ()> = run_schema_and_directive_with_query(
        &schema,
        &query,
        vec![(&"bar", |_: &Field, _: &()| async {
            Ok(Value::String("hello".into()))
        })],
        vec![("returns_foo", ReturnsFooDirective)],
        &(),
    )
    .await;
    let expected = CastleResult {
        data: [("bar".into(), "foo".into())].into(),
        errors: vec![],
    };
    assert_eq!(result, expected)
}

// use std::future::Future;

// struct A;
// #[async_trait::async_trait]
// trait Foo {
//     async fn foo(&self, a: &A, x: &()) -> ();
// }

// #[async_trait::async_trait]
// impl<F> Foo for F
// where
//     F: for<'a, 'b> Fn2<&'a A, &'b ()> + Sync,
//     for<'a, 'b> <F as Fn2<&'a A, &'b ()>>::Output: Future<Output = ()> + Send,
// {
//     async fn foo(&self, a: &A, x: &()) {
//         self(a, x).await
//     }
// }

// trait Fn2<Arg1, Arg2>: Fn(Arg1, Arg2) -> <Self as Fn2<Arg1, Arg2>>::Output {
//     type Output;
// }
// impl<F: Fn(Arg1, Arg2) -> O, Arg1, Arg2, O> Fn2<Arg1, Arg2> for F {
//     type Output = O;
// }

// #[tokio::test]
// async fn main() {
//     async fn some_fn(abc: Vec<(&str, impl Foo + 'static)>) {}

//     async fn abc(a: &A, x: &()) -> () {

//     }

//     let x = vec![
//         ("abc", abc)
//     ];

//     some_fn(x).await;
// }
