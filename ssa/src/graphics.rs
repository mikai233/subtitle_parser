use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct Graphics {
    pub graphics: Vec<String>,
}

impl Deref for Graphics {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.graphics
    }
}

impl DerefMut for Graphics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.graphics
    }
}
