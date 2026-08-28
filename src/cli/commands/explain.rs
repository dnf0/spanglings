use crate::core::reference::{get_grammar_concept, list_grammar_concepts};
use colored::Colorize;

pub fn show_explanation(topic: Option<&str>) -> anyhow::Result<()> {
    match topic {
        Some(query) if !query.trim().is_empty() => match get_grammar_concept(query) {
            Some(concept) => {
                println!("{}", concept.card.cyan());
                Ok(())
            }
            None => {
                println!(
                    "{}",
                    format!("Unknown grammar topic or intent: '{}'", query)
                        .red()
                        .bold()
                );
                print_topic_catalog();
                Ok(())
            }
        },
        _ => {
            print_topic_catalog();
            Ok(())
        }
    }
}

fn print_topic_catalog() {
    println!(
        "\n{}",
        "Available Grammar Reference Cards:".bold().underline()
    );
    for c in list_grammar_concepts() {
        println!(
            "  • {:<24} {}{}{}",
            c.slug.yellow().bold(),
            c.title.bold(),
            " — ".bright_black(),
            c.gloss.dimmed()
        );
    }
    println!(
        "\n{}\n",
        "Usage: spanglings explain <topic-or-intent> (e.g. spanglings explain wishes)".cyan()
    );
}
