use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExpenseError {
    
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    
    #[error("Storage error: {0}")]
    Storage(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}