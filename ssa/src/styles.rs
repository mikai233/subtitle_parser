use crate::value::Value;

#[derive(
    Debug, Copy, Clone, Eq, PartialEq, strum::Display, strum::EnumString, strum::VariantNames,
)]
#[strum(ascii_case_insensitive)]
pub enum StyleFormat {
    Name,
    Fontname,
    Fontsize,
    PrimaryColour,
    SecondaryColour,
    TertiaryColour,
    OutlineColor,
    BackColour,
    Bold,
    Italic,
    Underline,
    StrikeOut,
    ScaleX,
    ScaleY,
    Spacing,
    Angle,
    BorderStyle,
    Outline,
    Shadow,
    Alignment,
    MarginL,
    MarginR,
    MarginV,
    AlphaLevel,
    Encoding,
}

#[derive(Debug, Clone)]
pub struct Style(Vec<(StyleFormat, Option<Value>)>);

impl Style {
    pub fn new(styles: &V4Styles) -> Self {
        let mut style = vec![];
        for format in styles.order() {
            style.push((format.clone(), None));
        }
        Self(style)
    }

    pub fn set(&mut self, format: StyleFormat, value: impl Into<Value>) {
        for (f, v) in self.0.iter_mut() {
            if f == &format {
                *v = Some(value.into());
                return;
            }
        }
    }

    pub fn get(&self, format: StyleFormat) -> Option<&Value> {
        for (f, v) in self.0.iter() {
            if f == &format {
                return v.as_ref();
            }
        }
        None
    }

    pub fn get_mut(&mut self, format: StyleFormat) -> Option<&mut Value> {
        for (f, v) in self.0.iter_mut() {
            if f == &format {
                return v.as_mut();
            }
        }
        None
    }

    pub fn remove(&mut self, format: StyleFormat) {
        for (f, v) in self.0.iter_mut() {
            if f == &format {
                *v = None;
                return;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct V4Styles {
    order: Vec<StyleFormat>,
    styles: Vec<(String, Style)>,
}

impl V4Styles {
    pub fn new(order: Vec<StyleFormat>) -> crate::Result<Self> {
        if !order.contains(&StyleFormat::Name) {
            return Err(crate::Error::V4StyleNameNotFound);
        }
        let styles = Self {
            order,
            styles: vec![],
        };
        Ok(styles)
    }

    pub fn order(&self) -> &[StyleFormat] {
        &self.order
    }

    pub fn add(&mut self, style: Style) -> crate::Result<()> {
        let name = style
            .get(StyleFormat::Name)
            .ok_or(crate::Error::V4StyleNameNotFound)?
            .as_str()
            .ok_or(crate::Error::invalid_type("str"))?
            .to_string();
        self.styles.push((name, style));
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Style> {
        for (n, style) in self.styles.iter() {
            if n == name {
                return Some(style);
            }
        }
        None
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Style> {
        for (n, style) in self.styles.iter_mut() {
            if n == name {
                return Some(style);
            }
        }
        None
    }

    pub fn remove(&mut self, name: &str) {
        self.styles.retain(|(n, _)| n != name);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &Style)> {
        self.styles.iter().map(|(n, s)| (n.as_str(), s))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&str, &mut Style)> {
        self.styles.iter_mut().map(|(n, s)| (n.as_str(), s))
    }
}

impl Default for V4Styles {
    fn default() -> Self {
        Self {
            order: vec![
                StyleFormat::Name,
                StyleFormat::Fontname,
                StyleFormat::Fontsize,
                StyleFormat::PrimaryColour,
                StyleFormat::SecondaryColour,
                StyleFormat::OutlineColor,
                StyleFormat::BackColour,
                StyleFormat::Bold,
                StyleFormat::Italic,
                StyleFormat::Underline,
                StyleFormat::StrikeOut,
                StyleFormat::ScaleX,
                StyleFormat::ScaleY,
                StyleFormat::Spacing,
                StyleFormat::Angle,
                StyleFormat::BorderStyle,
                StyleFormat::Outline,
                StyleFormat::Shadow,
                StyleFormat::Alignment,
                StyleFormat::MarginL,
                StyleFormat::MarginR,
                StyleFormat::MarginV,
                StyleFormat::Encoding,
            ],
            styles: vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use std::{f32::consts::E, str::FromStr};

    use crate::{error::Error, events::EventFormat};

    use super::StyleFormat;

    #[test]
    fn test_events_format() {
        let event = StyleFormat::from_str("Name").unwrap();
        assert_eq!(event, StyleFormat::Name);
        let event = StyleFormat::from_str("SecondaryColour").unwrap();
        assert_eq!(event, StyleFormat::SecondaryColour);
        let event = StyleFormat::from_str("secondaryColour").unwrap();
        assert_eq!(event, StyleFormat::SecondaryColour);
        let event = StyleFormat::from_str("secondarycolour").unwrap();
        assert_eq!(event, StyleFormat::SecondaryColour);
        assert_eq!(event.to_string(), "SecondaryColour");
        assert_eq!(StyleFormat::MarginV.to_string(), "MarginV");
    }

    #[test]
    fn test_parse_event_format() -> crate::Result<()> {
        let format = [
            "Name",
            "Fontname",
            "Fontsize",
            "PrimaryColour",
            "SecondaryColour",
            "OutlineColour",
            "BackColour",
            "Bold",
            "Italic",
            "Underline",
            "StrikeOut",
            "ScaleX",
            "ScaleY",
            "Spacing",
            "Angle",
            "BorderStyle",
            "Outline",
            "Shadow",
            "Alignment",
            "MarginL",
            "MarginR",
            "MarginV",
            "Encoding",
        ];
        for ele in format {
            EventFormat::from_str(ele)
                .map_err(|_| Error::parse_error::<EventFormat>(ele.to_string()))?;
        }
        Ok(())
    }
}
