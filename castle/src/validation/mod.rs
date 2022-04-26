pub(crate) mod validate_schema;
pub(crate) mod validate_query;
pub(crate) mod validate_directives_exist;
pub(crate) mod validate_resolvers_exist;

fn join_paths(path: &[&str]) -> String {
    let mut joined = String::new();
    for (i, path) in path.iter().enumerate() {
        if i > 0 {
            joined.push('.');
        }
        joined.push_str(path);
    }
    joined
}