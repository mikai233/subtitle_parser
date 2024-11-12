use std::str::FromStr;

use crate::{
    error::Error,
    events::{Event, EventType, Events},
    fonts::Fonts,
    graphics::Graphics,
    script_info::{Collisions, Key, ScriptInfo, ScriptType},
    styles::{Style, V4Styles},
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
            match Key::from_str(key) {
                Ok(key) => match key {
                    Key::Comment => {}
                    Key::Title
                    | Key::OriginalScript
                    | Key::OriginalTranslation
                    | Key::OriginalEditing
                    | Key::OriginalTiming
                    | Key::SynchPoint
                    | Key::ScriptUpdatedBy
                    | Key::UpdateDetails => {
                        self.script_info
                            .add_property(key.to_string(), Value::Str(value.trim().to_string()));
                    }
                    Key::ScriptType => {
                        self.script_info
                            .set_script_type(ScriptType::parse(value.trim())?);
                    }
                    Key::Collisions => {
                        self.script_info
                            .set_collisions(Collisions::parse(value.trim())?);
                    }
                    Key::PlayResY | Key::PlayResX | Key::PlayDepth | Key::WrapStyle => {
                        let value = value
                            .trim()
                            .parse::<i64>()
                            .map_err(|error| Error::parse_int_error(error, value.trim()))?;
                        self.script_info.add_property(key.to_string(), value);
                    }
                    Key::Timer => {
                        let value = value
                            .trim()
                            .parse::<f64>()
                            .map_err(|error| Error::parse_float_error(error, value.trim()))?;
                        self.script_info.set_timer(value);
                    }
                    Key::ScaledBorderAndShadow => {
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
                    let value = format.parse_value(value)?;
                    style.set(*format, value);
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
                    let value = format.parse_value(value)?;
                    event.set(*format, value);
                    Ok::<_, Error>(())
                })?;
            self.events.push(event);
        }
        Ok(())
    }

    pub(crate) fn parse_fonts(&mut self, src: &str) -> crate::Result<()> {
        if let Some(font) = src.strip_prefix("fontname:").map(str::trim) {
            self.fonts.push(font.to_string());
        }
        Ok(())
    }

    pub(crate) fn parse_graphics(&mut self, src: &str) -> crate::Result<()> {
        if let Some(file) = src.strip_prefix("filename:").map(str::trim) {
            self.graphics.push(file.to_string());
        }
        Ok(())
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

pub fn parse_i64(src: &str) -> crate::Result<Value> {
    let value = src
        .parse::<i64>()
        .map_err(|error| Error::parse_int_error(error, src))?;
    Ok(value.into())
}

pub fn parse_f64(src: &str) -> crate::Result<Value> {
    let value = src
        .parse::<f64>()
        .map_err(|error| Error::parse_float_error(error, src))?;
    Ok(value.into())
}

#[cfg(test)]
mod tests {
    use super::SsaParser;
    use crate::file::File;
    use crate::{events::EventFormat, styles::StyleFormat, value::Value};

    #[test]
    fn test_parse_script_info() -> crate::Result<()> {
        let mut parser = SsaParser::default();
        parser.parse_script_info("; comment1")?;
        parser.parse_script_info(";comment2")?;
        parser.parse_script_info("Title: Title1")?;
        parser.parse_script_info("Custom: xxxxx")?;
        assert_eq!(
            parser.script_info.get_comments().unwrap(),
            &[
                Value::Str("comment1".to_string()),
                Value::Str("comment2".to_string())
            ]
        );
        assert_eq!(parser.script_info.get_title().unwrap(), "Title1");
        assert_eq!(
            parser.script_info.get_property("Custom").unwrap(),
            &Value::Str("xxxxx".to_string())
        );
        Ok(())
    }

    #[test]
    fn test_parse_styles() -> crate::Result<()> {
        let mut parser = SsaParser::default();
        parser.parse_styles("Style: Default,Arial,20,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,2,2,10,10,10,1")?;
        parser.parse_styles("Style: OPCN,华康方圆体W7,50,&H00FFFFFF,&H00000000,&H00DDAB2F,&H00000000,-1,0,0,0,100,100,0.1,0,1,4,0,8,80,80,30,1")?;
        let style = parser.styles.get_mut("OPCN").unwrap();
        assert_eq!(
            style
                .get(StyleFormat::Name)
                .and_then(Value::as_str)
                .unwrap(),
            "OPCN"
        );
        assert_eq!(
            style
                .get(StyleFormat::Fontname)
                .and_then(Value::as_str)
                .unwrap(),
            "华康方圆体W7"
        );
        style.remove(StyleFormat::Fontname);
        assert!(style.get(StyleFormat::Fontname).is_none());
        style.set(StyleFormat::Fontname, "Arial");
        assert_eq!(
            style
                .get(StyleFormat::Fontname)
                .and_then(Value::as_str)
                .unwrap(),
            "Arial"
        );
        Ok(())
    }

    #[test]
    fn test_parse_events() -> crate::Result<()> {
        let mut parser = SsaParser::default();
        parser.parse_events("Dialogue: 0,0:00:00.00,0:00:05.00,Default,,0,0,0,,Hello, World!")?;
        parser.parse_events("Dialogue: 0,0:00:05.00,0:00:10.00,Default,,0,0,0,,你好，世界！")?;
        let event = parser.events.get(0).unwrap();
        assert_eq!(
            event
                .get(EventFormat::Layer)
                .and_then(Value::as_int)
                .unwrap(),
            0
        );
        assert_eq!(
            event
                .get(EventFormat::Style)
                .and_then(Value::as_str)
                .unwrap(),
            "Default"
        );
        Ok(())
    }

    #[test]
    fn test_parse_fonts() -> crate::Result<()> {
        let mut parser = SsaParser::default();
        parser.parse_fonts("fontname:Arial")?;
        parser.parse_fonts("fontname:华康方圆体W7")?;
        assert_eq!(parser.fonts.get(0).unwrap(), "Arial");
        assert_eq!(parser.fonts.get(1).unwrap(), "华康方圆体W7");
        Ok(())
    }

    #[test]
    fn test_parse_graphics() -> crate::Result<()> {
        let mut parser = SsaParser::default();
        parser.parse_graphics("filename:logo.png")?;
        parser.parse_graphics("filename:background.jpg")?;
        assert_eq!(parser.graphics.get(0).unwrap(), "logo.png");
        assert_eq!(parser.graphics.get(1).unwrap(), "background.jpg");
        Ok(())
    }

    #[test]
    fn test_parse_str() -> crate::Result<()> {
        let ass_str = r#"
[Script Info]
; Script generated by Aegisub 9530-cibuilds-79a0655eb
; http://www.aegisub.org/
Title: [SweetSub] Oniichan ha Oshimai! - 05
ScriptType: v4.00+
WrapStyle: 0
ScaledBorderAndShadow: yes
YCbCr Matrix: TV.709
PlayResX: 1920
LayoutResX: 1920
PlayResY: 1080
LayoutResY: 1080

[V4+ Styles]
Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding
Style: Default,Source Han Sans SC Medium,90,&H00FFFFFF,&H00000000,&H00000000,&H00000000,0,0,0,0,100,100,0.1,0,1,3,0,2,30,30,25,1
Style: Text - CN,Source Han Sans SC Medium,80,&H00F0F0F0,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0.1,0,1,3,0,2,20,20,50,1
Style: Text - JP,Source Han Sans Medium,50,&H00F0F0F0,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0.1,0,1,3,0,2,20,20,10,1
Style: Text - CN - top,Source Han Sans SC Medium,80,&H00F0F0F0,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0.1,0,1,3,0,8,20,20,10,1
Style: Text - JP - top,Source Han Sans Medium,50,&H00F0F0F0,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0.1,0,1,3,0,8,20,20,80,1
Style: RUBY,Source Han Sans SC Medium,45,&H00FFFFFF,&H00000000,&H00000000,&H00000000,0,0,0,0,100,100,0.1,0,1,2.5,0,2,30,30,25,1
Style: Sign,Source Han Sans SC Medium,90,&H00FFFFFF,&H00000000,&H00000000,&H00000000,0,0,0,0,100,100,0.1,0,1,3,0,8,30,30,25,1
Style: OPJP,FOT-PopJoy Std B,60,&H00FFFFFF,&H00000000,&H00DDAB2F,&H00000000,0,0,0,0,100,100,0.1,0,1,4,0,2,80,80,30,1
Style: OPCN,华康方圆体W7,50,&H00FFFFFF,&H00000000,&H00DDAB2F,&H00000000,-1,0,0,0,100,100,0.1,0,1,4,0,8,80,80,30,1
Style: EDJP,FOT-PopJoy Std B,50,&H00DDAB2F,&H00000000,&H00FFFFFF,&H00000000,0,0,0,0,100,100,0.1,0,1,4,0,2,80,80,40,1
Style: EDCN,华康方圆体W7,43,&H00A066FD,&H00000000,&H00FFFFFF,&H00000000,-1,0,0,0,100,100,0.1,0,1,4,0,2,80,80,90,1
Style: Note,Source Han Sans SC Medium,60,&H00000000,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,1,0,1,0,0,7,30,30,30,1

[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
Comment: 0,0:00:00.00,0:00:00.00,Default,,0,0,0,,Hello, World!
Dialogue: 0,0:00:47.24,0:00:50.24,OPJP,,0,0,0,,World, Hello!
        "#;
        let file = File::from_str(ass_str)?;
        assert_eq!(
            file.script.get_comments().unwrap(),
            &vec![
                Value::Str("Script generated by Aegisub 9530-cibuilds-79a0655eb".to_string()),
                Value::Str("http://www.aegisub.org/".to_string())
            ]
        );
        assert_eq!(file.script.get_play_res_x().unwrap(), 1920);
        Ok(())
    }
}
