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
pub enum V4Styles {
    V4Plus(V4PlusStyles),
}

#[derive(Debug, Clone)]
pub struct V4PlusStyles {}

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
