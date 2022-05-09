
#[derive(Debug, PartialEq, Clone)]
pub enum Primitive {
    String(Box<str>),
    Number(Number),
    Boolean(bool),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Number {
    n: NumberKind,
}

impl Number {
    pub fn new(n: impl Into<Number>) -> Self {
        n.into()
    }

    pub fn as_float(&self) -> Option<f64> {
        match self.n {
            NumberKind::Float(f) => Some(f),
            NumberKind::Int(i) => Some(i as f64),
            NumberKind::UInt(u) => Some(u as f64),
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self.n {
            NumberKind::Float(f) => Some(f as i64),
            NumberKind::Int(i) => Some(i),
            NumberKind::UInt(u) => Some(u as i64),
        }
    }

    pub fn as_uint(&self) -> Option<u64> {
        match self.n {
            NumberKind::Float(f) => Some(f as u64),
            NumberKind::Int(i) => Some(i as u64),
            NumberKind::UInt(u) => Some(u),
        }
    }
}

impl From<f64> for Number {
    fn from(f: f64) -> Self {
        Self {
            n: NumberKind::Float(f),
        }
    }
}

impl From<f32> for Number {
    fn from(f: f32) -> Self {
        Self {
            n: NumberKind::Float(f as f64),
        }
    }
}

macro_rules! impl_from_unsigned {
    (
        $($ty:ty),*
    ) => {
        $(
            impl From<$ty> for Number {
                #[inline]
                fn from(u: $ty) -> Self {
                    Number { n: NumberKind::UInt(u as u64) }
                }
            }
        )*
    };
}

macro_rules! impl_from_signed {
    (
        $($ty:ty),*
    ) => {
        $(
            impl From<$ty> for Number {
                #[inline]
                fn from(i: $ty) -> Self {
                    Number {
                        n: if i < 0 {
                            NumberKind::Int(i as i64)
                        } else {
                            NumberKind::UInt(i as u64)
                        }
                    }
                }
            }
        )*
    };
}


impl_from_unsigned!(u8, u16, u32, u64, usize);
impl_from_signed!(i8, i16, i32, i64, isize);

#[derive(Debug, PartialEq, Clone, Copy)]
enum NumberKind {
    Float(f64),
    Int(i64),
    UInt(u64),
}

impl std::fmt::Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::String(s) => write!(f, "\"{}\"", s),
            Primitive::Number(n) => write!(f, "{}", n),
            Primitive::Boolean(b) => write!(f, "{}", b),
        }
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.n {
            NumberKind::Float(f) => write!(fmt, "{}", f),
            NumberKind::Int(i) => write!(fmt, "{}", i),
            NumberKind::UInt(u) => write!(fmt, "{}", u),
        }
    }
}