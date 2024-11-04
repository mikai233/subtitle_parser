use crate::error::Error;

pub mod error;
pub mod parser;
pub mod ssa;
pub mod value;

pub type Result<T> = std::result::Result<T, Error>;
