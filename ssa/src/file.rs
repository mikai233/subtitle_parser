use encoding_rs::UTF_8;
use std::fmt::Write;
use strum::VariantNames;

use crate::{
    error::Error,
    events::EventFormat,
    fonts::Fonts,
    graphics::Graphics,
    parser::{Context, SsaParser},
    styles::StyleFormat,
    version::Version,
};
use std::{path::Path, str::FromStr};

use super::{events::Events, script_info::ScriptInfo, styles::V4Styles};

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct File {
    pub version: Version,
    pub script: ScriptInfo,
    pub styles: V4Styles,
    pub events: Events,
    pub fonts: Fonts,
    pub graphics: Graphics,
}

impl File {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_str(ssa_str: impl AsRef<str>) -> crate::Result<Self> {
        Self::parse(ssa_str.as_ref().as_bytes())
    }

    pub fn from_file(path: impl AsRef<Path>) -> crate::Result<Self> {
        let ssa_bytes = std::fs::read(path)?;
        Self::parse(&ssa_bytes)
    }

    pub fn to_string(&self) -> crate::Result<String> {
        let mut ssa = String::new();
        writeln!(ssa, "[Script Info]")?;
        writeln!(ssa, "{}", self.script)?;
        match self.version {
            Version::V4 => {
                writeln!(ssa, "[V4 Styles]")?;
            }
            Version::V4Plus => {
                writeln!(ssa, "[V4+ Styles]")?;
            }
        }
        writeln!(ssa, "{}", self.styles)?;
        writeln!(ssa, "[Events]")?;
        writeln!(ssa, "{}", self.events)?;
        Ok(ssa)
    }

    pub fn write_to(&self, path: impl AsRef<Path>) -> crate::Result<()> {
        std::fs::write(path, self.to_string()?)?;
        Ok(())
    }

    fn parse(ssa_bytes: &[u8]) -> crate::Result<Self> {
        let (ssa_str, _, had_errors) = UTF_8.decode(ssa_bytes);
        if had_errors {
            return Err(Error::InvalidUTF8Encoding);
        }
        let mut version = Version::V4Plus;
        let mut parser = SsaParser::default();
        let mut lines_iter = ssa_str.lines().into_iter();
        while let Some(line) = lines_iter.next() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line.starts_with('[') && line.ends_with(']') {
                let section = line[1..line.len() - 1].trim().to_lowercase();
                match section.as_str() {
                    "script info" => {
                        parser.context = Context::ParseScriptInfo;
                        continue;
                    }
                    "v4 styles" => {
                        version = Version::V4;
                        parser.context = Context::ParseStyles;
                        let header = lines_iter.next().ok_or(Error::ParseError {
                            ty: "header",
                            msg: "missing v4 styles header".to_string(),
                        })?;
                        let order = Self::parse_style_header(header)?;
                        parser.styles = V4Styles::new(order)?;
                        continue;
                    }
                    "v4+ styles" => {
                        version = Version::V4Plus;
                        parser.context = Context::ParseStyles;
                        let header = lines_iter.next().ok_or(Error::ParseError {
                            ty: "header",
                            msg: "missing v4+ styles header".to_string(),
                        })?;
                        let order = Self::parse_style_header(header)?;
                        parser.styles = V4Styles::new(order)?;
                        continue;
                    }
                    "events" => {
                        parser.context = Context::ParseEvents;
                        let header = lines_iter.next().ok_or(Error::ParseError {
                            ty: "header",
                            msg: "missing events header".to_string(),
                        })?;
                        let order = Self::parse_event_header(header)?;
                        parser.events = Events::new(order);
                        continue;
                    }
                    "fonts" => {
                        parser.context = Context::ParseFonts;
                        continue;
                    }
                    "graphics" => {
                        parser.context = Context::ParseGraphics;
                        continue;
                    }
                    _ => {}
                }
            }
            match parser.context {
                Context::None => {}
                Context::ParseScriptInfo => {
                    parser.parse_script_info(line)?;
                }
                Context::ParseStyles => {
                    parser.parse_styles(line)?;
                }
                Context::ParseEvents => {
                    parser.parse_events(line)?;
                }
                Context::ParseFonts => {
                    parser.parse_fonts(line)?;
                }
                Context::ParseGraphics => {
                    parser.parse_graphics(line)?;
                }
            }
        }
        let SsaParser {
            script_info,
            styles,
            events,
            fonts,
            graphics,
            ..
        } = parser;
        let file = File {
            version,
            script: script_info,
            styles,
            events,
            fonts,
            graphics,
        };
        Ok(file)
    }

    fn parse_style_header(header: &str) -> crate::Result<Vec<StyleFormat>> {
        match header.find(':') {
            Some(pos) => {
                if header[..pos].trim().to_ascii_lowercase() != "format" {
                    return Err(Error::invalid_type("format"));
                }
                let order = header[pos + 1..]
                    .split(',')
                    .map(|s| s.trim())
                    .map(|s| StyleFormat::from_str(s))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|error| {
                        Error::invalid_type(format!(
                            "{} {:?}",
                            StyleFormat::VARIANTS.join(", "),
                            error
                        ))
                    })?;
                Ok(order)
            }
            None => Err(Error::parse_error::<V4Styles>(
                "invalid style header format",
            )),
        }
    }

    fn parse_event_header(header: &str) -> crate::Result<Vec<EventFormat>> {
        match header.find(':') {
            Some(pos) => {
                if header[..pos].trim().to_ascii_lowercase() != "format" {
                    return Err(Error::invalid_type("format"));
                }
                let order = header[pos + 1..]
                    .split(',')
                    .map(|s| s.trim())
                    .map(|s| EventFormat::from_str(s))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|error| {
                        Error::invalid_type(format!(
                            "{} {:?}",
                            EventFormat::VARIANTS.join(", "),
                            error
                        ))
                    })?;
                Ok(order)
            }
            None => Err(Error::parse_error::<Events>("invalid event header format")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::File;
    use crate::events::EventFormat;
    use crate::script_info::ScriptType;
    use crate::value::Value;
    use crate::version::Version;
    use std::time::Duration;

    #[test]
    fn test_default() {
        let file = File::new();
        assert_eq!(file.version, Version::V4Plus);
        assert!(matches!(
            file.script.get_script_type(),
            Some(ScriptType::V4Plus)
        ));
    }

    #[test]
    fn test_file() -> crate::Result<()> {
        let path="D:/BaiduNetdiskDownload/TSDM@sillonae@onimai 别当欧尼酱了/[SweetSub&VCB-Studio] Oniichan ha Oshimai! [Ma10p_1080p]/[SweetSub&VCB-Studio] Oniichan ha Oshimai! [01][Ma10p_1080p][x265_flac_2ac3].chs.ass";
        let mut file = File::from_file(path)?;
        for event in file.events.iter_mut() {
            if let Some(duration) = event
                .get_mut(EventFormat::Start)
                .and_then(Value::as_duration_mut)
            {
                *duration += Duration::from_secs(60);
            }
        }
        println!("{}", file.to_string()?);
        Ok(())
    }
}
