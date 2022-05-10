
#[derive(Debug, PartialEq, Clone)]
pub enum Primitive {
    String(Box<str>),
    Number(Number),
    Boolean(bool),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Number {
    pub n: NumberKind,
}

impl Number {
    pub fn new(n: impl Into<Number>) -> Self {
        n.into()
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

macro_rules! from_num_to_primitive {
    (
        $($ty:ty),*
    ) => {
        $(
            impl From<Number> for Option<$ty> {
                fn from(num: Number) -> Option<$ty> {
                    match num.n {
                        NumberKind::UInt(u) => Some(u as $ty),
                        NumberKind::Int(i) => Some(i as $ty),
                        NumberKind::Float(f) => Some(f as $ty),
                    }
                }
            }
        )*
    };
}

from_num_to_primitive!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f64, f32);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NumberKind {
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