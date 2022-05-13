


use castle_api::{castle::CastleBuilder, types::result::CastleResult, Resolver, Value};
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

#[tokio::test]
async fn resolver_can_return_string() {
    let schema = "
    type Root {
        bar(arg: String): String
    }
    ";
    let query = "
        message {
            bar(arg: \"world\")
        }
    ";
    let result: CastleResult<i32, ()> = run_schema_with_query(&schema, &query, vec![(&"bar", |_: &Field, _: &i32| async { Ok("hello".into()) })], &123).await;
    let expected = CastleResult {
        data: [("bar".into(), "hello".into())].into(),
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
    let result: CastleResult<(), ()> = run_schema_with_query(&schema, &query, vec![(&"bar", |_: &Field, _: &()| async { Ok(32.into()) })], &()).await;
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

    let _: CastleResult<i32, ()> = run_schema_with_query(&schema, &query, vec![
        ("foo", foo),
    ], &123).await;

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

    let result: CastleResult<(), ()> = run_schema_with_query(&schema, &query, vec![(&"foo", |_: &Field, _: &()| async { Ok(Value::Void) })], &()).await;
    let expected = CastleResult {
        data: [].into(),
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