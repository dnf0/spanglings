use crate::core::reference::{get_reference_card, list_reference_topics};
use colored::Colorize;

pub fn show_explanation(topic: &str) -> anyhow::Result<()> {
    match get_reference_card(topic) {
        Some(card) => {
            println!("{}", card.cyan());
            Ok(())
        }
        None => {
            println!(
                "{}",
                format!("Unknown grammar topic: '{}'", topic).red().bold()
            );
            println!("\nAvailable reference topics:");
            for t in list_reference_topics() {
                println!("  - {}", t.yellow());
            }
            Ok(())
        }
    }
}
