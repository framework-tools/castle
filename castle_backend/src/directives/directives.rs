use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::{directive_definition::{ DirectiveDefinition}};
use shared::CastleError;


pub fn generate_directives() -> Result<HashMap<Box<str>, DirectiveDefinition>, CastleError>{
    let mut directives = HashMap::new();

    
    return Ok(directives)
}