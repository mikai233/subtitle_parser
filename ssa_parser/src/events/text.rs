use std::fmt::Display;

use crate::parser::Parser;

#[derive(Debug, Clone, Hash)]
pub struct Text(String);

impl Text {}

impl Parser for Text {
    fn parse(src: &str) -> crate::Result<Self> {
        Ok(Self(src.to_string()))
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for Text {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Text {}
