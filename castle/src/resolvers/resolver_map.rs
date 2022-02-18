use std::collections::HashMap;

use super::{resolver_type::Resolver, individual_resolvers::page_resolvers::page_info_resolver::page_info};

//A HashMap containing all Resolvers
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

    pub fn add_all_resolvers(&mut self, mut resolvers: HashMap<Box<str>, Resolver<C, R>>) {
        resolvers.insert("page_info".into(), page_info);
    }
}