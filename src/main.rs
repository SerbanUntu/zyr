use serde::{Serialize, Deserialize};
use std::{fmt, fs::{self, File}, io::{self, Write}, thread, time::{Duration, SystemTime, UNIX_EPOCH}};

use clap::{Parser, Subcommand};

fn since_unix() -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
}

/// Parse a duration string like "1h35m50s" into `Duration`
fn parse_duration(s: &str) -> Result<Duration, String> {
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

#[derive(Parser)]
#[command(name = "zyr")]
#[command(version = "0.1.0")]
#[command(about = "Productivity timer and manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, PartialEq)]
enum Commands {
    Timer {
        #[command(subcommand)]
        command: TimerCommands,
    },
}

#[derive(Subcommand, PartialEq)]
enum TimerCommands {
    Start { 
        category: String, 

        #[arg(short, long, value_parser = parse_duration)]
        duration: Option<Duration>,
    },
    Add { 
        #[arg(value_parser = parse_duration)]
        time: Duration, 
    },
    Sub { 
        #[arg(value_parser = parse_duration)]
        time: Duration, 
    },
    End,
    Show,
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

    #[allow(dead_code)]
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

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    categories: Vec<String>,
    blocks: Vec<TimeBlock>
}

impl Data {
    #[allow(dead_code)]
    fn empty() -> Self {
        Self {
            categories: vec![],
            blocks: vec![]
        }
    }

    fn from_file(path: &str) -> Self {
        let json_str = fs::read_to_string(path).expect("File could not be read");
        serde_json::from_str(&json_str).expect("JSON could not be parsed")
    }

    fn save(&self, path: &str) {
        let mut file = File::create(path).expect("File could not be opened");
        let stringified = serde_json::to_string(self).expect("Object could not be serialized");
        let _ = file.write_all(stringified.as_bytes()).expect("Could not write to file");
    }

    fn get_running_timer(&self) -> Option<Timer> {
        self.blocks
            .iter()
            .filter(|b| { 
                if let Some(end) = b.end_unix {
                    let now = since_unix().as_millis() as u64;
                    end > now
                }
                else { true }
            })
            .map(|b| { Timer { start_unix: b.start_unix, end_unix: b.end_unix } })
            .next()
    }
}

fn main() {
    let cli = Cli::parse();
    let mut data = Data::from_file("data.json");
    let Commands::Timer { command: cmd } = cli.command;

    if let TimerCommands::Start { duration, category } = &cmd {
        if data.get_running_timer().is_some() {
            panic!("Timer already started!")
        }

        let timer: Timer;


        if let Some(d) = *duration {
            timer = Timer::with_duration(d);
        } else {
            timer = Timer::new();
        }

        data.blocks.push(timer.to_block(&category));
        data.save("data.json");
    }
    if cmd == TimerCommands::End {
        if let Some(mut timer) = data.get_running_timer() {
            timer.end();
            let category = data.blocks[data.blocks.len() - 1].category.clone();
            data.blocks.pop();
            data.blocks.push(timer.to_block(&category));
            data.save("data.json");
        } else {
            println!("No timer to end");
        }
    }
    if cmd == TimerCommands::Show {
        if let Some(timer) = data.get_running_timer() {
            loop {
                print!("\x1b[2K\r{}", timer);
                io::stdout().flush().unwrap();
                thread::sleep(Duration::from_secs(1))
            }
        } else {
            println!("No timer is running");
        }
    }
}

#[cfg(test)]
mod tests {
}
