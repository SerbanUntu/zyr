use serde::{Serialize, Deserialize};
use std::{fmt, fs::{self, File}, io::{self, Write}, thread, time::{self, Duration, SystemTime, UNIX_EPOCH}};

use clap::{Parser};

fn since_unix() -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
}

#[derive(Parser)]
#[command(name = "zyr")]
#[command(version = "0.1.0")]
#[command(about = "Productivity timer and manager")]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,
}

#[derive(Debug)]
struct Timer {
    start_unix: u64,
    end_unix: Option<u64>,
}

impl Timer {
    fn new() -> Self {
        Self {
            start_unix: since_unix().as_millis() as u64,
            end_unix: None
        }
    }

    fn with_duration(duration: Duration) -> Self {
        let now: u64 = since_unix().as_millis() as u64;
        Self {
            start_unix: now,
            end_unix: Some(now + duration.as_millis() as u64)
        }
    }

    fn with_initial_time(start_unix: u64) -> Self {
        Self {
            start_unix,
            end_unix: None
        }
    }

    fn end(&mut self) {
        self.end_unix = Some(since_unix().as_millis() as u64);
    }

    fn get_hours_minutes_seconds(&self) -> (u32, u32, u32) {
        let to_display: Duration;

        if let Some(end) = self.end_unix {
            // Time remaining
            to_display = since_unix().abs_diff(Duration::from_millis(end));
            
        } else {
            // Time since start
            to_display = since_unix().abs_diff(Duration::from_millis(self.start_unix));
        }

        let total_seconds = to_display.as_secs() as u32;
        let hours = total_seconds / 60 / 60;
        let minutes = (total_seconds - hours * 3600) / 60;
        
        (hours, minutes, total_seconds % 60)
    }

    fn to_block(&self, category: &str) -> TimeBlock {
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
struct TimeBlock {
    start_unix: u64,
    end_unix: Option<u64>,
    category: String,
}

impl TimeBlock {
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    categories: Vec<String>,
    blocks: Vec<TimeBlock>
}

impl Data {
    fn from_file(path: &str) -> Self {
        let json_str = fs::read_to_string(path).expect("File could not be read");
        println!("{}", &json_str);
        serde_json::from_str(&json_str).expect("JSON could not be parsed")
    }

    fn save(&self, path: &str) {
        let mut file = File::create(path).expect("File could not be opened");
        let stringified = serde_json::to_string(self).expect("Object could not be serialized");
        let _ = file.write_all(stringified.as_bytes()).expect("Could not write to file");
    }
}

fn main() {
    let _cli = Cli::parse();
    let mut timer = Timer::new();

    let mut data = Data::from_file("data.json");

    thread::sleep(Duration::from_secs(2));
    timer.end();

    data.blocks.push(timer.to_block("code"));
    data.save("data.json");
}

#[cfg(test)]
mod tests {
}
