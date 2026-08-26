use crate::core::exercise::Exercise;
use crate::engine::accents::{check_accent_match, AccentMode, AccentResult};
use crate::engine::diagnostics::Diagnostic;
use crate::engine::normalizer::normalize;
use crate::engine::rules::get_rule_title;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationResult {
    Passed {
        notice: Option<String>,
    },
    Failed {
        diagnostic: Diagnostic,
        user_input: String,
    },
}

impl ValidationResult {
    pub fn is_success(&self) -> bool {
        matches!(self, ValidationResult::Passed { .. })
    }
}

pub fn extract_user_answer(exercise: &Exercise, file_content: &str) -> String {
    // 1. Try to read the original template file from exercise.path
    if let Ok(orig_content) = std::fs::read_to_string(&exercise.path) {
        let orig_lines: Vec<&str> = orig_content.lines().collect();
        let user_lines: Vec<&str> = file_content.lines().collect();

        for (i, &orig_line) in orig_lines.iter().enumerate() {
            if orig_line.contains("___") {
                if let Some(&user_line) = user_lines.get(i) {
                    if let Some((prefix, suffix)) = orig_line.split_once("___") {
                        let user_trimmed = user_line.trim();
                        let prefix_trimmed = prefix.trim();
                        let suffix_trimmed = suffix.trim();

                        let mut answer = user_trimmed;
                        if !prefix_trimmed.is_empty() && answer.starts_with(prefix_trimmed) {
                            answer = &answer[prefix_trimmed.len()..];
                        }
                        if !suffix_trimmed.is_empty() && answer.ends_with(suffix_trimmed) {
                            answer = &answer[..answer.len() - suffix_trimmed.len()];
                        }
                        return answer.trim().to_string();
                    }
                }
            }
        }
    }

    // 2. Fallback heuristic: search for the exercise line
    for line in file_content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed.starts_with('>')
            || trimmed.starts_with("<!--")
        {
            continue;
        }
        if trimmed.starts_with("English:")
            || trimmed.starts_with("Prompt:")
            || trimmed.starts_with("Context:")
        {
            continue;
        }

        let mut candidates = vec![exercise.solution.as_str()];
        for alt in &exercise.alternatives {
            candidates.push(alt.as_str());
        }
        for cand in candidates {
            if trimmed.contains(cand) {
                return cand.to_string();
            }
        }

        if !trimmed.contains("___") && !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }

    if !exercise.solution.is_empty() {
        exercise.solution.clone()
    } else {
        "".to_string()
    }
}

pub fn validate_submission(
    exercise: &Exercise,
    user_input: &str,
    accent_mode: AccentMode,
) -> ValidationResult {
    // 1. Check primary solution
    match check_accent_match(
        user_input,
        &exercise.solution,
        accent_mode == AccentMode::Strict,
    ) {
        AccentResult::ExactMatch => return ValidationResult::Passed { notice: None },
        AccentResult::ForgivenMatch { tip, .. } => {
            return ValidationResult::Passed { notice: Some(tip) }
        }
        AccentResult::Mismatch => {}
    }

    // 2. Check alternative accepted forms
    for alt in &exercise.alternatives {
        match check_accent_match(user_input, alt, accent_mode == AccentMode::Strict) {
            AccentResult::ExactMatch => return ValidationResult::Passed { notice: None },
            AccentResult::ForgivenMatch { tip, .. } => {
                return ValidationResult::Passed { notice: Some(tip) }
            }
            AccentResult::Mismatch => {}
        }
    }

    // 3. Match against targeted diagnostic rules
    let norm_user = normalize(user_input);
    for rule in &exercise.diagnostic_rules {
        let norm_pat = normalize(&rule.pattern);
        if norm_user.contains(&norm_pat) || user_input.contains(&rule.pattern) {
            return ValidationResult::Failed {
                diagnostic: Diagnostic {
                    code: rule.code.clone(),
                    title: get_rule_title(&rule.code),
                    file_path: exercise.path.to_string_lossy().to_string(),
                    line_number: 1,
                    user_snippet: user_input.to_string(),
                    message: rule.message.clone(),
                    note: exercise.hints.get(2).cloned(),
                    help: exercise.hints.first().cloned(),
                    hint: exercise.hints.get(1).cloned(),
                },
                user_input: user_input.to_string(),
            };
        }
    }

    // 4. Fallback general diagnostic
    ValidationResult::Failed {
        diagnostic: Diagnostic {
            code: "E0001".to_string(),
            title: get_rule_title("E0001"),
            file_path: exercise.path.to_string_lossy().to_string(),
            line_number: 1,
            user_snippet: user_input.to_string(),
            message: format!("Expected '{}'.", exercise.solution),
            note: exercise.hints.get(2).cloned(),
            help: exercise.hints.first().cloned(),
            hint: exercise.hints.get(1).cloned(),
        },
        user_input: user_input.to_string(),
    }
}
