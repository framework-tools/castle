use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::{want::Want, argument::ArgumentOrTuple};

use crate::resolvers::resolvers::Resolver;

pub type DirectiveMap<C, T> = HashMap<Box<str>, Resolver<C, T>>; 
pub type Wants = HashMap<Box<str>, Want>;
pub type Args = HashMap<Box<str>, ArgumentOrTuple>;
pub type DirectiveOutput<T> = Box<T>;