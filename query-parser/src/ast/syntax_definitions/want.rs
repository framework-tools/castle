use std::collections::HashSet;



#[derive(Debug)]
enum Want {
    SingleField(Box<str>),
    Projection(HashSet<Want>)
}