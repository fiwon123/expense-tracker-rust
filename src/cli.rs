use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "expense-tracker")]
#[command(about = "A simple command-line expense tracker")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}


#[derive(Debug, Subcommand)]
pub enum Commands{

    Add{
        #[arg(short, long)]
        description: String,

        #[arg(short, long)]
        amount: f64,

        #[arg(short, long)]
        category: Option<String>,
    },
    List,
    Delete{
        #[arg(short, long)]
        id: u32,
    },
    Summary,
}

pub fn run(command: Commands){
    match command {
        Commands::Add{
            description,
            amount, 
            category,
        } => {
            println!("Adding expense");
            println!("Description: {description}");
            println!("Amount: {amount}");
            println!("Category: {category:?}");
        }

        Commands::List => {
            println!("Listing expenses");
        }

        Commands::Delete { id} => {
            println!("Deleting expense with ID: {id}");
        }

        Commands::Summary => {
            println!("showing summary");
        }
    }
}