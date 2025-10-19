use crate::{
    domain::{Data, Executable, TimeBlock},
    utils::{io_utils, parsers, time_utils},
};
use chrono::{DateTime, Local};
use clap::{ArgAction, Subcommand};
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
        #[arg(short, long)]
        category: Option<String>,

        #[arg(short, long, value_parser = parsers::parse_timestamp)]
        from: Option<DateTime<Local>>,

        #[arg(short, long, value_parser = parsers::parse_duration)]
        duration: Option<Duration>,

        #[arg(short, long, value_parser = parsers::parse_timestamp)]
        to: Option<DateTime<Local>>,

        order_number: Option<u32>,

        #[arg(short, long, action = ArgAction::SetTrue)]
        last: bool,
    },
    Del {
        order_number: Option<u32>,

        #[arg(short, long, action = ArgAction::SetTrue)]
        last: bool,
    },
}

impl Executable for PlanCommands {
    fn execute(&self, data: &mut Data) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Add {
                category,
                from,
                duration,
                to,
            } => Self::exec_add(category, *from, *duration, *to, data)?,
            Self::Edit {
                category,
                from,
                duration,
                to,
                order_number,
                last,
            } => Self::exec_edit(category, *from, *duration, *to, order_number, *last, data)?,
            Self::Del { order_number, last } => Self::exec_del(order_number, *last, data)?,
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
        let end_unix = match (duration, to) {
            (None, None) => {
                return Err("Either the time block duration or end time must be set!".into());
            }
            (Some(_), Some(_)) => {
                return Err(
                "The time block duration and end time cannot both be set at the same time! Please choose only one of them.".into()
            );
            }
            (Some(d), None) => (from + d).timestamp_millis() as u64,
            (None, Some(t)) => t.timestamp_millis() as u64,
        };

        let tb = TimeBlock {
            start_unix: from.timestamp_millis() as u64,
            end_unix: Some(end_unix),
            category: category.to_string(),
        };
        data.blocks.push(tb);

        Ok(())
    }

    fn get_index_from_order_number(
        order_number: &Option<u32>,
        last: bool,
        data: &mut Data,
    ) -> Result<usize, Box<dyn Error>> {
        let computed_order = match (*order_number, last) {
            (None, false) => todo!(),
            (Some(0), true) | (None, true) => 0,
            (_, true) => {
                return Err("Mismatched order numbers! Either manually provide the order number or use --last, but not both at the same time.".into());
            }
            (Some(x), false) => x,
        };
        (data.blocks.len()).checked_sub(1 + computed_order as usize).ok_or_else(
            || format!(
                "This order number does not exist. The number you selected was {}, which is greater than the total number of blocks, which is {}", 
                computed_order,
                data.blocks.len()).into())
    }

    fn exec_edit(
        category: &Option<String>,
        from: Option<DateTime<Local>>,
        duration: Option<Duration>,
        to: Option<DateTime<Local>>,
        order_number: &Option<u32>,
        last: bool,
        data: &mut Data,
    ) -> Result<(), Box<dyn Error>> {
        if category.is_none() && from.is_none() && duration.is_none() && to.is_none() {
            return Err("No modifications were provided. Try running zyr plan edit --help to see the intended usage.".into());
        }

        let index = Self::get_index_from_order_number(order_number, last, data)?;
        let target_block = &mut data.blocks[index];

        let start_unix = match from {
            Some(dt) => dt.timestamp_millis() as u64,
            None => target_block.start_unix,
        };
        let end_unix = match (duration, to) {
            (None, None) => target_block.end_unix,
            (Some(_), Some(_)) => {
                return Err(
                "The time block duration and end time cannot both be set at the same time! Please choose only one of them.".into()
            );
            }
            (Some(d), None) => {
                Some((time_utils::convert(start_unix) + d).timestamp_millis() as u64)
            }
            (None, Some(t)) => Some(t.timestamp_millis() as u64),
        };

        target_block.start_unix = start_unix;
        target_block.end_unix = end_unix;
        if let Some(c) = category {
            target_block.category = c.to_string();
        }

        Ok(())
    }

    fn exec_del(
        order_number: &Option<u32>,
        last: bool,
        data: &mut Data,
    ) -> Result<(), Box<dyn Error>> {
        let index = Self::get_index_from_order_number(order_number, last, data)?;
        if io_utils::confirm("delete this time block") {
            data.blocks.remove(index);
            println!("Time block removed successfully");
        } else {
            println!("Time block was not removed");
        }

        Ok(())
    }
}
