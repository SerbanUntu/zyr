pub mod parsers {

    use std::time::Duration;

    /// Parse a duration string like "1h35m50s" into `Duration`
    pub fn parse_duration(s: &str) -> Result<Duration, String> {
        let mut secs = 0u64;
        let mut num = String::new();

        for c in s.chars() {
            if c.is_ascii_digit() {
                num.push(c);
            } else {
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
}

pub mod time_utils {

    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use chrono::{Local, Timelike, Datelike, TimeZone, Utc, DateTime};

    pub fn since_unix() -> Duration {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
    }

    pub fn same_day(a: DateTime<Local>, b: DateTime<Local>) -> bool {
        a.year() == b.year() && a.month() == b.month() && a.day() == b.day()
    }

    pub fn convert(millis: u64) -> DateTime<Local> {
        let utc = Utc.timestamp_millis_opt(millis as i64)
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

