use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::{fn_definition::FnDefinition, want::Want};
use shared::CastleError;

type ResolverMap<C, O> = HashMap<String, Resolver<C, O>>; 
pub type Resolver<C, O> = dyn Fn(Wants, Args, C) -> Result<O, CastleError>;
type Wants = HashMap<String, Want>;
type Args = HashMap<String, Argument>;
