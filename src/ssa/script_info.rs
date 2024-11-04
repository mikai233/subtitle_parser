use std::{
    collections::{btree_map::Entry, BTreeMap},
    fmt::Display,
};

use crate::{parser::Parser, value::Value};

#[derive(Debug, Clone, Default)]
pub struct ScriptInfo {
    pub properties: BTreeMap<String, Value>,
}

impl ScriptInfo {
    pub fn add_property(&mut self, key: impl Into<String>, value: impl Into<Value>) {
        self.properties.insert(key.into(), value.into());
    }

    pub fn add_properties(&mut self, properties: BTreeMap<String, Value>) {
        self.properties.extend(properties);
    }

    pub fn get_property(&self, key: impl AsRef<str>) -> Option<&Value> {
        self.properties.get(key.as_ref())
    }

    pub fn add_comment(&mut self, comment: impl Into<String>) {
        match self.properties.entry(KeyProperty::Comment.to_string()) {
            Entry::Occupied(mut occupied_entry) => {
                occupied_entry
                    .get_mut()
                    .as_list_mut()
                    .unwrap()
                    .push(Value::Str(comment.into()));
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(Value::List(vec![Value::Str(comment.into())]));
            }
        }
    }

    pub fn get_comments(&self) -> Option<&Vec<Value>> {
        self.properties
            .get(KeyProperty::Comment.as_ref())
            .and_then(Value::as_list)
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.properties
            .insert(KeyProperty::Title.to_string(), Value::Str(title.into()));
    }

    pub fn get_title(&self) -> Option<&str> {
        self.properties
            .get(KeyProperty::Title.as_ref())
            .and_then(Value::as_str)
    }

    pub fn set_original_script(&mut self, original_script: impl Into<String>) {
        self.properties.insert(
            KeyProperty::OriginalScript.to_string(),
            Value::Str(original_script.into()),
        );
    }

    pub fn get_original_script(&self) -> Option<&str> {
        self.properties
            .get(KeyProperty::OriginalScript.as_ref())
            .and_then(Value::as_str)
    }

    pub fn set_original_translation(&mut self, original_translation: impl Into<String>) {
        self.properties.insert(
            KeyProperty::OriginalTranslation.to_string(),
            Value::Str(original_translation.into()),
        );
    }

    pub fn get_original_translation(&self) -> Option<&str> {
        self.properties
            .get(KeyProperty::OriginalTranslation.as_ref())
            .and_then(Value::as_str)
    }

    pub fn set_original_editing(&mut self, original_editing: impl Into<String>) {
        self.properties.insert(
            KeyProperty::OriginalEditing.to_string(),
            Value::Str(original_editing.into()),
        );
    }

    pub fn get_original_editing(&self) -> Option<&str> {
        self.properties
            .get(KeyProperty::OriginalEditing.as_ref())
            .and_then(Value::as_str)
    }

    pub fn set_original_timing(&mut self, original_timing: impl Into<String>) {
        self.properties.insert(
            KeyProperty::OriginalTiming.to_string(),
            Value::Str(original_timing.into()),
        );
    }

    pub fn get_original_timing(&self) -> Option<&str> {
        self.properties
            .get(KeyProperty::OriginalTiming.as_ref())
            .and_then(Value::as_str)
    }

    pub fn set_synch_point(&mut self, synch_point: impl Into<String>) {
        self.properties.insert(
            KeyProperty::SynchPoint.to_string(),
            Value::Str(synch_point.into()),
        );
    }

    pub fn get_synch_point(&self) -> Option<&str> {
        self.properties
            .get(KeyProperty::SynchPoint.as_ref())
            .and_then(Value::as_str)
    }

    pub fn set_script_updated_by(&mut self, script_updated_by: impl Into<String>) {
        self.properties.insert(
            KeyProperty::ScriptUpdatedBy.to_string(),
            Value::Str(script_updated_by.into()),
        );
    }

    pub fn get_script_updated_by(&self) -> Option<&str> {
        self.properties
            .get(KeyProperty::ScriptUpdatedBy.as_ref())
            .and_then(Value::as_str)
    }

    pub fn set_update_details(&mut self, update_details: impl Into<String>) {
        self.properties.insert(
            KeyProperty::UpdateDetails.to_string(),
            Value::Str(update_details.into()),
        );
    }

    pub fn get_update_details(&self) -> Option<&str> {
        self.properties
            .get(KeyProperty::UpdateDetails.as_ref())
            .and_then(Value::as_str)
    }

    pub fn set_script_type(&mut self, script_type: ScriptType) {
        self.properties.insert(
            KeyProperty::ScriptType.to_string(),
            Value::Str(script_type.to_string()),
        );
    }

    pub fn get_script_type(&self) -> Option<ScriptType> {
        self.properties
            .get(KeyProperty::ScriptType.as_ref())
            .and_then(Value::as_str)
            .and_then(|s| ScriptType::parse(s).ok())
    }

    pub fn set_collisions(&mut self, collisions: Collisions) {
        self.properties.insert(
            KeyProperty::Collisions.to_string(),
            Value::Str(collisions.to_string()),
        );
    }

    pub fn get_collisions(&self) -> Option<Collisions> {
        self.properties
            .get(KeyProperty::Collisions.as_ref())
            .and_then(Value::as_str)
            .and_then(|s| Collisions::parse(s).ok())
    }

    pub fn set_play_res_y(&mut self, play_res_y: i64) {
        self.properties
            .insert(KeyProperty::PlayResY.to_string(), Value::Int(play_res_y));
    }

    pub fn get_play_res_y(&self) -> Option<i64> {
        self.properties
            .get(KeyProperty::PlayResY.as_ref())
            .and_then(Value::as_int)
    }

    pub fn set_play_res_x(&mut self, play_res_x: i64) {
        self.properties
            .insert(KeyProperty::PlayResX.to_string(), Value::Int(play_res_x));
    }

    pub fn get_play_res_x(&self) -> Option<i64> {
        self.properties
            .get(KeyProperty::PlayResX.as_ref())
            .and_then(Value::as_int)
    }

    pub fn set_play_depth(&mut self, play_depth: i64) {
        self.properties
            .insert(KeyProperty::PlayDepth.to_string(), Value::Int(play_depth));
    }

    pub fn get_play_depth(&self) -> Option<i64> {
        self.properties
            .get(KeyProperty::PlayDepth.as_ref())
            .and_then(Value::as_int)
    }

    pub fn set_timer(&mut self, timer: f64) {
        self.properties
            .insert(KeyProperty::Timer.to_string(), Value::Float(timer));
    }

    pub fn get_timer(&self) -> Option<f64> {
        self.properties
            .get(KeyProperty::Timer.as_ref())
            .and_then(Value::as_float)
    }

    pub fn set_wrap_style(&mut self, wrap_style: i64) {
        self.properties
            .insert(KeyProperty::WrapStyle.to_string(), Value::Int(wrap_style));
    }

    pub fn get_wrap_style(&self) -> Option<i64> {
        self.properties
            .get(KeyProperty::WrapStyle.as_ref())
            .and_then(Value::as_int)
    }

    pub fn set_scaled_border_and_shadow(&mut self, scaled_border_and_shadow: bool) {
        self.properties.insert(
            KeyProperty::ScaledBorderAndShadow.to_string(),
            Value::Boolean(scaled_border_and_shadow),
        );
    }

    pub fn get_scaled_border_and_shadow(&self) -> Option<bool> {
        self.properties
            .get(KeyProperty::ScaledBorderAndShadow.as_ref())
            .and_then(Value::as_bool)
    }
}

impl Display for ScriptInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.properties {
            if key == KeyProperty::ScaledBorderAndShadow.as_ref() {
                let value = if value.as_bool().unwrap_or_default() {
                    "Yes"
                } else {
                    "No"
                };
                writeln!(f, "{}: {}", key, value)?;
            }
            writeln!(f, "{}: {}", key, value)?;
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
        match s {
            "Normal" => Ok(Collisions::Normal),
            "Reverse" => Ok(Collisions::Reverse),
            _ => Err(crate::error::Error::parse_error::<Collisions>(
                s.to_string(),
            )),
        }
    }
}

#[derive(
    Debug, Copy, Clone, Hash, Eq, PartialEq, strum::EnumString, strum::Display, strum::AsRefStr,
)]
pub enum KeyProperty {
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
