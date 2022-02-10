use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::{fn_definition::FnDefinition, directive_definition::DirectiveDefinition};
use shared::CastleError;

pub struct Directive {
    pub resolver_definition: DirectiveDefinition,
    // unsure about this field currently
    // pub function: fn(args: &HashMap<String, String>) -> Result<String, CastleError>,
}

pub fn generate_directives() -> Result<HashMap<Box<str>, DirectiveDefinition>, CastleError>{
    let mut directives = HashMap::new();

    
    return Ok(directives)
}