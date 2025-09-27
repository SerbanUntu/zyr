use std::{fmt, io::{self, Write}, thread, time};

use clap::{Parser};

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
    seconds: u32,
}

impl Timer {
    fn new() -> Self {
        Self {
            seconds: 0
        }
    }

    fn with_initial_time(seconds: u32) -> Self {
        Self {
            seconds
        }
    }

    fn pass(&mut self) {
        self.seconds += 1;
    }

    fn get_hours(&self) -> u32 {
        self.seconds / 60 / 60
    }

    fn get_minutes(&self) -> u32 {
        (self.seconds - self.get_hours() * 3600) / 60
    }

    fn get_seconds(&self) -> u32 {
        self.seconds % 60
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}:{:0>2}", self.get_hours(), self.get_minutes(), self.get_seconds())
    }
}

fn main() {
    let _cli = Cli::parse();
    let mut timer = Timer::new();

    loop {
        print!("\x1b[2K\r{}", timer);
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_secs(1));
        timer.pass();
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_timer() {
        assert_eq!(format!("{}", Timer::new()), "00:00:00");
    }

    #[test]
    fn minute_boundary() {
        let mut timer = Timer::with_initial_time(59);
        assert_eq!(format!("{}", timer), "00:00:59");
        timer.pass();
        assert_eq!(format!("{}", timer), "00:01:00");
    }

    #[test]
    fn hour_boundary() {
        let mut timer = Timer::with_initial_time(59 * 60 + 59);
        assert_eq!(format!("{}", timer), "00:59:59");
        timer.pass();
        assert_eq!(format!("{}", timer), "01:00:00");
    }
}
