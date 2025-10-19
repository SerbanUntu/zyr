use crate::utils::time_utils;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::time::Duration;

type Timestamp = u64;

#[derive(Debug)]
pub struct Timer {
    pub start_unix: Timestamp,
    pub end_unix: Option<Timestamp>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start_unix: time_utils::since_unix().as_millis() as u64,
            end_unix: None,
        }
    }

    pub fn with_duration(duration: Duration) -> Self {
        let now: u64 = time_utils::since_unix().as_millis() as u64;
        Self {
            start_unix: now,
            end_unix: Some(now + duration.as_millis() as u64),
        }
    }

    #[allow(dead_code)]
    pub fn with_initial_time(start_unix: u64) -> Self {
        Self {
            start_unix,
            end_unix: None,
        }
    }

    pub fn add(&mut self, duration: Duration) {
        self.end_unix = Some(self.end_unix.unwrap() + duration.as_millis() as u64);
    }

    pub fn sub(&mut self, duration: Duration) {
        self.end_unix = Some(self.end_unix.unwrap() - duration.as_millis() as u64);
    }

    pub fn end(&mut self) {
        self.end_unix = Some(time_utils::since_unix().as_millis() as u64);
    }

    pub fn get_hours_minutes_seconds(&self) -> (u32, u32, u32) {
        let to_display: Duration;

        if let Some(end) = self.end_unix {
            // Time remaining
            to_display = time_utils::since_unix().abs_diff(Duration::from_millis(end));
        } else {
            // Time since start
            to_display = time_utils::since_unix().abs_diff(Duration::from_millis(self.start_unix));
        }

        let total_seconds = to_display.as_secs() as u32;
        let hours = total_seconds / 60 / 60;
        let minutes = (total_seconds - hours * 3600) / 60;

        (hours, minutes, total_seconds % 60)
    }

    pub fn to_block(&self, category: &str) -> TimeBlock {
        TimeBlock {
            start_unix: self.start_unix,
            end_unix: self.end_unix,
            category: category.to_owned(),
        }
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (hours, minutes, seconds) = self.get_hours_minutes_seconds();
        write!(f, "{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeBlock {
    pub start_unix: u64,
    pub end_unix: Option<u64>,
    pub category: String,
}

impl fmt::Display for TimeBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let start_dt = time_utils::convert(self.start_unix);
        let end_str = match self.end_unix {
            Some(e) => time_utils::convert(e).to_rfc2822(),
            None => "Ongoing".to_string(),
        };
        write!(
            f,
            "{}: {}, {}",
            self.category,
            start_dt.to_rfc2822(),
            end_str
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    categories: Vec<String>,
    pub blocks: Vec<TimeBlock>, //TODO: Expose block methods in interface
}

impl Data {
    pub fn empty() -> Self {
        Self {
            categories: vec![],
            blocks: vec![],
        }
    }

    pub fn new() -> Self {
        Self {
            categories: vec![String::from("break")],
            blocks: vec![],
        }
    }

    pub fn from_file(path: &Path) -> Self {
        let json_str = fs::read_to_string(path).expect("File could not be read");
        serde_json::from_str(&json_str).expect("JSON could not be parsed")
    }

    pub fn save(&self, path: &Path) {
        let mut file = File::create(path).expect("File could not be opened");
        let stringified = serde_json::to_string(self).expect("Object could not be serialized");
        file.write_all(stringified.as_bytes())
            .expect("Could not write to file");
    }

    pub fn get_running_timer(&self) -> Option<Timer> {
        self.blocks
            .iter()
            .filter(|b| {
                if let Some(end) = b.end_unix {
                    let now = time_utils::since_unix().as_millis() as u64;
                    end > now
                } else {
                    true
                }
            })
            .map(|b| Timer {
                start_unix: b.start_unix,
                end_unix: b.end_unix,
            })
            .next()
    }
}

pub trait Executable {
    fn execute(&self, data: &mut Data) -> Result<(), Box<dyn Error>>;
}
