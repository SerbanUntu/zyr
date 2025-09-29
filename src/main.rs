mod cli;
mod utils;
mod domain;

use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use chrono::{DateTime, Datelike, Local, TimeZone, Timelike, Utc};
use cli::Commands;
use cli::timer::TimerCommands;
use cli::Cli;
use domain::{Data, Timer};
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    let mut data = Data::from_file("data.json");

    match cli.command {
        Commands::Timer { command: cmd } => {

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
            }
            if let TimerCommands::Add { duration } = &cmd {
                if let Some(mut timer) = data.get_running_timer() {
                    if timer.end_unix.is_none() {
                        panic!("Timer does not have a set end time");
                    }
                    timer.add(*duration);
                    let category = data.blocks[data.blocks.len() - 1].category.clone();
                    data.blocks.pop();
                    data.blocks.push(timer.to_block(&category));
                } else {
                    println!("No timer is running");
                }
            }
            if let TimerCommands::Sub { duration } = &cmd {
                if let Some(mut timer) = data.get_running_timer() {
                    if timer.end_unix.is_none() {
                        panic!("Timer does not have a set end time");
                    }
                    timer.sub(*duration);
                    let category = data.blocks[data.blocks.len() - 1].category.clone();
                    data.blocks.pop();
                    data.blocks.push(timer.to_block(&category));
                } else {
                    println!("No timer is running");
                }
            }
            if cmd == TimerCommands::End {
                if let Some(mut timer) = data.get_running_timer() {
                    timer.end();
                    let category = data.blocks[data.blocks.len() - 1].category.clone();
                    data.blocks.pop();
                    data.blocks.push(timer.to_block(&category));
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
        },
        Commands::Plan { command: _ } => {
        },
        Commands::View => {
            let now_dt = Local::now();

            fn same_day(a: DateTime<Local>, b: DateTime<Local>) -> bool {
                a.year() == b.year() && a.month() == b.month() && a.day() == b.day()
            }

            fn convert(millis: u64) -> DateTime<Local> {
                let utc = Utc.timestamp_millis_opt(millis as i64)
                    .single()
                    .expect("invalid timestamp");
                utc.with_timezone(&Local)
            }

            fn millis_since_midnight(dt: DateTime<Local>) -> u64 {
                (dt.hour() * 3_600_000 + dt.minute() * 60_000 + dt.second() * 1_000) as u64
            }

            fn prettify_duration(d: Duration) -> String {
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

            let filtered: Vec<(Duration, &str)> = data.blocks
                .iter()
                .filter(|b| { 
                    same_day(now_dt, convert(b.start_unix)) || 
                    (b.end_unix.is_some() && same_day(now_dt, convert(b.end_unix.unwrap()))) ||
                    b.end_unix.is_none()
                })
                .map(|b| {
                    let millis;
                    let start_dt = convert(b.start_unix);
                    let mut end_dt = now_dt;

                    if let Some(end) = b.end_unix {
                        end_dt = convert(end);
                        if end_dt > now_dt {
                            end_dt = now_dt;
                        }
                    }

                    if !same_day(now_dt, start_dt) {
                        millis = millis_since_midnight(end_dt);
                    } else {
                        millis = (end_dt - start_dt).num_milliseconds() as u64;
                    }

                    (Duration::from_millis(millis), &b.category[..])
                })
                .collect();

            let time_worked: Duration = filtered
                .iter()
                .filter(|t| t.1 != "break")
                .fold(Duration::from_millis(0), |t, c| t + c.0);

            let time_break: Duration = filtered
                .iter()
                .filter(|t| t.1 == "break")
                .fold(Duration::from_millis(0), |t, c| t + c.0);

            println!("Overview:\n\nTime worked: {}\nBreak time: {}\nTotal: {}",
                prettify_duration(time_worked),
                prettify_duration(time_break),
                prettify_duration(time_worked + time_break)
            );
        }
        Commands::Clear => {
            println!("Are you sure you want to clear all data? (y/N)");
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).expect("Did not enter a correct string");

            if buf.chars().next().is_some_and(|ch| ch == 'y') {
                data = Data::empty();
                println!("All data has been deleted");
            } else {
                println!("Data was not deleted");
            }

        }
    }

    data.save("data.json");
}

#[cfg(test)]
mod tests {
}
