use chrono::Datelike;
use clap::{Parser, Subcommand};
use crate::errors::ExpenseError;
use crate::models::Expense;
use crate::reports::{self, total_by_category, total_expenses};
use crate::storage::{load_expenses, save_expenses};

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
    Summary {
        #[arg(short, long)]
        month: Option<u32>,
        #[arg(short, long)]
        year: Option<i32>,
    },
}

pub fn run(command: Commands) -> Result<(), anyhow::Error>{
    match command {
        Commands::Add{
            description,
            amount, 
            category,
        } => {

            // Validate
            if amount <= 0.0 {
                return Err(ExpenseError::InvalidAmount("Amount must be positive".into()).into());
            }
            if description.trim().is_empty() {
                return Err(ExpenseError::InvalidAmount("Description cannot be empty".into()).into());
            }

            let mut expenses = load_expenses()?;
            let next_id = expenses.iter().map(|e| e.id()).max().unwrap_or(0) + 1;
            let amount_cents = (amount * 100.0).round() as u64;

            let expense = Expense::new(
                next_id,
                description,
                amount_cents,
                chrono::Local::now().date_naive(),
                category,
            );

            expenses.push(expense);
            save_expenses(&expenses)?;
            println!("Expense added successfully!");
        }

        Commands::List => {
            let expenses = load_expenses()?;
            if expenses.is_empty() {
                println!("No expenses found.");
                return Ok(());
            }

            println!("{:<5} {:<12} {:<10}  {:<20} {}", "ID", "Date", "Amount", "Description", "Category");
            println!("{}", "-".repeat(60));

            for expense in expenses {
                let category = expense.category().unwrap_or("Uncategorized");
                println!("{:<5} {:<12} ${:<9.2} {:<20} {}",
                    expense.id(),
                    expense.date(),
                    expense.amount_cents() as f64 / 100.0,
                    expense.description(),
                    category
                );
            }
        }


        Commands::Delete { id} => {
            let mut expenses = load_expenses()?;
            let len_before = expenses.len();
            expenses.retain(|e| e.id() != id);

            if expenses.len() == len_before {
                println!("Expense with ID {} not found.", id);
            } else {
                save_expenses(&expenses)?;
                println!("Expense {} deleted.", id);
            }
        }

        Commands::Summary {month ,year}=> {
            let expenses = load_expenses()?;
            let now = chrono::Local::now();
            let month = month.unwrap_or(now.month());
            let year = year.unwrap_or(now.year());
            

            let total = reports::total_for_month(&expenses, year, month);
            println!("Total for {}/{}: ${:.2}", month, year, total as f64 / 100.0);

            println!("\nBy category:");
            for (category, amount_cents) in reports::total_by_category_for_month(&expenses, year, month) {
                println!("- {}: ${:.2}", category, amount_cents as f64 / 100.0);
            }
        }
    }

    Ok(())
}