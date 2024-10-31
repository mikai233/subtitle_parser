use crate::ssa::events::dialogue::Dialogue;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct Comment(Dialogue);

impl Deref for Comment {
    type Target = Dialogue;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Comment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
