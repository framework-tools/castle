use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::{want::{Want, ObjectProjection, SingleField}, argument::{IdentifierAndValueArgument}};
use shared::CastleError;

pub type ResolverMap<C, T> = HashMap<Box<str>, Box<Resolver<C, T>>>; 
pub type Resolver<C, T> = dyn Fn(&Wants, &Args, &C) -> Result<ResolvedWant<T>, CastleError>;
pub type Wants = HashMap<Box<str>, Want>;
pub type Args = HashMap<Box<str>, IdentifierAndValueArgument>;
pub type AllResolvedWants<T> = HashMap<Box<str>, ResolvedWant<T>>;
pub type ResolvedWant<T> = HashMap<Box<str>, T>;

///For each top level want, resolve each want & insert in AllResolvedWants
fn resolve_all_wants<C, T>(wants: Wants, resolver_map: ResolverMap<C, T>,  context: C) -> Result<AllResolvedWants<T>, CastleError> {
    let mut all_wants_in_query = HashMap::new();
    for (identifier, want ) in wants {
        let resolved_want = resolve_projection(want, &context, &resolver_map)?;
        all_wants_in_query.insert(identifier, resolved_want);
    }
    return Ok(all_wants_in_query)
}

///For a Top Level Want (object projection or single field)
/// Match the Want
///     If object projection Want
///     - Check it's not a match statement
///     - If not a match:
///         - Use the want's identifier to get the corresponding resolver
///         - Pass in the want's fields, arguments, and context to get the resolved fields
///         - Return the resolved fields    
/// 
///     Else If single field want, resolve it
fn resolve_projection<C, T>(want: Want, context: &C, resolver_map: &ResolverMap<C, T>) -> Result<ResolvedWant<T>, CastleError> {
    let mut resolved_fields = HashMap::new();
    match want {
        Want::SingleField( single_field ) => {
            let identifier = single_field.identifier.clone();
            let arguments = &single_field.arguments;
            let resolver = resolver_map.get(&identifier).unwrap();
            let mut single_field_in_hashmap = HashMap::new();
            let want = Want::SingleField(SingleField {
                identifier: single_field.identifier.clone(),
                arguments: HashMap::new(),
                match_statement: None
            });
            single_field_in_hashmap.insert(identifier, want);
            resolved_fields = resolver(&single_field_in_hashmap, &arguments, &context)?;
        },
        Want::ObjectProjection(ObjectProjection { identifier, arguments, fields, match_statement }) => {
            if match_statement.is_none() {
                if fields.is_none() {
                    return Err(CastleError::EmptyObject("ObjectProjection must have fields or match_statement".into()));
                } else {
                    let fields = fields.unwrap();
                    let resolver = resolver_map.get(&identifier).unwrap();
                    resolved_fields = resolver(&fields, &arguments, &context)?;
                }
            }
        }
    };
    return Ok(resolved_fields)
}

// for (identifier, field) in fields {
//     match field {
//         Want::ObjectProjection(object_projection) => {
//             let resolved_field = resolve_want(field, context)?;
//             resolved_fields.insert(identifier, resolved_field);
//         },
//         Want::SingleField(single_field) => {
//             let resolved_field = resolve_single_field_want(single_field, context)?;
//             resolved_fields.insert(identifier, resolved_field);
//         },
//     }
// }