use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands{
    Add{
        description: String,
        amount: f64,
        #[arg(long)]
        category: Option<String>,
    },
    List,
    Delete{
        id: u32,
    },
    Summary,
}