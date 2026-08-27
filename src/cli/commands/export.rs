use crate::core::curriculum::find_all_exercises;
use crate::core::exercise::Exercise;
use crate::core::state::AppState;
use anyhow::{Context, Result};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn generate_anki_tsv(
    exercises: &[Exercise],
    state: &AppState,
    only_due: bool,
    level_filter: Option<&str>,
    topic_filter: Option<&str>,
) -> String {
    let mut out = String::new();
    out.push_str("#separator:tab\n#html:true\n#tags column:3\n");

    let now = chrono::Utc::now();

    for ex in exercises {
        if let Some(level) = level_filter {
            if !ex.level.to_string().eq_ignore_ascii_case(level) {
                continue;
            }
        }
        if let Some(topic) = topic_filter {
            if !ex.topic.eq_ignore_ascii_case(topic) {
                continue;
            }
        }
        if only_due {
            if let Some(item) = state.srs.get(&ex.id) {
                if item.next_review_due > now {
                    continue;
                }
            }
        }

        // Front
        let front = format!(
            "<div style='font-family: sans-serif;'><div style='color: #888; font-size: 0.9em; margin-bottom: 8px;'>[{level}] {topic} &mdash; {title}</div><div style='margin-bottom: 12px;'><b>Prompt:</b> {raw}</div></div>",
            level = ex.level,
            topic = ex.topic,
            title = html_escape(&ex.title),
            raw = html_escape(&ex.raw_content.lines().filter(|l| !l.starts_with("<!--") && !l.starts_with("#")).collect::<Vec<_>>().join("<br>"))
        );

        // Back
        let mut back = format!(
            "<div style='font-family: sans-serif;'><div style='color: #2e7d32; font-size: 1.2em; font-weight: bold; margin-bottom: 8px;'>{solution}</div>",
            solution = html_escape(&ex.solution)
        );

        if !ex.alternatives.is_empty() {
            back.push_str(&format!(
                "<div style='color: #555; margin-bottom: 8px;'><i>Alternatives:</i> {alts}</div>",
                alts = html_escape(&ex.alternatives.join(", "))
            ));
        }

        if !ex.hints.is_empty() {
            back.push_str("<div style='margin-top: 10px; font-size: 0.9em; border-top: 1px solid #ccc; padding-top: 6px;'><b>Hints:</b><ul>");
            for hint in &ex.hints {
                back.push_str(&format!("<li>{}</li>", html_escape(hint)));
            }
            back.push_str("</ul></div>");
        }
        back.push_str("</div>");

        // Tags
        let tags = format!(
            "spanglings {level} {topic}",
            level = ex.level,
            topic = ex.topic
        );

        let clean_front = front.replace(['\t', '\n'], " ");
        let clean_back = back.replace(['\t', '\n'], " ");
        out.push_str(&format!("{}\t{}\t{}\n", clean_front, clean_back, tags));
    }

    out
}

pub fn generate_markdown_notes(
    exercises: &[Exercise],
    _state: &AppState,
    level_filter: Option<&str>,
    topic_filter: Option<&str>,
) -> String {
    let mut out = String::new();
    out.push_str("# Spanglings Study Notes & Curriculum Guide\n\n");
    out.push_str("> Auto-generated comprehensive study guide covering grammar rules, exercises, and vocabulary.\n\n");

    let mut current_topic = String::new();

    for ex in exercises {
        if let Some(level) = level_filter {
            if !ex.level.to_string().eq_ignore_ascii_case(level) {
                continue;
            }
        }
        if let Some(topic) = topic_filter {
            if !ex.topic.eq_ignore_ascii_case(topic) {
                continue;
            }
        }

        if ex.topic != current_topic {
            current_topic = ex.topic.clone();
            out.push_str(&format!("## Topic: {} [{}]\n\n", current_topic, ex.level));
        }

        out.push_str(&format!("### {}\n\n", ex.title));
        out.push_str(&format!("- **ID**: `{}`\n", ex.id));
        out.push_str(&format!("- **Level**: {}\n", ex.level));
        out.push_str(&format!("- **Target Solution**: `{}`\n", ex.solution));
        if !ex.alternatives.is_empty() {
            out.push_str(&format!(
                "- **Accepted Alternatives**: `{}`\n",
                ex.alternatives.join("`, `")
            ));
        }
        if !ex.hints.is_empty() {
            out.push_str("- **Grammar Tips & Hints**:\n");
            for hint in &ex.hints {
                out.push_str(&format!("  - {}\n", hint));
            }
        }
        out.push_str("\n---\n\n");
    }

    out
}

pub fn generate_json_export(
    exercises: &[Exercise],
    state: &AppState,
    level_filter: Option<&str>,
    topic_filter: Option<&str>,
) -> Result<String> {
    let filtered: Vec<_> = exercises
        .iter()
        .filter(|ex| {
            if let Some(level) = level_filter {
                if !ex.level.to_string().eq_ignore_ascii_case(level) {
                    return false;
                }
            }
            if let Some(topic) = topic_filter {
                if !ex.topic.eq_ignore_ascii_case(topic) {
                    return false;
                }
            }
            true
        })
        .map(|ex| {
            let srs = state.srs.get(&ex.id);
            let is_completed = state.is_completed(&ex.id);
            serde_json::json!({
                "id": ex.id,
                "title": ex.title,
                "level": ex.level.to_string(),
                "topic": ex.topic,
                "solution": ex.solution,
                "alternatives": ex.alternatives,
                "hints": ex.hints,
                "is_completed": is_completed,
                "repetitions": srs.map(|s| s.repetitions).unwrap_or(0),
                "ease_factor": srs.map(|s| s.ease_factor).unwrap_or(2.5),
                "next_review": srs.map(|s| s.next_review_due.to_rfc3339()),
            })
        })
        .collect();

    serde_json::to_string_pretty(&filtered).context("Failed to serialize export json")
}

fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub fn run_export(
    format: &str,
    out: Option<&str>,
    level: Option<&str>,
    topic: Option<&str>,
    only_due: bool,
) -> Result<()> {
    let exercises_dir = Path::new("exercises");
    let exercises = find_all_exercises(exercises_dir)?;
    let state = AppState::load().unwrap_or_default();

    let output_content = match format.to_lowercase().as_str() {
        "anki" | "tsv" => generate_anki_tsv(&exercises, &state, only_due, level, topic),
        "markdown" | "md" => generate_markdown_notes(&exercises, &state, level, topic),
        "json" => generate_json_export(&exercises, &state, level, topic)?,
        unknown => {
            anyhow::bail!(
                "Unknown export format '{}'. Supported formats: 'anki', 'markdown', 'json'",
                unknown
            );
        }
    };

    if let Some(out_path) = out {
        fs::write(out_path, &output_content)
            .with_context(|| format!("Failed to write export to {}", out_path))?;
        println!("Export written successfully to: {}", out_path);
    } else {
        io::stdout().write_all(output_content.as_bytes())?;
    }

    Ok(())
}
