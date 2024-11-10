use std::{str::FromStr, time::Duration};

use crate::{
    error::Error,
    events::{effect::Effect, Event, EventFormat, EventType, Events},
    fonts::Fonts,
    graphics::Graphics,
    script_info::{Collisions, KeyProperty, ScriptInfo, ScriptType},
    styles::{Style, StyleFormat, V4Styles},
    value::Value,
};

pub trait Parser: Sized {
    fn parse(src: &str) -> crate::Result<Self>;
}

#[derive(Debug, Clone, Default)]
pub(crate) struct SsaParser {
    pub(crate) context: Context,
    pub(crate) script_info: ScriptInfo,
    pub(crate) styles: V4Styles,
    pub(crate) events: Events,
    pub(crate) fonts: Fonts,
    pub(crate) graphics: Graphics,
}

impl SsaParser {
    pub(crate) fn parse_script_info(&mut self, src: &str) -> crate::Result<()> {
        if src.starts_with(';') {
            if let Some(comment) = src.strip_prefix(";") {
                self.script_info.add_comment(comment.trim());
            } else if let Some(comment) = src.strip_prefix("!") {
                self.script_info.add_comment(comment.trim());
            }
            return Ok(());
        }
        if let Some(pos) = src.find(':') {
            let key = &src[..pos];
            let value = &src[pos + 1..];
            match KeyProperty::from_str(key) {
                Ok(key) => match key {
                    KeyProperty::Comment => {}
                    KeyProperty::Title
                    | KeyProperty::OriginalScript
                    | KeyProperty::OriginalTranslation
                    | KeyProperty::OriginalEditing
                    | KeyProperty::OriginalTiming
                    | KeyProperty::SynchPoint
                    | KeyProperty::ScriptUpdatedBy
                    | KeyProperty::UpdateDetails => {
                        self.script_info
                            .add_property(key.to_string(), Value::Str(value.trim().to_string()));
                    }
                    KeyProperty::ScriptType => {
                        self.script_info
                            .set_script_type(ScriptType::parse(value.trim())?);
                    }
                    KeyProperty::Collisions => {
                        self.script_info
                            .set_collisions(Collisions::parse(value.trim())?);
                    }
                    KeyProperty::PlayResY
                    | KeyProperty::PlayResX
                    | KeyProperty::PlayDepth
                    | KeyProperty::WrapStyle => {
                        let value = value
                            .trim()
                            .parse::<i64>()
                            .map_err(|error| Error::parse_int_error(error, value.trim()))?;
                        self.script_info.add_property(key.to_string(), value);
                    }
                    KeyProperty::Timer => {
                        let value = value
                            .trim()
                            .parse::<f64>()
                            .map_err(|error| Error::parse_float_error(error, value.trim()))?;
                        self.script_info.set_timer(value);
                    }
                    KeyProperty::ScaledBorderAndShadow => {
                        let value = if value.trim().to_ascii_lowercase() == "yes" {
                            true
                        } else {
                            false
                        };
                        self.script_info.set_scaled_border_and_shadow(value);
                    }
                },
                Err(_) => {
                    self.script_info
                        .add_property(key, Value::Str(value.trim().to_string()));
                }
            }
        }
        Ok(())
    }

    pub(crate) fn parse_styles(&mut self, src: &str) -> crate::Result<()> {
        if let Some(style_str) = src.strip_prefix("Style:").map(str::trim) {
            let mut style = Style::new(&self.styles);
            style_str
                .split(',')
                .zip(self.styles.order())
                .try_for_each(|(value, format)| {
                    match format {
                        StyleFormat::Name | StyleFormat::Fontname => {
                            style.set(*format, value.to_string());
                        }
                        StyleFormat::Fontsize => {
                            let value = value
                                .parse::<i64>()
                                .map_err(|error| Error::parse_int_error(error, value))?;
                            style.set(StyleFormat::Fontsize, value);
                        }
                        StyleFormat::PrimaryColour
                        | StyleFormat::SecondaryColour
                        | StyleFormat::TertiaryColour
                        | StyleFormat::OutlineColour
                        | StyleFormat::BackColour => {
                            style.set(*format, value.to_string());
                        }
                        StyleFormat::Bold
                        | StyleFormat::Italic
                        | StyleFormat::Underline
                        | StyleFormat::StrikeOut => {
                            let value = value
                                .parse::<i64>()
                                .map_err(|error| Error::parse_int_error(error, value))?;
                            style.set(*format, value);
                        }
                        StyleFormat::ScaleX
                        | StyleFormat::ScaleY
                        | StyleFormat::Spacing
                        | StyleFormat::Angle => {
                            let value = value
                                .parse::<f64>()
                                .map_err(|error| Error::parse_float_error(error, value))?;
                            style.set(*format, value);
                        }
                        StyleFormat::BorderStyle => {
                            let value = value
                                .parse::<i64>()
                                .map_err(|error| Error::parse_int_error(error, value))?;
                            style.set(*format, value);
                        }
                        StyleFormat::Outline => {
                            let value = value
                                .parse::<f64>()
                                .map_err(|error| Error::parse_float_error(error, value))?;
                            style.set(*format, value);
                        }
                        StyleFormat::Shadow
                        | StyleFormat::Alignment
                        | StyleFormat::MarginL
                        | StyleFormat::MarginR
                        | StyleFormat::MarginV => {
                            let value = value
                                .parse::<i64>()
                                .map_err(|error| Error::parse_int_error(error, value))?;
                            style.set(*format, value);
                        }
                        StyleFormat::AlphaLevel => {
                            let value = value
                                .parse::<f64>()
                                .map_err(|error| Error::parse_float_error(error, value))?;
                            style.set(*format, value);
                        }
                        StyleFormat::Encoding => {
                            let value = value
                                .parse::<i64>()
                                .map_err(|error| Error::parse_int_error(error, value))?;
                            style.set(*format, value);
                        }
                    }
                    Ok::<_, Error>(())
                })?;
            self.styles.add(style)?;
        }
        Ok(())
    }

    pub(crate) fn parse_events(&mut self, src: &str) -> crate::Result<()> {
        if let Some(pos) = src.find(':') {
            let event_type = EventType::from_str(&src[..pos]).map_err(|_| {
                Error::parse_error::<EventType>(format!("invalid event type {}", &src[..pos]))
            })?;
            let mut event = Event::new(event_type, &self.events);
            let event_data = &src[pos + 1..].trim();
            let n = self.events.order().len();
            event_data
                .splitn(n, ',')
                .zip(self.events.order())
                .try_for_each(|(value, format)| {
                    match format {
                        EventFormat::Layer => {
                            let value = value
                                .parse::<i64>()
                                .map_err(|error| Error::parse_int_error(error, value))?;
                            event.set(*format, value);
                        }
                        EventFormat::Marked => {
                            if let Some(pos) = value.find('=') {
                                let key = &value[..pos];
                                if key.to_ascii_lowercase() != "marked" {
                                    return Err(Error::parse_error::<EventFormat>(format!(
                                        "invalid marked key {}",
                                        key
                                    )));
                                }
                                let value = value[pos + 1..].parse::<i64>().map_err(|error| {
                                    Error::parse_int_error(error, &value[pos + 1..])
                                })?;
                                event.set(*format, value);
                            }
                        }
                        EventFormat::Start | EventFormat::End => {
                            let value = Duration::parse(value)?;
                            event.set(*format, value);
                        }
                        EventFormat::Style | EventFormat::Name => {
                            event.set(*format, value.to_string());
                        }
                        EventFormat::MarginL | EventFormat::MarginR | EventFormat::MarginV => {
                            let value = value
                                .parse::<i64>()
                                .map_err(|error| Error::parse_int_error(error, value))?;
                            event.set(*format, value);
                        }
                        EventFormat::Effect => {
                            if !value.is_empty() {
                                let effect = Effect::parse(value)?;
                                event.set(*format, effect);
                            }
                        }
                        EventFormat::Text => {
                            event.set(*format, value.to_string());
                        }
                    }
                    Ok::<_, Error>(())
                })?;
            self.events.push(event);
        }
        Ok(())
    }

    pub(crate) fn parse_fonts(&mut self, src: &str) -> crate::Result<()> {
        todo!("{}", src)
    }

    pub(crate) fn parse_graphics(&mut self, src: &str) -> crate::Result<()> {
        todo!("{}", src)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum Context {
    #[default]
    None,
    ParseScriptInfo,
    ParseStyles,
    ParseEvents,
    ParseFonts,
    ParseGraphics,
}
