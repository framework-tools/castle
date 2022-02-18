use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::{want::{Want, Wants}, argument::{IdentifierAndValueArgument, self}, fn_definition::FnDefinition};
use shared::CastleError;

use crate::castle_object::resolver_return_types::Value;

use super::{resolver_map::ResolverMap, resolver_type::TopLevelResolvers};

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
fn resolve_projection<C, R>(identifier: Box<str>, want: Want, context: &C, resolver_map: &ResolverMap<C, R>) -> Result<Value<R>, CastleError> {
    let resolved;
    let resolver = resolver_map.resolvers.get(&identifier).unwrap();
    match want {
        Want::SingleField(arguments) => {
            resolved = resolver(None, &arguments, resolver_map, context)?;
        },
        Want::ObjectProjection(fields, arguments  ) => {
            resolved = resolver(Some(&fields), &arguments, resolver_map, context)?;
        },
        Want::Match(match_statement) => {
            let mut match_fields = HashMap::new();
            match_fields.insert(identifier, Want::Match(match_statement));
            resolved = resolver(Some(&match_fields), &HashMap::new(), resolver_map, context)?;
        },
    };
    return Ok(resolved)
}