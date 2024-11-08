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

#[derive(Debug, Copy, Clone, Eq, PartialEq, strum::EnumString)]
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

#[derive(Debug, Clone, Default)]
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

    pub fn push(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn iter(&self) -> std::slice::Iter<Event> {
        self.events.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Event> {
        self.events.iter_mut()
    }
}
