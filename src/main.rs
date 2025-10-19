mod cli;
mod domain;
mod utils;

use clap::Parser;
use cli::Cli;
use domain::Data;
use domain::Executable;

fn main() {
    let cli = Cli::parse();
    let mut data = Data::from_file("data.json");

    let result = cli.command.execute(&mut data);
    if let Err(e) = result {
        eprintln!("Execution failed. {}", e)
    }

    data.save("data.json");
}

#[cfg(test)]
mod tests {}
