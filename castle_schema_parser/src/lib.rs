use castle_tokenizer::Tokenizer;
use castle_types::AppliedDirective;
use parsers::parse_directives::parse_directives;
use syn::Attribute;


pub mod parsers;


pub fn parse_directives_from_str(directives: &str) -> Vec<AppliedDirective> {
    let bytes = directives.as_bytes();
    let mut tokenizer = Tokenizer::new(bytes);

    return parse_directives(&mut tokenizer).unwrap();
}