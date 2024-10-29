use std::any::type_name;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("parse to {ty} error, {msg}")]
    ParseError { ty: &'static str, msg: String },
    #[error("{msg}")]
    ParseIntError {
        #[source]
        error: std::num::ParseIntError,
        msg: String,
    },
}

impl Error {
    pub fn parse_error<Ty>(msg: impl Into<String>) -> Self {
        Error::ParseError {
            ty: type_name::<Ty>(),
            msg: msg.into(),
        }
    }

    pub fn parse_int_error(error: std::num::ParseIntError, msg: impl Into<String>) -> Self {
        Error::ParseIntError {
            error,
            msg: msg.into(),
        }
    }
}
