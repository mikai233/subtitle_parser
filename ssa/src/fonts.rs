use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct Fonts {
    pub fonts: Vec<String>,
}

impl Deref for Fonts {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.fonts
    }
}

impl DerefMut for Fonts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fonts
    }
}
