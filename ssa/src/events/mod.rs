use effect::Effect;
use itertools::Itertools;
use std::ops::{Deref, DerefMut};
use std::{fmt::Display, time::Duration};
use text::Text;

use crate::error::Error;
use crate::parser::{parse_i64, Parser};
use crate::value::Value;

pub mod effect;
pub mod text;

#[derive(
    Debug, Copy, Clone, Eq, PartialEq, strum::EnumString, strum::Display, strum::VariantNames,
)]
#[strum(ascii_case_insensitive)]
pub enum EventFormat {
    Layer,
    Marked,
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

impl EventFormat {
    pub fn default_value(&self) -> Value {
        match self {
            EventFormat::Layer => 0.into(),
            EventFormat::Marked => "Marked=0".to_owned().into(),
            EventFormat::Start | EventFormat::End => Duration::default().into(),
            EventFormat::Style | EventFormat::Name => "".to_owned().into(),
            EventFormat::MarginL | EventFormat::MarginR | EventFormat::MarginV => 10.into(),
            EventFormat::Effect => Effect::None.into(),
            EventFormat::Text => "".to_owned().into(),
        }
    }

    pub fn parse_value(&self, src: &str) -> crate::Result<Value> {
        let value = match self {
            EventFormat::Layer => parse_i64(src)?,
            EventFormat::Marked => {
                let pos = src.find('=').ok_or(Error::ParseError {
                    ty: "Marked",
                    msg: format!("Invalid Marked value: {}", src),
                })?;
                let key = &src[..pos];
                if key.to_ascii_lowercase() != "marked" {
                    return Err(Error::ParseError {
                        ty: "Marked",
                        msg: format!("Invalid Marked key: {}", key),
                    });
                }
                parse_i64(&src[pos + 1..])?
            }
            EventFormat::Start | EventFormat::End => {
                let value = Duration::parse(src)?;
                value.into()
            }
            EventFormat::Style | EventFormat::Name => src.to_owned().into(),
            EventFormat::MarginL | EventFormat::MarginR | EventFormat::MarginV => parse_i64(src)?,
            EventFormat::Effect => {
                let value = Effect::parse(src)?;
                value.into()
            }
            EventFormat::Text => {
                let value = Text::parse(src)?;
                value.into()
            }
        };
        Ok(value)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, strum::EnumString, strum::Display)]
#[strum(ascii_case_insensitive)]
pub enum EventType {
    Dialogue,
    Comment,
    Picture,
    Sound,
    Movie,
    Command,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Event {
    event_type: EventType,
    values: Vec<(EventFormat, Option<Value>)>,
}

impl Event {
    pub fn new(event_type: EventType, events: &Events) -> Self {
        let mut values = vec![];
        for format in events.order() {
            values.push((format.clone(), None));
        }
        Self { event_type, values }
    }

    pub fn set(&mut self, format: EventFormat, value: impl Into<Value>) {
        for (f, v) in self.values.iter_mut() {
            if f == &format {
                *v = Some(value.into());
                return;
            }
        }
    }

    pub fn get(&self, format: EventFormat) -> Option<&Value> {
        for (f, v) in self.values.iter() {
            if f == &format {
                return v.as_ref();
            }
        }
        None
    }

    pub fn get_mut(&mut self, format: EventFormat) -> Option<&mut Value> {
        for (f, v) in self.values.iter_mut() {
            if f == &format {
                return v.as_mut();
            }
        }
        None
    }

    pub fn remove(&mut self, format: EventFormat) {
        for (f, v) in self.values.iter_mut() {
            if f == &format {
                *v = None;
                return;
            }
        }
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut values = Vec::with_capacity(self.values.len());
        for (format, ele) in &self.values {
            match ele {
                Some(ele) => {
                    values.push(ele.to_string());
                }
                None => {
                    values.push(format.default_value().to_string());
                }
            }
        }
        write!(f, "{}: {}", self.event_type, values.join(","))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Events {
    order: Vec<EventFormat>,
    pub events: Vec<Event>,
}

impl Events {
    pub fn new(order: Vec<EventFormat>) -> Self {
        Self {
            order,
            events: vec![],
        }
    }

    pub fn order(&self) -> &Vec<EventFormat> {
        &self.order
    }
}

impl Display for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Format: {}", self.order.iter().join(", "))?;
        for event in &self.events {
            writeln!(f, "{}", event)?;
        }
        Ok(())
    }
}

impl Deref for Events {
    type Target = Vec<Event>;

    fn deref(&self) -> &Self::Target {
        &self.events
    }
}

impl DerefMut for Events {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.events
    }
}

impl Default for Events {
    fn default() -> Self {
        Self {
            order: vec![
                EventFormat::Layer,
                EventFormat::Start,
                EventFormat::End,
                EventFormat::Style,
                EventFormat::Name,
                EventFormat::MarginL,
                EventFormat::MarginR,
                EventFormat::MarginV,
                EventFormat::Effect,
                EventFormat::Text,
            ],
            events: Default::default(),
        }
    }
}
