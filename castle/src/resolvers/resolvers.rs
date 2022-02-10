use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::fn_definition::FnDefinition;
use shared::CastleError;

pub struct Resolver {
    pub resolver_definition: FnDefinition,
    // unsure about this field currently
    // pub function: fn(args: &HashMap<String, String>) -> Result<String, CastleError>,
}

pub fn generate_resolvers() -> Result<HashMap<Box<str>, Resolver>, CastleError>{
    let mut resolvers = HashMap::new();

    
    return Ok(resolvers)
}