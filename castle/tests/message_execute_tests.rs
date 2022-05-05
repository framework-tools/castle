use castle::castle::CastleBuilder;
use castle_error::CastleError;
use query_parser::Field;




#[test]
fn resolver_can_return_string() {
    let schema = "
    type Root {
        bar(arg: String): String
    }
    ";
    let castle = CastleBuilder::<(), ()>::new(schema)
        .add_resolver(&"bar", |_: &Field, _: &()| Ok("hello".into()))
        .build().unwrap();

    let query = "
        message {
            bar(arg: \"world\")
        }
    ";

        struct CastleResult<Ctx, E> {
            data: Value<Ctx>,
            errors: Vec<E>
        }

        // return type of run_message() -> Result<CastleResult, CastleError>

    let result = castle.run_message(query, ctx).await
        .unwrap()
}

