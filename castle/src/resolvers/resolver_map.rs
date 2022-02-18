use std::collections::HashMap;

use super::{resolver_type::Resolver, individual_resolvers::page_resolvers::{page_info_resolver::page_info, basic_page_info::basic_page_info}};

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
}

pub fn create_resolver_map_with_all_resolvers() -> HashMap<Box<str>, Resolver<String, String>> {
    let mut resolver_map = HashMap::new();
    resolver_map.insert("page_info".into(), page_info);
    resolver_map.insert("basic_page_info".into(), basic_page_info);
    return resolver_map
}