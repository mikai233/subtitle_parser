use std::{fmt::Display, time::Duration};

use effect::Effect;
use itertools::Itertools;

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
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, strum::EnumString, strum::Display)]
#[strum(ascii_case_insensitive)]
pub enum EventType {
    Dialogue,
    Comment,
    Picture,
    Sound,
    Movie,
    Command,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Events {
    order: Vec<EventFormat>,
    events: Vec<Event>,
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

    pub fn add(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn iter(&self) -> std::slice::Iter<Event> {
        self.events.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Event> {
        self.events.iter_mut()
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
