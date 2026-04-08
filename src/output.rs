use serde::Serialize;
use tabled::{Table, Tabled};

/// Output either a human-friendly table or raw JSON.
pub fn print_table<T: Tabled + Serialize>(items: &[T], json: bool) {
    if json {
        match serde_json::to_string_pretty(items) {
            Ok(j) => println!("{}", j),
            Err(e) => eprintln!("Failed to serialize JSON: {}", e),
        }
    } else if items.is_empty() {
        println!("No results found.");
    } else {
        let table = Table::new(items).to_string();
        println!("{}", table);
    }
}

/// Output a single item as a table or JSON.
pub fn print_single<T: Tabled + Serialize>(item: &T, json: bool) {
    if json {
        match serde_json::to_string_pretty(item) {
            Ok(j) => println!("{}", j),
            Err(e) => eprintln!("Failed to serialize JSON: {}", e),
        }
    } else {
        let table = Table::new(std::iter::once(item)).to_string();
        println!("{}", table);
    }
}

/// Format a money value for display (e.g., "$123.45 AUD").
pub fn format_money(value: &str, currency: &str) -> String {
    format!("{} {}", value, currency)
}

/// Truncate a string to a max length, appending "..." if truncated.
pub fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max.saturating_sub(3)])
    }
}
