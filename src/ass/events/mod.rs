use text::Text;

use crate::ass::events::comment::Comment;
use crate::ass::events::dialogue::Dialogue;
use crate::ass::events::effect::Effect;
use crate::ass::format_duration;
use std::fmt::{Display, Formatter};
use std::time::Duration;

pub mod comment;
pub mod dialogue;
pub mod effect;
pub mod text;

#[derive(Debug)]
pub enum Events {
    Dialogue(Dialogue),
    Comment(Comment),
    Picture,
    Sound,
    End,
    Movie,
    Command,
}

#[derive(Debug, Clone)]
pub struct EventsFormat {
    pub is_ass: bool,
    pub marked: i32,
    pub layer: i32,
    pub start: Duration,
    pub end: Duration,
    pub style: String,
    pub name: String,
    pub margin_l: i32,
    pub margin_r: i32,
    pub margin_v: i32,
    pub effect: Option<Effect>,
    pub text: Text,
}

impl Display for EventsFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let marked_or_layer = if self.is_ass { self.layer } else { self.marked };
        let effect = match &self.effect {
            None => "".to_owned(),
            Some(effect) => effect.to_string(),
        };
        write!(
            f,
            "{},{},{},{},{},{},{},{},{},{}",
            marked_or_layer,
            format_duration(self.start),
            format_duration(self.end),
            self.style,
            self.name,
            self.margin_l,
            self.margin_r,
            self.margin_v,
            effect,
            self.text
        )
    }
}
