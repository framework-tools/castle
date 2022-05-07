use std::fmt::Debug;

use castle_api::{castle::CastleBuilder, types::result::CastleResult, Resolver, Value};
use castle_query_parser::Field;

async fn run_schema_with_query<Ctx: Debug + Send + Sync + 'static, E: Debug + 'static>(
    schema: &str,
    query: &str,
    resolvers: Vec<(&str, impl Resolver<Ctx, E> + 'static)>,
    ctx: &Ctx,
) -> CastleResult<Ctx, E> {
    let mut castle = CastleBuilder::new(schema);
    for (field_name, resolver) in resolvers {
        castle = castle.add_resolver(field_name, resolver);
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
    let result: CastleResult<(), ()> = run_schema_with_query(&schema, &query, vec![(&"bar", |_: &Field, _: &()| Ok("hello".into()))], &()).await;
    let expected = CastleResult {
        data: [("bar".into(), Value::String("hello".into()))].into(),
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
    let result: CastleResult<(), ()> = run_schema_with_query(&schema, &query, vec![(&"bar", |_: &Field, _: &()| Ok(32.into()))], &()).await;
    let expected = CastleResult {
        data: [("bar".into(), Value::Int(32))].into(),
        errors: vec![],
    };
    assert_eq!(result, expected)
}
