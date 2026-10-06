use clap::Parser;
use crate::cli::{Cli, run};

mod cli;
mod models;
mod errors;
mod storage;
mod reports;

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    run(cli.command)
}
