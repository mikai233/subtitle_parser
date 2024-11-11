use std::fmt::{Display, Formatter};

use crate::{error::Error, parser::Parser};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Effect {
    None,
    Unknown(String),
    Karaoke,
    ScrollUp {
        y1: i32,
        y2: i32,
        delay: i32,
        fadeawayheight: Option<i32>,
    },
    ScrollDown {
        y1: i32,
        y2: i32,
        delay: i32,
        fadeawayheight: Option<i32>,
    },
    Banner {
        delay: i32,
        lefttoright: bool,
        fadeawayheight: Option<i32>,
    },
}

impl Effect {
    fn parse_scroll_effect(parts: &[&str], direction: &str) -> crate::Result<Self> {
        if parts.len() < 4 {
            return Err(Error::parse_error::<Effect>(format!(
                "Invalid {} effect",
                direction
            )));
        }
        let y1 = parts[1]
            .parse::<i32>()
            .map_err(|_| Error::parse_error::<Effect>("Invalid y1"))?;
        let y2 = parts[2]
            .parse::<i32>()
            .map_err(|_| Error::parse_error::<Effect>("Invalid y2"))?;
        let delay = parts[3]
            .parse::<i32>()
            .map_err(|_| Error::parse_error::<Effect>("Invalid delay"))?;
        let fadeawayheight = if parts.len() > 4 {
            Some(
                parts[4]
                    .parse::<i32>()
                    .map_err(|_| Error::parse_error::<Effect>("Invalid fadeawayheight"))?,
            )
        } else {
            None
        };

        match direction {
            "Scroll up" => Ok(Effect::ScrollUp {
                y1,
                y2,
                delay,
                fadeawayheight,
            }),
            "Scroll down" => Ok(Effect::ScrollDown {
                y1,
                y2,
                delay,
                fadeawayheight,
            }),
            _ => Err(Error::parse_error::<Effect>("Unknown scroll direction")),
        }
    }

    fn parse_banner_effect(parts: &[&str]) -> crate::Result<Self> {
        if parts.len() < 3 {
            return Err(Error::parse_error::<Effect>("Invalid Banner effect"));
        }
        let delay = parts[1]
            .parse::<i32>()
            .map_err(|_| Error::parse_error::<Effect>("Invalid delay"))?;
        let lefttoright = parts[2]
            .parse::<i32>()
            .map_err(|_| Error::parse_error::<Effect>("Invalid lefttoright"))?
            == 1;
        let fadeawayheight = if parts.len() > 3 {
            Some(
                parts[3]
                    .parse::<i32>()
                    .map_err(|_| Error::parse_error::<Effect>("Invalid fadeawayheight"))?,
            )
        } else {
            None
        };

        Ok(Effect::Banner {
            delay,
            lefttoright,
            fadeawayheight,
        })
    }
}

impl Display for Effect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Effect::None => {
                write!(f, "")
            }
            Effect::Unknown(s) => {
                write!(f, "{}", s)
            }
            Effect::Karaoke => {
                write!(f, "Karaoke")
            }
            Effect::ScrollUp {
                y1,
                y2,
                delay,
                fadeawayheight,
            } => {
                write!(f, "Scroll up;{y1};{y2};{delay}")?;
                if let Some(fadeawayheight) = fadeawayheight {
                    write!(f, ";{fadeawayheight}")?;
                }
                Ok(())
            }
            Effect::ScrollDown {
                y1,
                y2,
                delay,
                fadeawayheight,
            } => {
                write!(f, "Scroll down;{y1};{y2};{delay}")?;
                if let Some(fadeawayheight) = fadeawayheight {
                    write!(f, ";{fadeawayheight}")?;
                }
                Ok(())
            }
            Effect::Banner {
                delay,
                lefttoright,
                fadeawayheight,
            } => {
                write!(f, "Banner;{delay};{}", if *lefttoright { 1 } else { 0 })?;
                if let Some(fadeawayheight) = fadeawayheight {
                    write!(f, ";{fadeawayheight}")?;
                }
                Ok(())
            }
        }
    }
}

impl Parser for Effect {
    fn parse(src: &str) -> crate::Result<Self> {
        let parts: Vec<&str> = src.split(';').collect();
        let effect = parts.get(0).map(|s| s.to_ascii_lowercase());
        match effect.as_ref().map(|s| s.as_str()) {
            Some("karaoke") => Ok(Effect::Karaoke),
            Some("scroll up") => Effect::parse_scroll_effect(&parts, "Scroll up"),
            Some("scroll down") => Effect::parse_scroll_effect(&parts, "Scroll down"),
            Some("banner") => Effect::parse_banner_effect(&parts),
            _ => Ok(Effect::Unknown(src.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_karaoke_effect() {
        let src = "Karaoke";
        let effect = Effect::parse(src).unwrap();
        assert_eq!(effect, Effect::Karaoke);
    }

    #[test]
    fn test_parse_scroll_up_effect() {
        let src = "Scroll up;100;200;100";
        let effect = Effect::parse(src).unwrap();
        assert_eq!(
            effect,
            Effect::ScrollUp {
                y1: 100,
                y2: 200,
                delay: 100,
                fadeawayheight: None
            }
        );
    }

    #[test]
    fn test_parse_scroll_down_effect() {
        let src = "Scroll down;200;300;100";
        let effect = Effect::parse(src).unwrap();
        assert_eq!(
            effect,
            Effect::ScrollDown {
                y1: 200,
                y2: 300,
                delay: 100,
                fadeawayheight: None
            }
        );
    }

    #[test]
    fn test_parse_banner_effect() {
        let src = "Banner;100;1;50";
        let effect = Effect::parse(src).unwrap();
        assert_eq!(
            effect,
            Effect::Banner {
                delay: 100,
                lefttoright: true,
                fadeawayheight: Some(50)
            }
        );
    }

    #[test]
    fn test_parse_unknown_effect() {
        let src = "UnknownEffect;data";
        let effect = Effect::parse(src).unwrap();
        assert_eq!(effect, Effect::Unknown(src.to_string()));
    }
}
