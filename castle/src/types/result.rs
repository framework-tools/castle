use std::{collections::HashMap, fmt::Debug};

use crate::Value;


#[derive(Debug, PartialEq)]
pub struct CastleResult<Ctx: Debug, E: Debug> {
    pub data: HashMap<Box<str>, Value<Ctx, E>>,
    pub errors: Vec<E>
}