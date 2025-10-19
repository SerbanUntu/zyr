pub mod clear;
pub mod plan;
pub mod timer;
pub mod view;

use crate::domain::{Data, Executable};
use clap::{Parser, Subcommand};
use plan::PlanCommands;
use std::error::Error;
use timer::TimerCommands;

#[derive(Parser)]
#[command(name = "zyr")]
#[command(version = "0.1.0")]
#[command(about = "Productivity timer and manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, PartialEq)]
pub enum Commands {
    /// Start a timer to track your work
    Timer {
        #[command(subcommand)]
        command: TimerCommands,
    },
    /// Manually create, delete, or modify time blocks
    Plan {
        #[command(subcommand)]
        command: PlanCommands,
    },
    /// Reset all data stored on this device
    Clear,
    /// View statistics about what you worked on today
    View,
}

impl Executable for Commands {
    fn execute(&self, data: &mut Data) -> Result<(), Box<dyn Error>> {
        match self {
            Commands::Timer { command } => command.execute(data)?,
            Commands::Plan { command } => command.execute(data)?,
            Commands::Clear => clear::exec(data),
            Commands::View => view::exec(data),
        }
        Ok(())
    }
}
