use crate::value::Value;

#[derive(Debug, Copy, Clone, strum::Display, strum::FromRepr, strum::EnumString)]
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

impl PartialEq for StyleFormat {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for StyleFormat {}

#[derive(Debug, Clone)]
pub struct Event {}

#[derive(Debug, Clone)]
pub struct Style(Vec<Option<Value>>);

impl Style {
     
}

#[derive(Debug, Clone)]
pub struct V4Styles {
    order: Vec<StyleFormat>,
    styles: Vec<Style>,
}

impl V4Styles {
    pub fn new(order: Vec<StyleFormat>) -> Self {
        Self {
            order,
            styles: vec![],
        }
    }

    pub fn order(&self) -> &[StyleFormat] {
        &self.order
    }

    pub fn add(&mut self, format: StyleFormat) {}
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

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
}
