use crate::core::curriculum::Level;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExerciseType {
    Cloze,
    Transformation,
    BugFix,
    Translation,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("invalid exercise type: {0}")]
pub struct ParseExerciseTypeError(pub String);

impl FromStr for ExerciseType {
    type Err = ParseExerciseTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "cloze" => Ok(ExerciseType::Cloze),
            "transformation" => Ok(ExerciseType::Transformation),
            "bugfix" | "bug_fix" => Ok(ExerciseType::BugFix),
            "translation" => Ok(ExerciseType::Translation),
            other => Err(ParseExerciseTypeError(other.to_string())),
        }
    }
}

impl fmt::Display for ExerciseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExerciseType::Cloze => "Cloze",
            ExerciseType::Transformation => "Transformation",
            ExerciseType::BugFix => "BugFix",
            ExerciseType::Translation => "Translation",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticRule {
    pub pattern: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exercise {
    pub path: PathBuf,
    pub id: String,
    pub level: Level,
    pub topic: String,
    pub exercise_type: ExerciseType,
    pub is_done: bool,
    pub title: String,
    pub solution: String,
    pub alternatives: Vec<String>,
    pub diagnostic_rules: Vec<DiagnosticRule>,
    pub hints: Vec<String>,
    pub raw_content: String,
    #[serde(default)]
    pub concept_tags: Vec<String>,
    #[serde(default)]
    pub prerequisites: Vec<String>,
    #[serde(default)]
    pub grammar_focus: Option<String>,
    #[serde(default)]
    pub contrast_note: Option<String>,
}

static COMMENT_RE: OnceLock<regex::Regex> = OnceLock::new();

fn split_metadata_line(line: &str) -> Vec<&str> {
    if !line.contains('|') {
        return vec![line];
    }
    let mut parts = Vec::new();
    let mut in_quotes = false;
    let mut quote_char = ' ';
    let mut in_brackets = false;
    let mut start = 0;
    let bytes = line.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        if b == b'"' || b == b'\'' {
            if !in_quotes {
                in_quotes = true;
                quote_char = b as char;
            } else if quote_char == b as char {
                in_quotes = false;
            }
        } else if b == b'[' && !in_quotes {
            in_brackets = true;
        } else if b == b']' && !in_quotes {
            in_brackets = false;
        } else if b == b'|' && !in_quotes && !in_brackets {
            parts.push(&line[start..i]);
            start = i + 1;
        }
    }
    parts.push(&line[start..]);
    parts
}

fn parse_string_list(val: &str) -> Vec<String> {
    let val = val.trim();
    if val.is_empty() || val == "[]" {
        return Vec::new();
    }
    let inner = if let Some(stripped) = val.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        stripped.trim()
    } else {
        val
    };

    if inner.is_empty() {
        return Vec::new();
    }

    inner
        .split(',')
        .map(|item| item.trim().trim_matches(|c| c == '"' || c == '\'').trim())
        .filter(|item| !item.is_empty())
        .map(|item| item.to_string())
        .collect()
}

fn parse_optional_string(val: &str) -> Option<String> {
    let trimmed = val.trim();
    let unquoted = trimmed.trim_matches(|c| c == '"' || c == '\'').trim();
    if unquoted.is_empty() {
        None
    } else {
        Some(unquoted.to_string())
    }
}

impl Exercise {
    pub fn from_markdown<P: AsRef<Path>>(path: P, content: &str) -> anyhow::Result<Self> {
        let comment_re =
            COMMENT_RE.get_or_init(|| regex::Regex::new(r"(?s)<!--([\s\S]*?)-->").unwrap());

        let mut is_done = true;
        let mut metadata_str = None;
        let mut solution_str = None;
        let mut alternatives_str = None;
        let mut diagnostic_rules_str = None;
        let mut hints_str = None;

        for cap in comment_re.captures_iter(content) {
            if let Some(m) = cap.get(1) {
                let comment_body = m.as_str().trim();
                if comment_body == "I AM NOT DONE" {
                    is_done = false;
                } else if let Some(stripped) = comment_body.strip_prefix("SOLUTION") {
                    solution_str = Some(stripped.trim_start_matches(':').trim());
                } else if let Some(stripped) = comment_body.strip_prefix("ALTERNATIVES") {
                    alternatives_str = Some(stripped.trim_start_matches(':').trim());
                } else if let Some(stripped) = comment_body.strip_prefix("DIAGNOSTIC_RULES") {
                    diagnostic_rules_str = Some(stripped.trim_start_matches(':').trim());
                } else if let Some(stripped) = comment_body.strip_prefix("HINTS") {
                    hints_str = Some(stripped.trim_start_matches(':').trim());
                } else if comment_body.contains("id:") && comment_body.contains("level:") {
                    metadata_str = Some(comment_body);
                }
            }
        }

        let mut id = None;
        let mut level = None;
        let mut topic = None;
        let mut exercise_type = None;
        let mut title_from_meta = None;
        let mut concept_tags = Vec::new();
        let mut prerequisites = Vec::new();
        let mut grammar_focus = None;
        let mut contrast_note = None;

        if let Some(meta_str) = metadata_str {
            for line in meta_str.lines() {
                let trimmed_line = line.trim();
                if trimmed_line.is_empty() {
                    continue;
                }
                for part in split_metadata_line(trimmed_line) {
                    if let Some((k, v)) = part.split_once(':') {
                        let key = k.trim();
                        let val = v.trim();
                        match key {
                            "id" => {
                                id = Some(
                                    val.trim_matches(|c| c == '"' || c == '\'')
                                        .trim()
                                        .to_string(),
                                )
                            }
                            "level" => {
                                let unquoted = val.trim_matches(|c| c == '"' || c == '\'').trim();
                                level = Some(unquoted.parse::<Level>()?);
                            }
                            "topic" => {
                                topic = Some(
                                    val.trim_matches(|c| c == '"' || c == '\'')
                                        .trim()
                                        .to_string(),
                                )
                            }
                            "type" => {
                                let unquoted = val.trim_matches(|c| c == '"' || c == '\'').trim();
                                exercise_type = Some(unquoted.parse::<ExerciseType>()?);
                            }
                            "title" => title_from_meta = parse_optional_string(val),
                            "concepts" | "concept_tags" => {
                                concept_tags = parse_string_list(val);
                            }
                            "prerequisites" => {
                                prerequisites = parse_string_list(val);
                            }
                            "grammar_focus" => {
                                grammar_focus = parse_optional_string(val);
                            }
                            "contrast_note" => {
                                contrast_note = parse_optional_string(val);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        let id = id.ok_or_else(|| anyhow::anyhow!("Missing 'id' in exercise metadata"))?;
        let level = level.ok_or_else(|| anyhow::anyhow!("Missing 'level' in exercise metadata"))?;
        let topic = topic.ok_or_else(|| anyhow::anyhow!("Missing 'topic' in exercise metadata"))?;
        let exercise_type =
            exercise_type.ok_or_else(|| anyhow::anyhow!("Missing 'type' in exercise metadata"))?;

        let title = if let Some(t) = title_from_meta.filter(|s| !s.is_empty()) {
            t
        } else {
            let mut t = String::new();
            for line in content.lines() {
                let line = line.trim();
                if let Some(stripped) = line.strip_prefix("# ") {
                    t = stripped.trim().to_string();
                    break;
                }
            }
            t
        };

        let solution = solution_str.unwrap_or_default().trim().to_string();

        let mut alternatives = Vec::new();
        if let Some(alt_str) = alternatives_str {
            for line in alt_str.lines() {
                let line = line.trim();
                if !line.is_empty() && line != "[]" {
                    alternatives.push(line.to_string());
                }
            }
        }

        let mut diagnostic_rules = Vec::new();
        if let Some(rules_str) = diagnostic_rules_str {
            for line in rules_str.lines() {
                let line = line.trim();
                if line.is_empty() || line == "[]" {
                    continue;
                }
                let mut pattern = String::new();
                let mut code = String::new();
                let mut message = String::new();
                for part in line.split('|') {
                    if let Some((k, v)) = part.split_once(':') {
                        let key = k.trim();
                        let val = v.trim();
                        let val_stripped = val.trim_matches(|c| c == '"' || c == '\'').to_string();
                        match key {
                            "pattern" => pattern = val_stripped,
                            "code" => code = val_stripped,
                            "message" => message = val_stripped,
                            _ => {}
                        }
                    }
                }
                if !pattern.is_empty() {
                    diagnostic_rules.push(DiagnosticRule {
                        pattern,
                        code,
                        message,
                    });
                }
            }
        }

        let mut hints = Vec::new();
        if let Some(h_str) = hints_str {
            for line in h_str.lines() {
                let line = line.trim();
                if !line.is_empty() && line != "[]" {
                    hints.push(line.to_string());
                }
            }
        }

        Ok(Exercise {
            path: path.as_ref().to_path_buf(),
            id,
            level,
            topic,
            exercise_type,
            is_done,
            title,
            solution,
            alternatives,
            diagnostic_rules,
            hints,
            raw_content: content.to_string(),
            concept_tags,
            prerequisites,
            grammar_focus,
            contrast_note,
        })
    }
}
