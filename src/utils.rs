pub mod parsers {

    use chrono::{DateTime, Local};
    use std::time::Duration;

    /// Parse a duration string like "1h35m50s" into `Duration`
    pub fn parse_duration(s: &str) -> Result<Duration, String> {
        let mut secs = 0u64;
        let mut num = String::new();

        for c in s.chars() {
            if c.is_ascii_digit() {
                num.push(c);
            } else if !c.is_whitespace() {
                let value: u64 = num.parse().map_err(|_| format!("Invalid number in {s}"))?;
                num.clear();

                match c {
                    'h' => secs += value * 3600,
                    'm' => secs += value * 60,
                    's' => secs += value,
                    _ => return Err(format!("Unknown duration unit: {c}")),
                }
            }
        }

        if !num.is_empty() {
            return Err(format!("Trailing number without unit in {s}"));
        }

        Ok(Duration::from_secs(secs))
    }

    pub fn parse_timestamp(s: &str) -> Result<DateTime<Local>, String> {
        let humantime_result = humantime::parse_rfc3339_weak(s);
        match humantime_result {
            Ok(st) => Ok(st.into()),
            Err(e) => Err(e.to_string()),
        }
    }
}

pub mod file_utils {

    use std::fs::{self, *};
    use std::path::PathBuf;

    use crate::domain::Data;

    pub fn get_data_path() -> Box<PathBuf> {
        let file_path = directories::ProjectDirs::from("org", "zyr", "zyr")
            .expect("Could not open the project directory")
            .data_dir()
            .join("data.json");

        match file_path.try_exists() {
            Err(_) | Ok(false) => {
                if let Some(parent) = file_path.parent() {
                    fs::create_dir_all(parent).expect("Could not create project directory");
                }

                let _ = File::create(&file_path);
                Data::new().save(&file_path);
            }
            _ => (),
        }

        Box::new(file_path)
    }
}

pub mod io_utils {

    use std::io;

    pub fn confirm(msg: &str) -> bool {
        println!("Are you sure you want to {msg}? (y/N)");
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Did not enter a correct string");

        buf.chars().next().is_some_and(|ch| ch == 'y')
    }
}

pub mod time_utils {

    use chrono::{DateTime, Datelike, Local, TimeZone, Timelike, Utc};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    pub fn since_unix() -> Duration {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
    }

    pub fn same_day(a: DateTime<Local>, b: DateTime<Local>) -> bool {
        a.year() == b.year() && a.month() == b.month() && a.day() == b.day()
    }

    pub fn convert(millis: u64) -> DateTime<Local> {
        let utc = Utc
            .timestamp_millis_opt(millis as i64)
            .single()
            .expect("invalid timestamp");
        utc.with_timezone(&Local)
    }

    pub fn millis_since_midnight(dt: DateTime<Local>) -> u64 {
        (dt.hour() * 3_600_000 + dt.minute() * 60_000 + dt.second() * 1_000) as u64
    }

    pub fn prettify_duration(d: Duration) -> String {
        let mut result = String::from("");

        let total_seconds = d.as_secs() as u32;
        let hours = total_seconds / 60 / 60;
        let minutes = (total_seconds - hours * 3600) / 60;
        let seconds = total_seconds % 60;

        if hours > 0 {
            result.push_str(&format!("{}h", hours));
        }
        if hours > 0 || minutes > 0 {
            result.push_str(&format!("{}m", minutes));
        }
        result.push_str(&format!("{}s", seconds));

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration_only_hour() {
        let result = parsers::parse_duration("4h");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_secs(), 4 * 3600);
    }

    #[test]
    fn test_parse_duration_only_minutes() {
        let result = parsers::parse_duration("34m");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_secs(), 34 * 60);
    }

    #[test]
    fn test_parse_duration_only_seconds() {
        let result = parsers::parse_duration("14s");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_secs(), 14);
    }

    #[test]
    fn test_parse_duration_combined_units() {
        let result = parsers::parse_duration("7h49m5s");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_secs(), 7 * 3600 + 49 * 60 + 5);
    }

    #[test]
    fn test_parse_duration_zeros() {
        let result = parsers::parse_duration("0h0m0s");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_secs(), 0);
    }

    #[test]
    fn test_parse_duration_random_order() {
        let result = parsers::parse_duration("3m5s3h");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_secs(), 3 * 3600 + 3 * 60 + 5);
    }

    #[test]
    fn test_parse_duration_whitespace() {
        let result = parsers::parse_duration(" 8h 5s  ");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_secs(), 8 * 3600 + 5);
    }

    #[test]
    fn test_parse_duration_above_23_or_59() {
        let result = parsers::parse_duration("161h345m287s");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_secs(), 161 * 3600 + 345 * 60 + 287);
    }

    #[test]
    fn test_parse_duration_empty() {
        let result = parsers::parse_duration("");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_secs(), 0);
    }

    #[test]
    fn test_parse_duration_unknown_time_quantity() {
        let result = parsers::parse_duration("73q");
        assert!(result.is_err());
    }
}
