use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Want {
    SingleField(Box<str>),
    Projection(ObjectProjection)
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ObjectProjection {
    identifier: Option<String>,
    fields: Vec<Want>
}

