use std::collections::{HashMap, HashSet};

use parser_and_schema::ast::syntax_definitions::want::{Wants, Want};

use crate::castle_struct::resolver_return_types::Value;

use super::resolvers::{Args, ResolverMap};


pub fn generic_resolver<C, R>(
    wants: Option<&Wants>, 
    possible_fields: &HashSet<Box<str>>,
    fields_with_values_from_db: &mut HashMap<Box<str>, Value<R>>,
    args: &Args, 
    resolver_map: &ResolverMap<C, R>, 
    context: &C
) -> Value<R> {
    let wants = wants.unwrap();
    let mut resolved_fields= HashMap::new();
    for (identifier, value) in fields_with_values_from_db {
        if wants.contains_key(identifier){
            //different for inner object
            let current_want = wants.get(identifier).unwrap();
            match current_want {
                Want::SingleField(_) => { 
                    resolved_fields.insert(identifier.to_string(), value); 
                },
                Want::ObjectProjection(fields, args) => {
                    //needs to call resolver to resolve want
                    let inner_resolver = resolver_map.resolvers.get(identifier).unwrap();
                    let context = context;
                    let inner_return_value = inner_resolver(Some(fields), args, resolver_map, context);
                    resolved_fields.insert(identifier.to_string(), return_value);
                },
                Want::Match(_) => {} //ignore match for now
            }
        }
    }
    let return_value = Value::Object(resolved_fields);
    return return_value
}