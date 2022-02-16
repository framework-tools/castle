
pub enum ReturnValue<C> {
    Null,
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    String(String),
    EnumValue(EnumValue),
    List(Vec<Value>),
    Object(HashMap<String, Value>),
    Custom(Box<C>),
}