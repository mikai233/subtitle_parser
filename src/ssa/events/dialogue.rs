use std::ops::{Deref, DerefMut};

use super::EventFormat;

#[derive(Debug, Clone)]
pub struct Dialogue(EventFormat);

impl Deref for Dialogue {
    type Target = EventFormat;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Dialogue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
