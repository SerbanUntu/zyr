use std::time::Duration;
use clap::Subcommand;
use crate::utils::parsers;

#[derive(Subcommand, PartialEq)]
pub enum TimerCommands {
    Start { 
        category: String, 

        #[arg(short, long, value_parser = parsers::parse_duration)]
        duration: Option<Duration>,
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
