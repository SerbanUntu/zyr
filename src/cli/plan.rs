use crate::{
    domain::{Data, Executable, TimeBlock},
    utils::parsers,
};
use chrono::{DateTime, Local};
use clap::Subcommand;
use std::error::Error;
use std::time::Duration;

#[derive(Subcommand, PartialEq)]
pub enum PlanCommands {
    Add {
        category: String,

        #[arg(short, long, value_parser = parsers::parse_timestamp)]
        from: DateTime<Local>,

        #[arg(short, long, value_parser = parsers::parse_duration)]
        duration: Option<Duration>,

        #[arg(short, long, value_parser = parsers::parse_timestamp)]
        to: Option<DateTime<Local>>,
    },
    Edit {
        order_number: u32,

        #[arg(short, long, action)]
        last: Option<bool>,
    },
    Del {
        order_number: u32,

        #[arg(short, long, action)]
        last: Option<bool>,
    },
}

impl Executable for PlanCommands {
    fn execute(&self, data: &mut Data) -> Result<(), Box<dyn Error>> {
        match self {
            PlanCommands::Add {
                category,
                from,
                duration,
                to,
            } => Self::exec_add(category, *from, *duration, *to, data)?,
            PlanCommands::Edit { .. } => todo!(),
            PlanCommands::Del { .. } => todo!(),
        }
        Ok(())
    }
}

impl PlanCommands {
    fn exec_add(
        category: &str,
        from: DateTime<Local>,
        duration: Option<Duration>,
        to: Option<DateTime<Local>>,
        data: &mut Data,
    ) -> Result<(), Box<dyn Error>> {
        let end_unix: u64;

        match (duration, to) {
            (None, None) => {
                return Err("Either the time block duration or end time must be set!".into());
            }
            (Some(_), Some(_)) => {
                return Err(
                "The time block duration and end time cannot both be set at the same time! Please choose only one of them.".into()
            );
            }
            (Some(d), None) => {
                end_unix = (from + d).timestamp_millis() as u64;
            }
            (None, Some(t)) => {
                end_unix = t.timestamp_millis() as u64;
            }
        }

        let tb = TimeBlock {
            start_unix: from.timestamp_millis() as u64,
            end_unix: Some(end_unix),
            category: category.to_string(),
        };
        data.blocks.push(tb);

        Ok(())
    }
}
