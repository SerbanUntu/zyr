use crate::{
    domain::{Data, Executable, Timer},
    terminal::{FRAME_DURATION_MS, RawTerminal},
    utils::{file_utils, parsers},
};
use clap::{ArgAction, Subcommand};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute, style,
};
use std::error::Error;
use std::io;
use std::time::Duration;

#[derive(Subcommand, PartialEq)]
pub enum TimerCommands {
    /// Start a new timer
    Start {
        /// The type of work or activity. E.g., code, study, break
        category: String,

        /// Optional duration of the timer. If provided, the timer will count backwards.
        #[arg(short, long, value_parser = parsers::parse_duration)]
        duration: Option<Duration>,

        /// Run the `zyr timer show` command immediately after
        #[arg(short, long, action = ArgAction::SetTrue)]
        show: bool,
    },
    /// Make the timer end later. E.g., zyr timer add 5m
    Add {
        /// The amount of time to delay the timer for. E.g., 5m
        #[arg(value_parser = parsers::parse_duration)]
        duration: Duration,
    },
    /// Make the timer end earlier. E.g., zyr timer sub 5m
    Sub {
        /// The amount of time to subtract from the timer. E.g., 5m
        #[arg(value_parser = parsers::parse_duration)]
        duration: Duration,
    },
    /// Stop the currently running timer
    End,
    /// Show the currently running timer
    Show,
}

impl Executable for TimerCommands {
    fn execute(&self, data: &mut Data) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Start {
                category,
                duration,
                show,
            } => Self::exec_start(category, *duration, *show, data)?,
            Self::Add { duration } => Self::exec_add(*duration, data)?,
            Self::Sub { duration } => Self::exec_sub(*duration, data)?,
            Self::End => {
                Self::exec_end(data);
            }
            Self::Show => {
                Self::exec_show(data)?;
            }
        }
        Ok(())
    }
}

impl TimerCommands {
    /// Implementation of the `zyr timer start` command
    fn exec_start(
        category: &str,
        duration: Option<Duration>,
        show: bool,
        data: &mut Data,
    ) -> Result<(), Box<dyn Error>> {
        if data.get_running_timer().is_some() {
            return Err("Timer already started!".into());
        }

        let timer: Timer;

        if let Some(d) = duration {
            timer = Timer::with_duration(d);
        } else {
            timer = Timer::new();
        }

        data.blocks.push(timer.to_block(category));
        data.save(&file_utils::get_data_path());

        if show {
            Self::exec_show(data)?;
        }
        Ok(())
    }

    /// Implementation of the `zyr timer add` command
    fn exec_add(duration: Duration, data: &mut Data) -> Result<(), Box<dyn Error>> {
        if let Some(mut timer) = data.get_running_timer() {
            if timer.end_unix.is_none() {
                return Err("Timer does not have a set end time".into());
            }
            timer.add(duration);
            let category = data.blocks[data.blocks.len() - 1].category.clone();
            data.blocks.pop();
            data.blocks.push(timer.to_block(&category));
        } else {
            println!("No timer is running");
        }
        Ok(())
    }

    /// Implementation of the `zyr timer sub` command
    fn exec_sub(duration: Duration, data: &mut Data) -> Result<(), Box<dyn Error>> {
        if let Some(mut timer) = data.get_running_timer() {
            if timer.end_unix.is_none() {
                return Err("Timer does not have a set end time".into());
            }
            timer.sub(duration);
            let category = data.blocks[data.blocks.len() - 1].category.clone();
            data.blocks.pop();
            data.blocks.push(timer.to_block(&category));
        } else {
            println!("No timer is running");
        }
        Ok(())
    }

    /// Implementation of the `zyr timer end` command
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

    /// Implementation of the `zyr timer show` command
    fn exec_show(data: &Data) -> Result<(), Box<dyn Error>> {
        if let Some(timer) = data.get_running_timer() {
            let _raw_terminal = RawTerminal::new()?;
            let mut dur = Duration::ZERO;
            let frame_dur = Duration::from_millis(FRAME_DURATION_MS);

            fn print_timer(timer: &Timer) {
                execute!(
                    io::stdout(),
                    cursor::MoveToColumn(0),
                    style::Print(format!("{timer}"))
                )
                .unwrap();
            }

            print_timer(&timer);
            loop {
                if event::poll(frame_dur)?
                    && let Event::Key(e) = event::read()?
                    && e.code == KeyCode::Char('c')
                    && e.modifiers.contains(KeyModifiers::CONTROL)
                {
                    break;
                }
                dur += frame_dur;
                if dur >= Duration::from_secs(1) {
                    print_timer(&timer);
                    dur = Duration::ZERO;
                }
            }
        } else {
            println!("No timer is running");
        }

        Ok(())
    }
}
