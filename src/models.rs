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