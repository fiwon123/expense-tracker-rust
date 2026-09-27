use std::collections::HashMap;

use anyhow::Ok;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Expense{
    id: u32,
    description: String,
    amount_cents: u64,
    date: NaiveDate,
    category: Option<String>,
}

fn total_expenses(expenses: &[Expense]) -> u64{
    unimplemented!()
}

fn total_for_month(expenses: &[Expense], year:i32, month: u32) -> u64{
    unimplemented!()
}

fn total_by_category(expense: &[Expense]) -> HashMap<String, u64>{
    unimplemented!()
}