use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::fn_definition::FnDefinition;
use shared::CastleError;

#[derive(Debug, PartialEq)]
pub struct Resolver {
    pub resolver_definition: FnDefinition,
    // unsure about this field currently
    // pub function: fn<T>() -> T
}
impl Resolver {
    pub fn new(resolver_definition: FnDefinition) -> Self {
        Self {
            resolver_definition,
            // function: None,
        }
    }
}

pub fn generate_resolvers() -> Result<HashMap<Box<str>, Resolver>, CastleError>{
    let mut resolvers = HashMap::new();

    
    return Ok(resolvers)
}