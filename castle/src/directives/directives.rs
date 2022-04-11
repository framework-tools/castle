use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::{want::Want, argument::ArgumentOrTuple, directive_definition::DirectiveDefinition};

use crate::resolvers::resolver_type::Resolver;

pub type DirectiveMap<C, R> = HashMap<Box<str>, Resolver<C, R>>; 
pub type Wants = HashMap<Box<str>, Want>;
pub type DirectiveOutput<R> = Box<R>;