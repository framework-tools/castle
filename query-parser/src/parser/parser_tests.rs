use crate::parser::parse_query::parse_query;



#[cfg(test)]
#[test]
fn can_parse_empty_query() {
    use std::collections::HashSet;

    let query = "";
    let expected = HashSet::new();
    let actual = parse_query(query).unwrap();
    assert_eq!(expected, actual);
}