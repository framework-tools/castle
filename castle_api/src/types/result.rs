use std::{collections::HashMap};

use crate::Value;


#[derive(Debug, PartialEq)]
pub struct CastleResult<Ctx, E> {
    pub data: HashMap<Box<str>, Value<Ctx, E>>,
    pub errors: Vec<E>
}