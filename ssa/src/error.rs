use std::any::type_name;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown ssa file format version `{0}`")]
    UnknownSSAVersion(String),
    #[error("parse to {ty} error, {msg}")]
    ParseError { ty: &'static str, msg: String },
    #[error("{msg}")]
    ParseIntError {
        #[source]
        error: std::num::ParseIntError,
        msg: String,
    },
    #[error("{msg}")]
    ParseFloatError {
        #[source]
        error: std::num::ParseFloatError,
        msg: String,
    },
    #[error("v4 style name not found")]
    V4StyleNameNotFound,
    #[error("invalid type, expected {expected}")]
    InvalidType { expected: String },
    #[error("invalid utf-8 encoding")]
    InvalidUTF8Encoding,
    #[error("io error")]
    IoError {
        #[from]
        source: std::io::Error,
    },
    #[error("fmt error")]
    FmtError {
        #[from]
        source: std::fmt::Error,
    }
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

    pub fn parse_float_error(error: std::num::ParseFloatError, msg: impl Into<String>) -> Self {
        Error::ParseFloatError {
            error,
            msg: msg.into(),
        }
    }

    pub fn unknown_version(version: impl Into<String>) -> Self {
        Error::UnknownSSAVersion(version.into())
    }

    pub fn invalid_type(expected: impl Into<String>) -> Self {
        Error::InvalidType {
            expected: expected.into(),
        }
    }
}
