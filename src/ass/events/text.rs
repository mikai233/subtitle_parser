use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Text(String);

impl Text {}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
