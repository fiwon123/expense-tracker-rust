use crate::models::Expense;



fn load_expenses() -> Result<Vec<Expense>, anyhow::Error> {
    Ok(vec![])
}

fn save_expenses(expenses: &[Expense]) -> Result<(), anyhow::Error> {
    let _ = expenses;
    Ok(())
}