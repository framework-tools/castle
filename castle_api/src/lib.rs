#![feature(if_let_guard)]


pub use crate::castle::Castle;
pub use anyhow::Error;

pub mod castle;
pub(crate) mod executor;
pub(crate) mod validation;
pub use castle_macro::castle as castle_macro;
pub use async_trait::async_trait;

pub mod types {
    pub use castle_types::*;
}