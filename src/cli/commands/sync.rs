use crate::core::state::AppState;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct PortableStateBackup {
    pub version: String,
    pub exported_at: chrono::DateTime<chrono::Utc>,
    pub completed_count: usize,
    pub srs_items_count: usize,
    pub state: AppState,
}

pub fn export_state_json(state: &AppState) -> Result<String> {
    let backup = PortableStateBackup {
        version: env!("CARGO_PKG_VERSION").to_string(),
        exported_at: chrono::Utc::now(),
        completed_count: state.completed_exercises.len(),
        srs_items_count: state.srs.len(),
        state: state.clone(),
    };
    serde_json::to_string_pretty(&backup).context("Failed to serialize state backup JSON")
}

pub fn import_state_json(json_str: &str, current_state: &mut AppState) -> Result<usize> {
    let backup: PortableStateBackup =
        if let Ok(b) = serde_json::from_str::<PortableStateBackup>(json_str) {
            b
        } else {
            // Fallback to direct AppState JSON
            let direct_state: AppState = serde_json::from_str(json_str)
                .context("Invalid state JSON format (neither wrapped backup nor raw AppState)")?;
            PortableStateBackup {
                version: env!("CARGO_PKG_VERSION").to_string(),
                exported_at: chrono::Utc::now(),
                completed_count: direct_state.completed_exercises.len(),
                srs_items_count: direct_state.srs.len(),
                state: direct_state,
            }
        };

    let mut merged_items_count = 0;

    // Merge completed exercises
    for completed in backup.state.completed_exercises {
        if !current_state.is_completed(&completed) {
            current_state.completed_exercises.insert(completed);
            merged_items_count += 1;
        }
    }

    // Merge SRS items
    for (id, incoming_item) in backup.state.srs {
        if let Some(existing_item) = current_state.srs.get_mut(&id) {
            if incoming_item.repetitions > existing_item.repetitions
                || (incoming_item.repetitions == existing_item.repetitions
                    && incoming_item.last_reviewed > existing_item.last_reviewed)
            {
                *existing_item = incoming_item;
                merged_items_count += 1;
            }
        } else {
            current_state.srs.insert(id, incoming_item);
            merged_items_count += 1;
        }
    }

    // Merge daily activity heatmap logs
    for (date_str, count) in backup.state.activity_history {
        let entry = current_state.activity_history.entry(date_str).or_insert(0);
        if count > *entry {
            *entry = count;
        }
    }

    Ok(merged_items_count)
}

pub fn run_sync(export_path: Option<&str>, import_path: Option<&str>) -> Result<()> {
    if export_path.is_none() && import_path.is_none() {
        anyhow::bail!("Specify either --export <path> or --import <path> for state sync.");
    }

    if let Some(out_path) = export_path {
        let state = AppState::load().unwrap_or_default();
        let json_str = export_state_json(&state)?;
        fs::write(out_path, json_str)
            .with_context(|| format!("Failed to write state export to {}", out_path))?;
        println!("State successfully exported to: {}", out_path);
    }

    if let Some(in_path) = import_path {
        let json_str = fs::read_to_string(in_path)
            .with_context(|| format!("Failed to read state import file at {}", in_path))?;
        let mut state = AppState::load().unwrap_or_default();
        let count = import_state_json(&json_str, &mut state)?;
        state.save().context("Failed to save merged state")?;
        println!(
            "State successfully imported and merged ({} updates applied).",
            count
        );
    }

    Ok(())
}
