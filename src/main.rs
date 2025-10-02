mod cli;
mod utils;
mod domain;

use cli::Cli;
use domain::{Data};
use clap::Parser;
use domain::Executable;

fn main() {
    let cli = Cli::parse();
    let mut data = Data::from_file("data.json");

    cli.command.execute(&mut data);

    data.save("data.json");
}

#[cfg(test)]
mod tests {
}
