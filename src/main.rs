mod cli;
mod domain;
mod terminal;
mod utils;

use clap::Parser;
use cli::Cli;
use domain::Data;
use domain::Executable;

use crate::utils::file_utils;

fn main() {
    let data_path = file_utils::get_data_path();
    let mut data = Data::from_file(&data_path);
    let cli = Cli::parse();

    let result = cli.command.execute(&mut data);
    if let Err(e) = result {
        eprintln!("Execution failed. {}", e)
    }

    data.save(&data_path);
}

#[cfg(test)]
mod tests {}
