use std::collections::HashMap;

use chrono::Datelike;

use crate::models::Expense;


pub fn total_expenses(expenses: &[Expense]) -> u64{
    expenses
        .iter()
        .map(|expense| expense.amount_cents())
        .sum()
}

fn total_for_month(
    expenses: &[Expense], 
    year:i32, 
    month: u32
) -> u64{
    expenses
        .iter()
        .filter(|expense| {
            expense.date().year() == year
                && expense.date().month() == month
        })
        .map(|expense| expense.amount_cents())
        .sum()
    
}

fn total_by_category(expense: &[Expense]) -> HashMap<String, u64>{
    let mut totals = HashMap::new();

    for expense in expenses {
        let category = expense
            .category()
            .unwrap_or("Uncategorized")
            .to_string();
    
        *totals.entry(category).or_insert(0) += expense.amount_cents();
    }

    totals
}