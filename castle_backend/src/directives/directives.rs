use std::collections::HashMap;

use parser_and_schema::ast::syntax_definitions::{want::Want, argument::ArgumentOrTuple};
use shared::CastleError;

pub type DirectiveMap<C, T> = HashMap<Box<str>, Box<Directive<C, T>>>; 
pub type Directive<C, T> = dyn Fn(Wants, Args, C) -> Result<DirectiveOutput<T>, CastleError>;
pub type Wants = HashMap<Box<str>, Want>;
pub type Args = HashMap<Box<str>, ArgumentOrTuple>;
pub type DirectiveOutput<T> = Box<T>;