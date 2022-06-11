use std::fmt::Display;


/// Definition of a type as a return value;
/// e.g. `Vec<User>`, `User` or `String`
#[derive(Debug, PartialEq, Clone)]
pub struct Kind {
    pub ident: Box<str>,
    pub generics: Vec<Kind>,
}

pub trait HasKind {
    fn kind() -> Kind;
}


impl<T, E> HasKind for Result<T, E> where T: HasKind {
    fn kind() -> Kind {
        T::kind()
    }
}

impl<G> HasKind for Vec<G> where G: HasKind {
    fn kind() -> Kind {
        Kind {
            ident: "Vec".into(),
            generics: vec![G::kind()],
        }
    }
}

impl<G> HasKind for Option<G> where G: HasKind {
    fn kind() -> Kind {
        Kind {
            ident: "Option".into(),
            generics: vec![G::kind()],
        }
    }
}

macro_rules! impl_has_kind_for_scalars {
    (
        $($ty:ty: $ident:ident,)*
    ) => {
        $(
            impl HasKind for $ty {
                fn kind() -> Kind {
                    Kind {
                        ident: stringify!($ident).into(),
                        generics: vec![],
                    }
                }
            }
        )*
    };
}

impl_has_kind_for_scalars! {
    isize: number,
    i64: number,
    i32: number,
    i16: number,
    i8: number,
    usize: number,
    u64: number,
    u32: number,
    u16: number,
    u8: number,
    f64: number,
    f32: number,
    bool: bool,
    String: String,
    (): void,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ident)?;
        if !self.generics.is_empty() {
            write!(f, "<")?;
            for (i, kind) in self.generics.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", kind)?;
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}