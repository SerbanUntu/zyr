pub mod clear;
pub mod timer;
pub mod plan;
pub mod view;

use clap::{Parser, Subcommand};
use crate::domain::{Executable, Data};
use timer::TimerCommands;
use plan::PlanCommands;

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
    Timer {
        #[command(subcommand)]
        command: TimerCommands,
    },
    Plan {
        #[command(subcommand)]
        command: PlanCommands,
    },
    Clear,
    View,
}

impl Executable for Commands {
    fn execute(&self, data: &mut Data) {
        match self {
            Commands::Timer { command } => command.execute(data),
            Commands::Plan { command } => command.execute(data),
            Commands::Clear => clear::exec(data),
            Commands::View => view::exec(data),
        }
    }
}
