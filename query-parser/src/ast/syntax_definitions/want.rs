

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Want {
    SingleField(Box<str>),
    Projection(ObjectProjection)
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ObjectProjection {
    pub identifier: Option<String>,
    pub fields: Vec<Box<Want>>
}

