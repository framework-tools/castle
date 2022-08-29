
pub trait ConvertFrom<T> {
    fn from(value: T) -> Self;
}