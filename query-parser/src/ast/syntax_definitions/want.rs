

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Want {
    SingleField(Box<str>),
    Projection(ObjectProjection)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ObjectProjection {
    pub identifier: Option<Box<str>>,
    pub fields: Vec<Box<Want>>
}

impl Want {
    pub fn new_single_field(identifier: Box<str>) -> Want {
        return Want::SingleField(identifier)
    }
}
