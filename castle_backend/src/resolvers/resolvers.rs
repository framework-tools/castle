use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::{want::Want, argument::ArgumentOrTuple};
use shared::CastleError;

pub type ResolverMap<C, T> = HashMap<Box<str>, Box<Resolver<C, T>>>; 
pub type Resolver<C, T> = dyn Fn(Wants, Args, C) -> Result<ResolverOutput<T>, CastleError>;
pub type Wants = HashMap<Box<str>, Want>;
pub type Args = HashMap<Box<str>, ArgumentOrTuple>;
pub type ResolverOutput<T> = HashMap<Box<str>, ResolvedWant<T>>;
pub type ResolvedWant<T> = HashMap<Box<str>, T>;

///For each top level want, resolve each want & insert in resolver_map
fn resolve_all_wants<C, T>(wants: Wants,  context: C) -> Result<ResolverOutput<T>, CastleError> {
    let mut resolver_map = HashMap::new();
    return Ok(resolver_map)
}
///for a top level want (object projection or single field)
/// If single field want, resolve it
/// Else if object projection want, resolve each field
fn resolve_want<C, T>(wants: Want,  context: C) -> Result<ResolvedWant<T>, CastleError> {
    let want = HashMap::new();
    return Ok(want)
}