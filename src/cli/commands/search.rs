use crate::core::curriculum::find_all_exercises_or_embedded;
use crate::core::exercise::Exercise;
use colored::Colorize;

pub fn search_exercises(query: &str) -> anyhow::Result<Vec<Exercise>> {
    let all = find_all_exercises_or_embedded("exercises")?;
    let q = query.trim().to_lowercase();
    let matches: Vec<Exercise> = all
        .into_iter()
        .filter(|e| {
            e.id.to_lowercase().contains(&q)
                || e.title.to_lowercase().contains(&q)
                || e.topic.to_lowercase().contains(&q)
                || e.level.to_string().to_lowercase() == q
                || e.solution.to_lowercase().contains(&q)
                || e.raw_content.to_lowercase().contains(&q)
                || e.hints.iter().any(|h| h.to_lowercase().contains(&q))
        })
        .collect();
    Ok(matches)
}

pub fn run_search(query: &str, json: bool) -> anyhow::Result<()> {
    let results = search_exercises(query)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&results)?);
        return Ok(());
    }

    if results.is_empty() {
        println!("No exercises found matching query: '{}'", query.yellow());
        return Ok(());
    }

    println!(
        "Found {} exercise(s) matching '{}':\n",
        results.len().to_string().bold().green(),
        query.cyan()
    );
    for ex in results {
        println!(
            "  • [{}] {} (ID: {}) - Topic: {}",
            ex.level.to_string().magenta().bold(),
            ex.title.bold(),
            ex.id.cyan(),
            ex.topic.yellow()
        );
        println!("    Path: {}", ex.path.display().to_string().dimmed());
    }
    Ok(())
}
