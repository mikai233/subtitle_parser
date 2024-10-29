use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Effect {
    Karaoke,
    ScrollUp {
        y1: i32,
        y2: i32,
        delay: u16,
        fadeawayheigh: Option<i32>,
    },
    ScrollDown {
        y1: i32,
        y2: i32,
        delay: u16,
        fadeawayheight: Option<i32>,
    },
    Banner {
        delay: u16,
        lefttoright: bool,
        fadeawayheight: Option<i32>,
    },
}

impl Display for Effect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Effect::Karaoke => {
                write!(f, "Karaoke")
            }
            Effect::ScrollUp { y1, y2, delay, fadeawayheigh } => {
                write!(f, "Scroll up;{y1};{y2};{delay}")?;
                if let Some(fadeawayheigh) = fadeawayheigh {
                    write!(f, ";{fadeawayheigh}")?;
                }
                Ok(())
            }
            Effect::ScrollDown { y1, y2, delay, fadeawayheight } => {
                write!(f, "Scroll down;{y1};{y2};{delay}")?;
                if let Some(fadeawayheight) = fadeawayheight {
                    write!(f, ";{fadeawayheight}")?;
                }
                Ok(())
            }
            Effect::Banner { delay, lefttoright, fadeawayheight } => {
                write!(f, "Banner;{delay};{lefttoright}")?;
                if let Some(fadeawayheight) = fadeawayheight {
                    write!(f, ";{fadeawayheight}")?;
                }
                Ok(())
            }
        }
    }
}