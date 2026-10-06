# Expense Tracker

A simple command-line expense tracker built in Rust. Add, list, delete, and summarize your expenses — data is stored in a local `expenses.json` file.

## Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- Cargo (comes with Rust)

Check your versions:

```bash
rustc --version
cargo --version
```

## Getting Started

```bash
# Clone the repository
git clone <your-repo-url>
cd expense-tracker-rust

# Build
cargo build

# Run
cargo run -- <command>
```

## Usage

### Add an expense

```bash
cargo run -- add -d "Coffee" -a 5.50 -c Food
```

| Flag | Description | Required |
|------|-------------|----------|
| `-d, --description` | What you spent on | Yes |
| `-a, --amount` | Amount (in dollars) | Yes |
| `-c, --category` | Category tag | No |

### List all expenses

```bash
cargo run -- list
```

Example output:

```
ID    Date         Amount     Description          Category
------------------------------------------------------------
1     2026-10-06   $5.50      Coffee               Food
2     2026-10-06   $1200.00   Rent                 Housing
```

### Delete an expense

```bash
cargo run -- delete -i 1
```

### View summary

Current month:

```bash
cargo run -- summary
```

Specific month/year:

```bash
cargo run -- summary -m 10 -y 2026
```

Example output:

```
Total for 10/2026: $1205.50

By category:
- Food: $5.50
- Housing: $1200.00
```

## Data Storage

Expenses are saved to `expenses.json` in the project root:

```json
[
  {
    "id": 1,
    "description": "Coffee",
    "amount_cents": 550,
    "date": "2026-10-06",
    "category": "Food"
  }
]
```

Delete this file to reset all data.

## Project Structure

```
src/
├── main.rs      # Entry point
├── cli.rs       # CLI parsing and command handling
├── models.rs    # Expense struct
├── storage.rs   # Load/save to JSON
├── reports.rs   # Totals and aggregations
└── errors.rs    # Custom error types
```

## Development

```bash
cargo check   # Type-check without building
cargo build   # Build
cargo run -- <command>  # Run
cargo clippy  # Lint (requires: rustup component add clippy)
cargo fmt     # Format (requires: rustup component add rustfmt)
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| clap | CLI argument parsing |
| serde / serde_json | JSON serialization |
| chrono | Date handling |
| anyhow / thiserror | Error handling |
