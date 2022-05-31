#![feature(if_let_guard)]

use castle_types::{Context, Value, Field};
use std::fmt::Debug;
use std::future::Future;

pub use crate::castle::Castle;
pub use anyhow::Error;

pub mod castle;
pub(crate) mod executor;
pub(crate) mod validation;

pub mod types {
    pub use castle_types::*;
}