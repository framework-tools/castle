use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::{want::{Want}, argument::{IdentifierAndValueArgument, self}, fn_definition::FnDefinition};
use shared::CastleError;

use crate::castle_struct::resolver_return_types::ReturnValue;

//A HashMap containing all Resolvers
pub type ResolverMap<C, R> = HashMap<Box<str>, Resolver<C, R>>;

//A resolver takes in fields (inner wants), arguments and context and returns the resolved want
pub type Resolver<C, R> = fn(&Option<Wants>, &Args, &C) -> ReturnValue<R>;
//Fields that a query wants resolved
pub type Wants = HashMap<Box<str>, Want>;
//Arguments for a resolver
pub type Args = HashMap<Box<str>, IdentifierAndValueArgument>;
//A single resolved want - Likely also for top layer
pub type TopLevelResolvers<R> = HashMap<Box<str>, ReturnValue<R>>;

///For each top level want, resolve each want & insert in AllResolvedWants
pub fn resolve_all_wants<C, R>(wants: Wants, resolver_map: &ResolverMap<C, R>,  context: C) -> Result<TopLevelResolvers<R>, CastleError> {
    let mut all_wants_in_query = HashMap::new();
    for (identifier, want ) in wants {
        let resolved_want = resolve_projection(identifier.clone(), want, &context, &resolver_map)?;
        all_wants_in_query.insert(identifier, resolved_want);
    }
    return Ok(all_wants_in_query)
}

///For a Top Level Want (object projection or single field)
/// Match the Want & unwrap from object projection or single field
///     - Check it's not a match statement
///     - If not a match:
///         - Use the want's identifier to get the corresponding resolver
///         - Pass in the want's fields, arguments, and context to get the resolved fields
///         - Return the resolved fields    
fn resolve_projection<C, R>(identifier: Box<str>, want: Want, context: &C, resolver_map: &ResolverMap<C, R>) -> Result<ReturnValue<R>, CastleError> {
    let resolved;
    let resolver = resolver_map.get(&identifier).unwrap();
    match want {
        Want::SingleField(arguments) => {
            resolved = resolver(&None, &arguments, context);
        },
        Want::ObjectProjection(fields, arguments  ) => {
            resolved = resolver(&Some(fields), &arguments, context);
        },
        Want::Match(match_statement) => {
            let mut match_fields = HashMap::new();
            match_fields.insert(identifier, Want::Match(match_statement));
            resolved = resolver(&Some(match_fields), &HashMap::new(), context);
        },
    };
    return Ok(resolved)
}