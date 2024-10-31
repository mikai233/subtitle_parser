use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct ScriptInfo {
    pub comments: Vec<String>,
    pub title: Option<String>,
    pub original_script: Option<String>,
    pub original_translation: Option<String>,
    pub original_editing: Option<String>,
    pub original_timing: Option<String>,
    pub synch_point: Option<String>,
    pub script_updated_by: Option<String>,
    pub update_details: Option<String>,
    pub script_type: ScriptType,
    pub collisions: Collisions,
    pub play_res_y: i32,
    pub play_res_x: i32,
    pub play_depth: i32,
    pub timer: f64,
    pub wrap_style: i32,
    pub scaled_border_and_shadow: bool,
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
