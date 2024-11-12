use itertools::Itertools;

use crate::{
    events::{effect::Effect, text::Text},
    format_duration,
};
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Boolean(bool),
    List(Vec<Value>),
    Duration(Duration),
    Effect(Effect),
    Text(Text),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::List(a), Value::List(b)) => a == b,
            (Value::Duration(a), Value::Duration(b)) => a == b,
            (Value::Effect(a), Value::Effect(b)) => a == b,
            (Value::Text(a), Value::Text(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Str(s) => write!(f, "{}", s),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::List(l) => {
                write!(f, "[")?;
                write!(f, "{}", l.iter().join(", "))?;
                write!(f, "]")
            }
            Value::Duration(duration) => write!(f, "{}", format_duration(duration)),
            Value::Effect(effect) => write!(f, "{}", effect),
            Value::Text(text) => write!(f, "{}", text),
        }
    }
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_str_mut(&mut self) -> Option<&mut String> {
        match self {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_int_mut(&mut self) -> Option<&mut i64> {
        match self {
            Value::Int(i) => Some(i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_float_mut(&mut self) -> Option<&mut f64> {
        match self {
            Value::Float(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_bool_mut(&mut self) -> Option<&mut bool> {
        match self {
            Value::Boolean(b) => Some(b),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&Vec<Value>> {
        match self {
            Value::List(l) => Some(l),
            _ => None,
        }
    }

    pub fn as_list_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Value::List(l) => Some(l),
            _ => None,
        }
    }

    pub fn as_duration(&self) -> Option<Duration> {
        match self {
            Value::Duration(d) => Some(*d),
            _ => None,
        }
    }

    pub fn as_duration_mut(&mut self) -> Option<&mut Duration> {
        match self {
            Value::Duration(d) => Some(d),
            _ => None,
        }
    }

    pub fn as_effect(&self) -> Option<&Effect> {
        match self {
            Value::Effect(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_effect_mut(&mut self) -> Option<&mut Effect> {
        match self {
            Value::Effect(e) => Some(e),
            _ => None,
        }
    }
}

impl Into<Value> for String {
    fn into(self) -> Value {
        Value::Str(self)
    }
}

impl Into<Value> for &str {
    fn into(self) -> Value {
        Value::Str(self.to_string())
    }
}

impl Into<Value> for i64 {
    fn into(self) -> Value {
        Value::Int(self)
    }
}

impl Into<Value> for f64 {
    fn into(self) -> Value {
        Value::Float(self)
    }
}

impl Into<Value> for bool {
    fn into(self) -> Value {
        Value::Boolean(self)
    }
}

impl Into<Value> for Vec<Value> {
    fn into(self) -> Value {
        Value::List(self)
    }
}

impl Into<Value> for Duration {
    fn into(self) -> Value {
        Value::Duration(self)
    }
}

impl Into<Value> for Effect {
    fn into(self) -> Value {
        Value::Effect(self)
    }
}

impl Into<Value> for Text {
    fn into(self) -> Value {
        Value::Text(self)
    }
}
