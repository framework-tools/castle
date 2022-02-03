use std::collections::HashSet;

#[derive(Debug, Hash, Eq)]
pub enum Want {
    SingleField(Box<str>),
    Projection(ObjectProjection)
}

#[derive(Debug)]
pub struct ObjectProjection {
    identifier: Option<String>,
    fields: HashSet<Want>
}

