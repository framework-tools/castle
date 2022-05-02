use castle::castle::Castle;

static SCHEMA: &str = "
type Query {
    hello: String
}
";

fn create_castle() -> Castle<()> {
    castle::castle::CastleBuilder::new(SCHEMA)
        .add_resolver(&"hello", |_, _|unimplemented!())
        .build()
        .unwrap()
}

#[test]
fn basic_projection_validates() {
    let msg = r#"
    message {
        hello
    }"#;
    create_castle().validate_message(msg).unwrap();
}