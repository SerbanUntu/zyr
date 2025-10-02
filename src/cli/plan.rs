use std::time::Duration;
use chrono::{DateTime, Local};
use clap::Subcommand;
use crate::{domain::{Executable, Data}, utils::parsers};

#[derive(Subcommand, PartialEq)]
pub enum PlanCommands {
    Add { 
        category: String, 

        #[arg(short, long)]
        from: DateTime<Local>,

        #[arg(short, long, value_parser = parsers::parse_duration)]
        duration: Option<Duration>,

        #[arg(short, long)]
        to: Option<DateTime<Local>>,
    },
    Edit {
        #[arg(short, long, action)]
        last: Option<bool>,
    },
    Del {
        #[arg(short, long, action)]
        last: Option<bool>,
    },
}

impl Executable for PlanCommands {
    fn execute(&self, _: &mut Data) {
        match self {
            PlanCommands::Add { .. } => todo!(),
            PlanCommands::Edit { .. } => todo!(),
            PlanCommands::Del { .. } => todo!(),
        }
    }
}
