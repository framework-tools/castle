use super::Type;



#[derive(Debug, PartialEq)]
pub struct ImplDefinition {
    pub impl_trait: Option<Type>,
    pub impl_for: Type,
    // TODO add impl body
}