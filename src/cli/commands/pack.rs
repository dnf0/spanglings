use crate::core::exercise::Exercise;
use crate::engine::accents::AccentMode;
use crate::engine::validator::validate_submission;
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};

pub fn run_pack_create(name: &str) -> anyhow::Result<()> {
    let slug = name.trim().to_lowercase().replace(' ', "_");
    let base_dir = if Path::new("exercises").is_dir() {
        PathBuf::from("exercises").join(&slug)
    } else {
        PathBuf::from(&slug)
    };

    if base_dir.exists() {
        println!(
            "{} Directory '{}' already exists.",
            "•".yellow(),
            base_dir.display().to_string().bold()
        );
        return Ok(());
    }

    fs::create_dir_all(&base_dir)?;

    // Create 3 starter exercise templates
    for i in 1..=3 {
        let file_name = format!("{:02}_{}.md", i, slug);
        let file_path = base_dir.join(&file_name);
        let id_tag = format!("{}_{:02}", slug, i);
        let content = format!(
            r#"<!-- I AM NOT DONE -->
# {track_title}: Exercise {num:02}
<!-- id: {id} | level: B2 | topic: {slug} | type: cloze -->

> **Grammar Rule**: Custom curriculum exercise for {track_title}.

### Context
English: "In this scenario, we proposed the optimal solution."

### Exercise
Nosotros ___ la solución más adecuada para el proyecto.

<!-- SOLUTION
propusimos
-->

<!-- ALTERNATIVES
hemos propuesto
-->

<!-- DIAGNOSTIC_RULES
pattern: "proponemos" | code: "E0001" | message: "Use past preterite 'propusimos', not present 'proponemos'."
-->

<!-- HINTS
Tier 1: Use the irregular preterite stem of proponer.
Tier 2: The root changes from propon- to propus-.
Tier 3: The answer is 'propusimos'.
-->
"#,
            track_title = name,
            num = i,
            id = id_tag,
            slug = slug
        );
        fs::write(&file_path, content)?;
    }

    println!(
        "{} Created custom curriculum pack '{}' at: {}",
        "✓".green().bold(),
        name.bold(),
        base_dir.display().to_string().cyan().bold()
    );
    println!("  Added 3 starter markdown exercises ready for editing.");
    println!("  Run 'spanglings pack validate {}' to check syntax.", base_dir.display());
    Ok(())
}

pub fn run_pack_validate(path_str: &str) -> anyhow::Result<bool> {
    let target_path = Path::new(path_str);
    if !target_path.exists() {
        println!("{} Path '{}' does not exist.", "✗".red().bold(), path_str);
        return Ok(false);
    }

    let mut files = Vec::new();
    if target_path.is_file() {
        if target_path.extension().and_then(|e| e.to_str()) == Some("md") {
            files.push(target_path.to_path_buf());
        }
    } else {
        collect_markdown_files(target_path, &mut files)?;
    }

    if files.is_empty() {
        println!("{} No markdown exercises found in '{}'.", "•".yellow(), path_str);
        return Ok(true);
    }

    println!(
        "\n{} Validating {} exercise file(s) in '{}'...\n",
        "🔍".bold(),
        files.len().to_string().cyan().bold(),
        path_str.bold()
    );

    let mut errors_count = 0;
    for file in &files {
        let relative_display = file.display().to_string();
        let content = fs::read_to_string(file)?;

        match Exercise::from_markdown(file, &content) {
            Ok(exercise) => {
                let mut issues = Vec::new();
                if exercise.solution.is_empty() {
                    issues.push("Missing <!-- SOLUTION ... --> block".to_string());
                }
                if exercise.title.is_empty() {
                    issues.push("Missing title header (# Title)".to_string());
                }
                if exercise.topic.is_empty() {
                    issues.push("Missing topic metadata in <!-- id: ... | topic: ... -->".to_string());
                }

                // Verify solvability with the declared primary solution
                if !exercise.solution.is_empty() {
                    let validation = validate_submission(&exercise, &exercise.solution, AccentMode::Forgiving);
                    if !validation.is_success() {
                        issues.push(format!(
                            "Primary solution '{}' fails validator test",
                            exercise.solution
                        ));
                    }
                }

                if issues.is_empty() {
                    println!("  {} {:<40} [{}]", "✓".green().bold(), relative_display, exercise.level);
                } else {
                    errors_count += 1;
                    println!("  {} {:<40}", "✗".red().bold(), relative_display.red());
                    for issue in issues {
                        println!("      {} {}", "↳".red(), issue);
                    }
                }
            }
            Err(e) => {
                errors_count += 1;
                println!(
                    "  {} {:<40}\n      {} Syntax error: {}",
                    "✗".red().bold(),
                    relative_display.red(),
                    "↳".red(),
                    e
                );
            }
        }
    }

    println!();
    if errors_count == 0 {
        println!(
            "{} All {} exercise(s) are valid and ready for learning!\n",
            "✨".bold(),
            files.len().to_string().green().bold()
        );
        Ok(true)
    } else {
        println!(
            "{} Found {} invalid exercise file(s).\n",
            "✗".red().bold(),
            errors_count.to_string().red().bold()
        );
        Ok(false)
    }
}

fn collect_markdown_files(dir: &Path, files: &mut Vec<PathBuf>) -> anyhow::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                collect_markdown_files(&path, files)?;
            } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
                files.push(path);
            }
        }
    }
    Ok(())
}
