use std::time::Duration;
use std::io::{self, Write};
use std::thread;
use clap::{ArgAction, Subcommand};
use crate::{domain::{Executable, Timer, Data}, utils::parsers};

#[derive(Subcommand, PartialEq)]
pub enum TimerCommands {
    Start { 
        category: String, 

        #[arg(short, long, value_parser = parsers::parse_duration)]
        duration: Option<Duration>,

        #[arg(short, long, action = ArgAction::SetTrue)]
        show: bool,
    },
    Add { 
        #[arg(value_parser = parsers::parse_duration)]
        duration: Duration, 
    },
    Sub { 
        #[arg(value_parser = parsers::parse_duration)]
        duration: Duration, 
    },
    End,
    Show,
}

impl Executable for TimerCommands {
    fn execute(&self, data: &mut Data) {
        match self {
            Self::Start { category, duration, show } => {
                Self::exec_start(category, *duration, *show, data);
            },
            Self::Add { duration } => {
                Self::exec_add(*duration, data);
            }
            Self::Sub { duration } => {
                Self::exec_sub(*duration, data);
            }
            Self::End => {
                Self::exec_end(data);
            }
            Self::Show => {
                Self::exec_show(data);
            }
        }
    }
}

impl TimerCommands {
    fn exec_start(category: &str, duration: Option<Duration>, show: bool, data: &mut Data) {
        if data.get_running_timer().is_some() {
            panic!("Timer already started!")
        }

        let timer: Timer;


        if let Some(d) = duration {
            timer = Timer::with_duration(d);
        } else {
            timer = Timer::new();
        }

        data.blocks.push(timer.to_block(&category));
        data.save("data.json"); //TODO: Move saving logic

        if show {
            Self::exec_show(data);
        }
    }

    fn exec_add(duration: Duration, data: &mut Data) {
        if let Some(mut timer) = data.get_running_timer() {
            if timer.end_unix.is_none() {
                panic!("Timer does not have a set end time");
            }
            timer.add(duration);
            let category = data.blocks[data.blocks.len() - 1].category.clone();
            data.blocks.pop();
            data.blocks.push(timer.to_block(&category));
        } else {
            println!("No timer is running");
        }
    }

    fn exec_sub(duration: Duration, data: &mut Data) {
        if let Some(mut timer) = data.get_running_timer() {
            if timer.end_unix.is_none() {
                panic!("Timer does not have a set end time");
            }
            timer.sub(duration);
            let category = data.blocks[data.blocks.len() - 1].category.clone();
            data.blocks.pop();
            data.blocks.push(timer.to_block(&category));
        } else {
            println!("No timer is running");
        }
    }
    
    fn exec_end(data: &mut Data) {
        if let Some(mut timer) = data.get_running_timer() {
            timer.end();
            let category = data.blocks[data.blocks.len() - 1].category.clone();
            data.blocks.pop();
            data.blocks.push(timer.to_block(&category));
            println!("Timer stopped successfully");
        } else {
            println!("No timer to end");
        }
    }

    fn exec_show(data: &Data) {
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
