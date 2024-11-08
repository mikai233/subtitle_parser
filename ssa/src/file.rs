use crate::{fonts::Fonts, graphics::Graphics};

use super::{events::Events, script_info::ScriptInfo, styles::V4Styles};

#[derive(Debug, Clone)]
pub struct File {
    pub script: ScriptInfo,
    pub styles: V4Styles,
    pub events: Events,
    pub fonts: Fonts,
    pub graphics: Graphics,
}

impl File {}
