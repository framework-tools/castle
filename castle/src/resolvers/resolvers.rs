use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::{want::{Want, ObjectProjection, SingleField, FieldsType}, argument::{IdentifierAndValueArgument}};
use shared::CastleError;

//A HashMap containing all Resolversß
pub type ResolverMap<C, R> = HashMap<Box<str>, Resolver<C, R>>; 
//A resolver takes in fields (inner wants), arguments and context and returns the resolved want
pub type Resolver<C, R> = fn(&Wants, &Args, &C) -> Result<TopLevelResolvers<R>, CastleError>;
//Fields that a query wants resolved
pub type Wants = HashMap<Box<str>, Want>;
//Arguments for a resolver
pub type Args = HashMap<Box<str>, IdentifierAndValueArgument>;
//The top level of resolved wants - The least nested layer in the query (not wrapped in another Want)
pub type AllResolvedWants<T> = HashMap<Box<str>, TopLevelResolvers<T>>;
//A single resolved want - Likely also for top layer
pub type TopLevelResolvers<T> = HashMap<Box<str>, T>;

///For each top level want, resolve each want & insert in AllResolvedWants
pub fn resolve_all_wants<C, T>(wants: Wants, resolver_map: &ResolverMap<C, T>,  context: C) -> Result<AllResolvedWants<T>, CastleError> {
    let mut all_wants_in_query = HashMap::new();
    for (identifier, want ) in wants {
        let resolved_want = resolve_projection(want, &context, &resolver_map)?;
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
/// 
///     Else If single field want, resolve it
fn resolve_projection<C, T>(want: Want, context: &C, resolver_map: &ResolverMap<C, T>) -> Result<TopLevelResolvers<T>, CastleError> {
    let mut resolved_fields = HashMap::new();
    match want {
        Want::ObjectProjection(ObjectProjection { identifier, arguments, fields }) => {
            match fields{
                FieldsType::Regular(fields) => {
                    let resolver = resolver_map.get(&identifier).unwrap();
                    let resolved_fields = resolver(&fields, &arguments, context)?;
                    return Ok(resolved_fields);
                },
                FieldsType::Match(match_statement) => {
                    // need to implement
                }
            }
        },
        //Single field might need to be refactored - Not as good as it could be
        Want::SingleField( single_field ) => {
            let identifier = single_field.identifier.clone();
            let arguments = &single_field.arguments;
            let resolver = resolver_map.get(&identifier).unwrap();
            let mut single_field_in_hashmap = HashMap::new();
            let want = Want::SingleField(SingleField {
                identifier: single_field.identifier.clone(),
                arguments: HashMap::new(),
            });
            single_field_in_hashmap.insert(identifier, want);
            resolved_fields = resolver(&single_field_in_hashmap, &arguments, &context)?;
        }
    };
    return Ok(resolved_fields)
}