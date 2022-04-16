
#[derive(Debug, PartialEq, Clone)]
pub enum Primitive {
    String(Box<str>),
    Float(f64),
    Int(i64),
    UInt(u64),
    Boolean(bool),
}