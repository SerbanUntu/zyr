pub mod clear;
pub mod timer;
pub mod plan;

use clap::{Parser, Subcommand};
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
