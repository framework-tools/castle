use std::collections::HashMap;

use super::resolver_type::Resolver;

//A HashMap containing all Resolvers
#[derive(Debug, PartialEq)]
pub struct ResolverMap<C, R> {
    pub resolvers: HashMap<Box<str>, Resolver<C, R>>
}

impl<C, R> ResolverMap<C, R> {
    pub fn new() -> Self {
        ResolverMap {
            resolvers: HashMap::new()
        }
    }

    pub fn add_individual_resolver(&mut self, resolver_name: &str, resolver: Resolver<C, R>) {
        self.resolvers.insert(resolver_name.into(), resolver);
    }
}

