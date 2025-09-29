use std::time::{Duration, SystemTime, UNIX_EPOCH};

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

pub fn since_unix() -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
}

