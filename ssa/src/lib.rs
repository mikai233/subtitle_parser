use crate::error::Error;
use crate::parser::Parser;
use std::time::Duration;

pub mod error;
pub mod events;
pub mod file;
pub mod fonts;
pub mod graphics;
pub mod parser;
pub mod script_info;
pub mod styles;
pub mod value;
pub mod version;

pub type Result<T> = std::result::Result<T, Error>;

pub fn format_duration(duration: &Duration) -> String {
    let total_millis = duration.as_millis();
    let hours = total_millis / 3600_000;
    let minutes = (total_millis % 3600_000) / 60_000;
    let seconds = (total_millis % 60_000) / 1_000;
    let millis_seconds = total_millis % 1_000;
    format!(
        "{}:{:02}:{:02}.{:02}",
        hours, minutes, seconds, millis_seconds
    )
}

impl Parser for Duration {
    fn parse(src: &str) -> crate::Result<Self> {
        let split1: Vec<_> = src.split(".").collect();
        if split1.len() != 2 {
            return Err(Error::parse_error::<Duration>(format!(
                "invalid duration format {}",
                src
            )));
        }
        let split2: Vec<_> = split1[0].split(":").collect();
        if split2.len() != 3 {
            return Err(Error::parse_error::<Duration>(format!(
                "invalid duration format {}",
                src
            )));
        }
        let h = split2[0]
            .parse::<u64>()
            .map_err(|e| Error::parse_int_error(e, split2[0]))?;
        if h >= 24 {
            return Err(Error::parse_error::<Duration>(format!(
                "hour {} out of range",
                src
            )));
        }
        let m = split2[1]
            .parse::<u64>()
            .map_err(|e| Error::parse_int_error(e, split2[1]))?;
        if m >= 60 {
            return Err(Error::parse_error::<Duration>(format!(
                "minute {} out of range",
                src
            )));
        }
        let s = split2[2]
            .parse::<u64>()
            .map_err(|e: std::num::ParseIntError| Error::parse_int_error(e, split2[2]))?;
        if s >= 60 {
            return Err(Error::parse_error::<Duration>(format!(
                "second {} out of range",
                src
            )));
        }
        let ms = split1[1]
            .parse::<u64>()
            .map_err(|e| Error::parse_int_error(e, split1[1]))?;
        if ms >= 1000 {
            return Err(Error::parse_error::<Duration>(format!(
                "millisecond {} out of range",
                src
            )));
        }
        let duration = Duration::from_millis(h * 3600_000 + m * 60_000 + s * 1_000 + ms);
        Ok(duration)
    }
}

#[cfg(test)]
mod test {
    use crate::format_duration;
    use crate::parser::Parser;
    use std::time::Duration;

    #[test]
    fn test_format_duration() {
        let z: Duration = Duration::ZERO;
        assert_eq!(format_duration(&z), "0:00:00.00");
    }

    #[test]
    fn test_duration_parse() {
        let d = Duration::parse("0v0:00:00.00");
        assert!(d.is_err());
        let d = Duration::parse("00:00:0000");
        assert!(d.is_err());
        let d = Duration::parse("08:35:09.88");
        assert!(d.is_ok());
        let d = Duration::parse("7:35:09.88");
        assert!(d.is_ok());
        let d = Duration::parse("08:35:99.88");
        assert!(d.is_err());
        let d = Duration::parse("08:60:9.88");
        assert!(d.is_err());
        let d = Duration::parse("24:59:9.88");
        assert!(d.is_err());
        let d = Duration::parse("24:59:9.1188");
        assert!(d.is_err());
    }
}
