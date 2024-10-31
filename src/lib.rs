use crate::error::Error;

pub mod ssa;
pub mod error;
pub mod parser;

pub type Result<T> = std::result::Result<T, Error>;
