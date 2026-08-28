use crate::core::curriculum::Level;
use crate::core::generator::DrillItem;
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

        let mut has_not_done_comment = false;
        let mut metadata_str = None;
        let mut solution_str = None;
        let mut alternatives_str = None;
        let mut diagnostic_rules_str = None;
        let mut hints_str = None;

        for cap in comment_re.captures_iter(content) {
            if let Some(m) = cap.get(1) {
                let comment_body = m.as_str().trim();
                if comment_body == "I AM NOT DONE" {
                    has_not_done_comment = true;
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

        let is_done = !has_not_done_comment
            && !content.contains("___")
            && !content.contains("<!-- ANSWER -->");

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

    pub fn to_drill_items(&self) -> Vec<DrillItem> {
        let topic = if let Some(concept) = crate::core::reference::get_grammar_concept(&self.topic)
        {
            concept.slug.to_string()
        } else if let Some(concept) = self
            .concept_tags
            .iter()
            .find_map(|t| crate::core::reference::get_grammar_concept(t))
        {
            concept.slug.to_string()
        } else if !self.topic.is_empty() {
            self.topic.clone()
        } else {
            "general".to_string()
        };

        // Find the trigger sentence containing cloze blank ___
        let mut trigger_sentence = String::new();
        for line in self.raw_content.lines() {
            let trimmed = line.trim();
            if trimmed.contains("___") {
                trigger_sentence = trimmed.to_string();
                break;
            }
        }
        if trigger_sentence.is_empty() {
            let mut in_exercise = false;
            for line in self.raw_content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("### Exercise") {
                    in_exercise = true;
                    continue;
                }
                if in_exercise {
                    if trimmed.starts_with("<!--") || trimmed.is_empty() || trimmed.starts_with('#')
                    {
                        continue;
                    }
                    trigger_sentence = trimmed.to_string();
                    break;
                }
            }
        }
        if trigger_sentence.is_empty() {
            trigger_sentence = format!("{} ____", self.title);
        }

        // Extract target verb from prompt/parentheses or instructions if available
        let mut target_verb = String::new();
        if let Some(paren_start) = trigger_sentence.find('(') {
            if let Some(paren_end) = trigger_sentence[paren_start..].find(')') {
                let inside = &trigger_sentence[paren_start + 1..paren_start + paren_end];
                if !inside.contains("___") && inside.len() < 30 {
                    target_verb = inside.trim().to_string();
                }
            }
        }
        if target_verb.is_empty() {
            for line in self.raw_content.lines() {
                let trimmed = line.trim();
                if trimmed.contains("Conjugate the verb *(")
                    || trimmed.contains("Conjugate the verb (")
                    || trimmed.contains("Conjugate *(")
                    || trimmed.contains("Conjugate (")
                {
                    if let Some(start) = trimmed.find('(') {
                        if let Some(end) = trimmed[start..].find(')') {
                            let candidate = &trimmed[start + 1..start + end];
                            let cleaned = candidate.trim_matches(|c| c == '*' || c == ' ');
                            if !cleaned.is_empty() && cleaned.len() < 30 {
                                target_verb = cleaned.to_string();
                                break;
                            }
                        }
                    }
                }
            }
        }
        if target_verb.is_empty() {
            target_verb = "n/a".to_string();
        }

        // Extract target subject
        let mut target_subject = String::new();
        let common_subjects = [
            "yo", "tú", "vos", "él", "ella", "usted", "nosotros", "nosotras", "vosotros",
            "vosotras", "ellos", "ellas", "ustedes",
        ];
        for line in self.raw_content.lines() {
            let lower = line.to_lowercase();
            if lower.contains("person")
                || lower.contains("singular")
                || lower.contains("plural")
                || lower.contains("subject")
            {
                for &subj in &common_subjects {
                    let needle1 = format!("(*{subj}*)");
                    let needle2 = format!("({subj})");
                    let needle3 = format!("*{subj}*");
                    if lower.contains(&needle1)
                        || lower.contains(&needle2)
                        || lower.contains(&needle3)
                    {
                        target_subject = subj.to_string();
                        break;
                    }
                }
                if !target_subject.is_empty() {
                    break;
                }
            }
        }
        if target_subject.is_empty() {
            let lower_sentence = trigger_sentence.to_lowercase();
            for &subj in &common_subjects {
                if lower_sentence.starts_with(subj) {
                    target_subject = subj.to_string();
                    break;
                }
            }
        }
        if target_subject.is_empty() {
            target_subject = "n/a".to_string();
        }

        let target = if !self.solution.is_empty() {
            self.solution.clone()
        } else {
            "n/a".to_string()
        };

        let formula_cue = if let Some(ref contrast) = self.contrast_note {
            contrast.clone()
        } else if let Some(ref focus) = self.grammar_focus {
            focus.clone()
        } else if let Some(hint) = self.hints.first() {
            hint.clone()
        } else {
            self.title.clone()
        };

        let explanation = if let Some(ref focus) = self.grammar_focus {
            if let Some(ref contrast) = self.contrast_note {
                format!("{} ({})", focus, contrast)
            } else if let Some(hint) = self.hints.first() {
                let clean_hint = hint.strip_prefix("Tier 1:").unwrap_or(hint).trim();
                format!("{}. {}", focus, clean_hint)
            } else {
                focus.clone()
            }
        } else if let Some(hint) = self.hints.get(1).or_else(|| self.hints.first()) {
            hint.strip_prefix("Tier 1:")
                .or_else(|| hint.strip_prefix("Tier 2:"))
                .unwrap_or(hint)
                .trim()
                .to_string()
        } else if let Some(rule) = self.diagnostic_rules.first() {
            rule.message.clone()
        } else {
            format!("{}: {}", self.id, self.title)
        };

        let explanation = if explanation.is_empty() {
            format!("{}: {}", self.id, self.title)
        } else {
            explanation
        };

        vec![DrillItem {
            topic,
            formula_cue,
            trigger_sentence,
            target_verb,
            target_subject,
            target,
            explanation,
        }]
    }
}
