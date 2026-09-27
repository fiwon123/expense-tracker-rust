use clap::Parser;
use crate::cli::{Cli, run};

mod cli;
mod models;
mod errors;
mod storage;

fn main() {
    let cli = Cli::parse();

    run(cli.command);
}
