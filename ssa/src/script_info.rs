use std::{collections::HashMap, fmt::Display};

use crate::{parser::Parser, value::Value};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ScriptInfo {
    properties: Vec<(String, Value)>,
}

impl ScriptInfo {
    pub fn add_property(&mut self, key: impl Into<String>, value: impl Into<Value>) {
        let key = key.into();
        let value = value.into();
        if let Some((_, v)) = self.properties.iter_mut().find(|(k, _)| k == &key) {
            *v = value;
        } else {
            self.properties.push((key, value));
        }
    }

    pub fn add_properties(&mut self, properties: HashMap<String, Value>) {
        for (key, value) in properties {
            self.add_property(key, value);
        }
    }

    pub fn remove_property(&mut self, key: impl AsRef<str>) {
        self.properties.retain(|(k, _)| k != key.as_ref());
    }

    pub fn get_property(&self, key: impl AsRef<str>) -> Option<&Value> {
        self.properties
            .iter()
            .find(|(k, _)| k == key.as_ref())
            .map(|(_, v)| v)
    }

    pub fn get_property_mut(&mut self, key: impl AsRef<str>) -> Option<&mut Value> {
        self.properties
            .iter_mut()
            .find(|(k, _)| k == key.as_ref())
            .map(|(_, v)| v)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &Value)> {
        self.properties.iter().map(|(k, v)| (k.as_str(), v))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&str, &mut Value)> {
        self.properties.iter_mut().map(|(k, v)| (k.as_str(), v))
    }

    pub fn clear(&mut self) {
        self.properties.clear();
    }

    pub fn add_comment(&mut self, comment: impl Into<String>) {
        let comments = self
            .get_property_mut(Key::Comment)
            .and_then(Value::as_list_mut);
        match comments {
            Some(comments) => {
                comments.push(Value::Str(comment.into()));
            }
            None => {
                self.add_property(
                    Key::Comment.to_string(),
                    Value::List(vec![Value::Str(comment.into())]),
                );
            }
        }
    }

    pub fn get_comments(&self) -> Option<&Vec<Value>> {
        self.get_property(Key::Comment)
            .and_then(Value::as_list)
    }

    pub fn get_comments_mut(&mut self) -> Option<&mut Vec<Value>> {
        self.get_property_mut(Key::Comment)
            .and_then(Value::as_list_mut)
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.add_property(Key::Title.to_string(), Value::Str(title.into()));
    }

    pub fn get_title(&self) -> Option<&str> {
        self.get_property(Key::Title)
            .and_then(Value::as_str)
    }

    pub fn get_title_mut(&mut self) -> Option<&mut String> {
        self.get_property_mut(Key::Title)
            .and_then(Value::as_str_mut)
    }

    pub fn set_original_script(&mut self, original_script: impl Into<String>) {
        self.add_property(
            Key::OriginalScript.to_string(),
            Value::Str(original_script.into()),
        );
    }

    pub fn get_original_script(&self) -> Option<&str> {
        self.get_property(Key::OriginalScript)
            .and_then(Value::as_str)
    }

    pub fn get_original_script_mut(&mut self) -> Option<&mut String> {
        self.get_property_mut(Key::OriginalScript)
            .and_then(Value::as_str_mut)
    }

    pub fn set_original_translation(&mut self, original_translation: impl Into<String>) {
        self.add_property(
            Key::OriginalTranslation.to_string(),
            Value::Str(original_translation.into()),
        );
    }

    pub fn get_original_translation(&self) -> Option<&str> {
        self.get_property(Key::OriginalTranslation)
            .and_then(Value::as_str)
    }

    pub fn get_original_translation_mut(&mut self) -> Option<&mut String> {
        self.get_property_mut(Key::OriginalTranslation)
            .and_then(Value::as_str_mut)
    }

    pub fn set_original_editing(&mut self, original_editing: impl Into<String>) {
        self.add_property(
            Key::OriginalEditing.to_string(),
            Value::Str(original_editing.into()),
        );
    }

    pub fn get_original_editing(&self) -> Option<&str> {
        self.get_property(Key::OriginalEditing)
            .and_then(Value::as_str)
    }

    pub fn get_original_editing_mut(&mut self) -> Option<&mut String> {
        self.get_property_mut(Key::OriginalEditing)
            .and_then(Value::as_str_mut)
    }

    pub fn set_original_timing(&mut self, original_timing: impl Into<String>) {
        self.add_property(
            Key::OriginalTiming.to_string(),
            Value::Str(original_timing.into()),
        );
    }

    pub fn get_original_timing(&self) -> Option<&str> {
        self.get_property(Key::OriginalTiming)
            .and_then(Value::as_str)
    }

    pub fn get_original_timing_mut(&mut self) -> Option<&mut String> {
        self.get_property_mut(Key::OriginalTiming)
            .and_then(Value::as_str_mut)
    }

    pub fn set_synch_point(&mut self, synch_point: impl Into<String>) {
        self.add_property(
            Key::SynchPoint.to_string(),
            Value::Str(synch_point.into()),
        );
    }

    pub fn get_synch_point(&self) -> Option<&str> {
        self.get_property(Key::SynchPoint)
            .and_then(Value::as_str)
    }

    pub fn get_synch_point_mut(&mut self) -> Option<&mut String> {
        self.get_property_mut(Key::SynchPoint)
            .and_then(Value::as_str_mut)
    }

    pub fn set_script_updated_by(&mut self, script_updated_by: impl Into<String>) {
        self.add_property(
            Key::ScriptUpdatedBy.to_string(),
            Value::Str(script_updated_by.into()),
        );
    }

    pub fn get_script_updated_by(&self) -> Option<&str> {
        self.get_property(Key::ScriptUpdatedBy)
            .and_then(Value::as_str)
    }

    pub fn get_script_updated_by_mut(&mut self) -> Option<&mut String> {
        self.get_property_mut(Key::ScriptUpdatedBy)
            .and_then(Value::as_str_mut)
    }

    pub fn set_update_details(&mut self, update_details: impl Into<String>) {
        self.add_property(
            Key::UpdateDetails.to_string(),
            Value::Str(update_details.into()),
        );
    }

    pub fn get_update_details(&self) -> Option<&str> {
        self.get_property(Key::UpdateDetails)
            .and_then(Value::as_str)
    }

    pub fn get_update_details_mut(&mut self) -> Option<&mut String> {
        self.get_property_mut(Key::UpdateDetails)
            .and_then(Value::as_str_mut)
    }

    pub fn set_script_type(&mut self, script_type: ScriptType) {
        self.add_property(
            Key::ScriptType.to_string(),
            Value::Str(script_type.to_string()),
        );
    }

    pub fn get_script_type(&self) -> Option<ScriptType> {
        self.get_property(Key::ScriptType)
            .and_then(Value::as_str)
            .and_then(|s| ScriptType::parse(s).ok())
    }

    pub fn set_collisions(&mut self, collisions: Collisions) {
        self.add_property(
            Key::Collisions.to_string(),
            Value::Str(collisions.to_string()),
        );
    }

    pub fn get_collisions(&self) -> Option<Collisions> {
        self.get_property(Key::Collisions)
            .and_then(Value::as_str)
            .and_then(|s| Collisions::parse(s).ok())
    }

    pub fn set_play_res_y(&mut self, play_res_y: i64) {
        self.add_property(Key::PlayResY.to_string(), Value::Int(play_res_y));
    }

    pub fn get_play_res_y(&self) -> Option<i64> {
        self.get_property(Key::PlayResY)
            .and_then(Value::as_int)
    }

    pub fn set_play_res_x(&mut self, play_res_x: i64) {
        self.add_property(Key::PlayResX.to_string(), Value::Int(play_res_x));
    }

    pub fn get_play_res_x(&self) -> Option<i64> {
        self.get_property(Key::PlayResX)
            .and_then(Value::as_int)
    }

    pub fn set_play_depth(&mut self, play_depth: i64) {
        self.add_property(Key::PlayDepth.to_string(), Value::Int(play_depth));
    }

    pub fn get_play_depth(&self) -> Option<i64> {
        self.get_property(Key::PlayDepth)
            .and_then(Value::as_int)
    }

    pub fn set_timer(&mut self, timer: f64) {
        self.add_property(Key::Timer.to_string(), Value::Float(timer));
    }

    pub fn get_timer(&self) -> Option<f64> {
        self.get_property(Key::Timer)
            .and_then(Value::as_float)
    }

    pub fn set_wrap_style(&mut self, wrap_style: i64) {
        self.add_property(Key::WrapStyle.to_string(), Value::Int(wrap_style));
    }

    pub fn get_wrap_style(&self) -> Option<i64> {
        self.get_property(Key::WrapStyle)
            .and_then(Value::as_int)
    }

    pub fn set_scaled_border_and_shadow(&mut self, scaled_border_and_shadow: bool) {
        self.add_property(
            Key::ScaledBorderAndShadow.to_string(),
            Value::Boolean(scaled_border_and_shadow),
        );
    }

    pub fn get_scaled_border_and_shadow(&self) -> Option<bool> {
        self.get_property(Key::ScaledBorderAndShadow)
            .and_then(Value::as_bool)
    }
}

impl Display for ScriptInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.properties {
            if key == Key::Comment.as_ref() {
                if let Some(comments) = value.as_list() {
                    for ele in comments {
                        writeln!(f, "; {}", ele)?;
                    }
                }
            } else if key == Key::ScaledBorderAndShadow.as_ref() {
                let value = if value.as_bool().unwrap_or_default() {
                    "yes"
                } else {
                    "no"
                };
                writeln!(f, "{}: {}", key, value)?;
            } else {
                writeln!(f, "{}: {}", key, value)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ScriptType {
    V4,
    V4Plus,
}

impl Display for ScriptType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScriptType::V4 => write!(f, "v4.00"),
            ScriptType::V4Plus => write!(f, "v4.00+"),
        }
    }
}

impl Parser for ScriptType {
    fn parse(s: &str) -> crate::Result<Self> {
        match s {
            "v4.00" => Ok(ScriptType::V4),
            "v4.00+" => Ok(ScriptType::V4Plus),
            _ => Err(crate::error::Error::parse_error::<ScriptType>(
                s.to_string(),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Collisions {
    Normal,
    Reverse,
}

impl Display for Collisions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Collisions::Normal => write!(f, "Normal"),
            Collisions::Reverse => write!(f, "Reverse"),
        }
    }
}

impl Parser for Collisions {
    fn parse(s: &str) -> crate::Result<Self> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(Collisions::Normal),
            "reverse" => Ok(Collisions::Reverse),
            _ => Err(crate::error::Error::parse_error::<Collisions>(
                s.to_string(),
            )),
        }
    }
}

#[derive(
    Debug, Copy, Clone, Hash, Eq, PartialEq, strum::EnumString, strum::Display, strum::AsRefStr,
)]
pub enum Key {
    #[strum(serialize = ";")]
    Comment,
    #[strum(serialize = "Title")]
    Title,
    OriginalScript,
    #[strum(serialize = "Original Translation")]
    OriginalTranslation,
    #[strum(serialize = "Original Editing")]
    OriginalEditing,
    #[strum(serialize = "Original Timing")]
    OriginalTiming,
    #[strum(serialize = "Synch Point")]
    SynchPoint,
    #[strum(serialize = "Script Updated By")]
    ScriptUpdatedBy,
    #[strum(serialize = "Update Details")]
    UpdateDetails,
    #[strum(serialize = "ScriptType")]
    ScriptType,
    #[strum(serialize = "Collisions")]
    Collisions,
    #[strum(serialize = "PlayResY")]
    PlayResY,
    #[strum(serialize = "PlayResX")]
    PlayResX,
    #[strum(serialize = "PlayDepth")]
    PlayDepth,
    #[strum(serialize = "Timer")]
    Timer,
    #[strum(serialize = "WrapStyle")]
    WrapStyle,
    #[strum(serialize = "ScaledBorderAndShadow")]
    ScaledBorderAndShadow,
}
