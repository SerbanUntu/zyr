use crate::{
    domain::{Data, Executable, TimeBlock},
    terminal::{FRAME_DURATION_MS, PAGE_SIZE, RawTerminal},
    utils::{io_utils, parsers, time_utils},
};
use chrono::{DateTime, Local};
use clap::{ArgAction, Subcommand};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{self, Color},
};
use std::time::Duration;
use std::{error::Error, io};

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
            (None, false) => Self::choose_index(data)?,
            (Some(0) | None, true) => 0,
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

    fn choose_index(data: &mut Data) -> Result<u32, Box<dyn Error>> {
        let _raw_terminal = RawTerminal::new()?;
        let frame_dur = Duration::from_millis(FRAME_DURATION_MS);
        let total_pages: usize =
            (f64::from(data.blocks.len() as u32) / f64::from(PAGE_SIZE as u32)).ceil() as usize;

        let lines: Vec<String> = data
            .blocks
            .iter()
            .rev()
            .enumerate()
            .map(|(i, b)| format!("{i}: {}", b))
            .collect();
        let mut page = 0;
        let mut pos: usize = 0;

        // Exclusive
        let mut max_pos: usize = PAGE_SIZE as usize;

        fn load_page(
            page: usize,
            pos: &mut usize,
            max_pos: &mut usize,
            total_pages: usize,
            lines: &[String],
        ) {
            let ps = PAGE_SIZE as usize;
            *max_pos = ps;
            if page * ps + *max_pos > lines.len() {
                *max_pos = lines.len() - page * ps;
            }
            if *pos > *max_pos {
                *pos = *max_pos - 1;
            }
            execute!(io::stdout(), cursor::MoveTo(0, 0),).unwrap();

            for i in 0..*max_pos {
                execute!(
                    io::stdout(),
                    cursor::MoveToColumn(0),
                    style::Print(format!("  {:<80}\n", lines[page * ps + i])),
                )
                .unwrap();
            }
            for _ in *max_pos..=PAGE_SIZE as usize {
                execute!(
                    io::stdout(),
                    cursor::MoveToColumn(0),
                    style::Print(format!("  {:<80}\n", "")),
                )
                .unwrap();
            }
            execute!(
                io::stdout(),
                cursor::MoveToColumn(0),
                style::Print(format!(
                    "Page {} of {total_pages}. Navigate with arrows or Vim motions. Enter to submit.",
                    page + 1
                )),
            )
            .unwrap();

            select_line(page, 0, *pos, lines);
        }

        fn select_line(page: usize, old_pos: usize, new_pos: usize, lines: &[String]) {
            execute!(
                io::stdout(),
                cursor::MoveToRow(old_pos as u16),
                cursor::MoveToColumn(0),
                style::SetForegroundColor(Color::Grey),
                style::Print(format!(
                    "  {}",
                    lines[page * (PAGE_SIZE as usize) + old_pos]
                )),
                cursor::MoveToRow(new_pos as u16),
                cursor::MoveToColumn(0),
                style::SetForegroundColor(Color::White),
                style::Print(format!(
                    "> {}",
                    lines[page * (PAGE_SIZE as usize) + new_pos]
                )),
                style::ResetColor,
            )
            .unwrap();
        }

        load_page(0, &mut pos, &mut max_pos, total_pages, &lines);

        loop {
            if event::poll(frame_dur)?
                && let Event::Key(e) = event::read()?
            {
                match (e.code, e.modifiers) {
                    (KeyCode::Char('c'), m) if m.contains(KeyModifiers::CONTROL) => {
                        return Err("Interrupt signal".into());
                    }
                    (KeyCode::Up | KeyCode::Char('k'), _) => {
                        if pos > 0 {
                            select_line(page, pos, pos - 1, &lines);
                            pos -= 1;
                        }
                    }
                    (KeyCode::Down | KeyCode::Char('j'), _) => {
                        if pos < max_pos - 1 {
                            select_line(page, pos, pos + 1, &lines);
                            pos += 1;
                        }
                    }
                    (KeyCode::Enter, _) => {
                        return Ok((pos + (page * (PAGE_SIZE as usize))) as u32);
                    }
                    (KeyCode::Left | KeyCode::Char('h'), _) => {
                        if page > 0 {
                            load_page(page - 1, &mut pos, &mut max_pos, total_pages, &lines);
                            page -= 1;
                        }
                    }
                    (KeyCode::Right | KeyCode::Char('l'), _) => {
                        if page < total_pages - 1 {
                            load_page(page + 1, &mut pos, &mut max_pos, total_pages, &lines);
                            page += 1;
                        }
                    }
                    _ => (),
                }
            }
        }
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
        if data.blocks.is_empty() {
            return Err("You do not have any time blocks stored. Create one with `zyr plan add` or `zyr timer start`".into());
        }
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
        if data.blocks.is_empty() {
            return Err("You do not have any time blocks stored. Create one with `zyr plan add` or `zyr timer start`".into());
        }

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
