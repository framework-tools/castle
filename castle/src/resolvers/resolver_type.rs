use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::{want::Want, argument::IdentifierAndValueArgument};
use shared::CastleError;

use crate::castle_object::resolver_return_types::Value;

use super::resolver_map::ResolverMap;

pub struct ResolverArguments<'a, C, R> {
    pub wants: Option<&'a Wants>,
    pub arguments: &'a Args,
    pub context: &'a C,
    pub resolver_map: &'a ResolverMap<C, R>,
}

impl<'a, C, R> ResolverArguments<'a, C, R> {
    pub fn new(
        wants: Option<&'a Wants>,
        arguments: &'a Args,
        context: &'a C,
        resolver_map: &'a ResolverMap<C, R>,
    ) -> Self {
        ResolverArguments {
            wants,
            arguments,
            context,
            resolver_map,
        }
    }
}

//A resolver takes in fields (inner wants), arguments and context and returns the resolved want
pub type Resolver<C, R> = fn(Option<&Wants>, &Args, &ResolverMap<C, R>, &C) -> Result<Value<R>, CastleError>;
//Fields that a query wants resolved
pub type Wants = HashMap<Box<str>, Want>;
//Arguments for a resolver
pub type Args = HashMap<Box<str>, IdentifierAndValueArgument>;
//A single resolved want on the top layer of a query
pub type TopLevelResolvers<R> = HashMap<Box<str>, Value<R>>;