mod cli;
mod utils;
mod domain;

use std::io::{self, Write};
use std::thread;
use std::time::Duration;
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
