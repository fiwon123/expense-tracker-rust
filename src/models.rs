use std::collections::HashMap;
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

impl Expense {
    pub(crate) fn new(
        id: u32,
        description: String,
        amount_cents: u64,
        date: NaiveDate,
        category: Option<String>,
    ) -> Self {
        Self {
            id,
            description,
            amount_cents,
            date,
            category,
        }
    }

    pub(crate) fn id(&self) -> u32 {
        self.id
    }

    pub(crate) fn amount_cents(&self) -> u64 {
        self.amount_cents
    }

    pub(crate) fn date(&self) -> NaiveDate {
        self.date
    }

    pub(crate) fn category(&self) -> Option<&str> {
        self.category.as_deref()
    }

    pub(crate) fn description(&self) -> &str {
        &self.description
    }

    
}