use comment::Comment;
use dialogue::Dialogue;

pub mod comment;
pub mod dialogue;
pub mod effect;
pub mod text;

#[derive(Debug, Clone)]
pub enum EventFormat {
    Layer,
    Start,
    End,
    Style,
    Name,
    MarginL,
    MarginR,
    MarginV,
    Effect,
    Text,
}

#[derive(Debug, Clone)]
pub enum Events {
    Dialogue(Dialogue),
    Comment(Comment),
    Picture,
    Sound,
    End,
    Movie,
    Command,
}
