use clap::{Parser, Subcommand};
use crate::reports::{total_by_category, total_expenses};
use crate::storage::load_expenses;

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
    let expenses = load_expenses()?;

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
            let total_cents = total_expenses(&expenses);
            let total = total_cents as f64 / 100.0;

            println!("Total expenses: ${total:.2}");

            println!("\nExpenses by category:");

            let category_totals = total_by_category(&expenses);

            for (category, amount_cents) in category_totals {
                let amount = amount_cents as f64 / 100.0;

                println!("- {category}: ${amount:.2}");
            }

        }
    }
}