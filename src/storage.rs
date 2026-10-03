use crate::models::Expense;
use std::{fs, path::Path};

const FILE_PATH: &str = "expenses.json";


pub fn load_expenses() -> Result<Vec<Expense>, anyhow::Error> {
    if !Path::new(FILE_PATH).exists() {
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(FILE_PATH)?;

    let expenses: Vec<Expense> = serde_json::from_str(&contents)?;

    Ok(expenses)
}

fn save_expenses(
    expenses: &[Expense]
) -> Result<(), anyhow::Error> {
    let json = serde_json::to_string_pretty(expenses)?;

    fs::write(FILE_PATH, json)?;

    Ok(())
}