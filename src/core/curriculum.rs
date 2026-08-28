use crate::core::exercise::Exercise;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Level {
    Baseline,
    B1,
    B2,
    C1,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Level::Baseline => "Baseline",
            Level::B1 => "B1",
            Level::B2 => "B2",
            Level::C1 => "C1",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("invalid level: {0}")]
pub struct ParseLevelError(pub String);

impl FromStr for Level {
    type Err = ParseLevelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "Baseline" | "baseline" => Ok(Level::Baseline),
            "B1" | "b1" => Ok(Level::B1),
            "B2" | "b2" => Ok(Level::B2),
            "C1" | "c1" => Ok(Level::C1),
            other => Err(ParseLevelError(other.to_string())),
        }
    }
}

pub fn find_all_exercises<P: AsRef<Path>>(root: P) -> anyhow::Result<Vec<Exercise>> {
    let mut exercises = Vec::new();
    let root = root.as_ref();
    if !root.exists() {
        return Ok(exercises);
    }
    collect_md_files(root, &mut exercises)?;
    exercises.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(exercises)
}

pub fn find_all_exercises_or_embedded<P: AsRef<Path>>(root: P) -> anyhow::Result<Vec<Exercise>> {
    let root = root.as_ref();
    if root.exists() && root.is_dir() {
        let disk_exercises = find_all_exercises(root)?;
        if !disk_exercises.is_empty() {
            return Ok(disk_exercises);
        }
    }
    crate::core::embedded::get_embedded_exercises()
}

fn collect_md_files(dir: &Path, exercises: &mut Vec<Exercise>) -> anyhow::Result<()> {
    if dir.is_dir() {
        let mut entries: Vec<_> = fs::read_dir(dir)?.filter_map(|e| e.ok()).collect();
        entries.sort_by_key(|e| e.path());
        for entry in entries {
            let path = entry.path();
            if path.is_dir() {
                collect_md_files(&path, exercises)?;
            } else if path.extension().is_some_and(|ext| ext == "md") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(ex) = Exercise::from_markdown(&path, &content) {
                        exercises.push(ex);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn find_exercise_by_query<'a>(exercises: &'a [Exercise], query: &str) -> Option<&'a Exercise> {
    let q = query.trim().to_lowercase();
    // 1. Exact ID match
    if let Some(ex) = exercises.iter().find(|e| e.id.to_lowercase() == q) {
        return Some(ex);
    }
    // 2. Path substring match
    if let Some(ex) = exercises
        .iter()
        .find(|e| e.path.to_string_lossy().to_lowercase().contains(&q))
    {
        return Some(ex);
    }
    // 3. Topic or Title match
    exercises
        .iter()
        .find(|e| e.topic.to_lowercase().contains(&q) || e.title.to_lowercase().contains(&q))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Curriculum {
    pub exercises: Vec<Exercise>,
}

pub fn load_curriculum() -> anyhow::Result<Curriculum> {
    let exercises = find_all_exercises_or_embedded("exercises")?;
    Ok(Curriculum { exercises })
}
