use castle_tokenizer::Tokenizer;
use castle_types::AppliedDirective;
use parsers::parse_directives::parse_directives;


pub mod parsers;


pub fn parse_directives_from_str(directives: &str) -> Vec<AppliedDirective> {
    let bytes = directives.as_bytes();
    let mut tokenizer = Tokenizer::new(bytes); 
    parse_directives(&mut tokenizer).unwrap()
}

#[test]
fn test_parse_directives_from_str() {
    let a = parse_directives_from_str("@authenticated(a: b)@sorted(a: b)");
    println!("{:#?}", a);

}