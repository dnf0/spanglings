use crate::core::exercise::Exercise;
use crate::engine::accents::AccentMode;
use crate::engine::validator::{extract_user_answer, validate_submission, ValidationResult};
use crate::lsp::protocol::{Diagnostic, Position, Range};
use std::path::Path;

pub fn compute_diagnostics(_uri: &str, content: &str, strict_accents: bool) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    // 1. Check for `<!-- I AM NOT DONE -->` marker
    for (line_idx, line) in lines.iter().enumerate() {
        if line.contains("<!-- I AM NOT DONE -->") {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position {
                        line: line_idx as u32,
                        character: 0,
                    },
                    end: Position {
                        line: line_idx as u32,
                        character: line.len() as u32,
                    },
                },
                severity: Some(3), // Information
                code: Some("INFO01".to_string()),
                source: Some("spanglings".to_string()),
                message: "Exercise is marked as in-progress. Remove this comment when completed."
                    .to_string(),
            });
            break;
        }
    }

    // 2. Try parsing exercise from content
    let dummy_path = Path::new("exercise.md");
    if let Ok(exercise) = Exercise::from_markdown(dummy_path, content) {
        let accent_mode = if strict_accents {
            AccentMode::Strict
        } else {
            AccentMode::Forgiving
        };

        // Find the exercise prompt line number
        let mut prompt_line = 0;
        for (idx, line) in lines.iter().enumerate() {
            if line.contains("### Exercise") {
                prompt_line = idx + 1;
                break;
            }
        }

        let user_input = extract_user_answer(&exercise, content);
        if !user_input.trim().is_empty() {
            let res = validate_submission(&exercise, &user_input, accent_mode);
            match res {
                ValidationResult::Failed { diagnostic, .. } => {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: prompt_line as u32,
                                character: 0,
                            },
                            end: Position {
                                line: prompt_line as u32,
                                character: lines.get(prompt_line).map(|l| l.len()).unwrap_or(20)
                                    as u32,
                            },
                        },
                        severity: Some(1), // Error
                        code: Some(diagnostic.code),
                        source: Some("spanglings".to_string()),
                        message: format!("Grammar check failed: {}", diagnostic.message),
                    });
                }
                ValidationResult::Passed { notice } => {
                    if let Some(notice_msg) = notice {
                        if !strict_accents {
                            diagnostics.push(Diagnostic {
                                range: Range {
                                    start: Position {
                                        line: prompt_line as u32,
                                        character: 0,
                                    },
                                    end: Position {
                                        line: prompt_line as u32,
                                        character: lines
                                            .get(prompt_line)
                                            .map(|l| l.len())
                                            .unwrap_or(20)
                                            as u32,
                                    },
                                },
                                severity: Some(2), // Warning
                                code: Some("W0101".to_string()),
                                source: Some("spanglings".to_string()),
                                message: notice_msg,
                            });
                        }
                    }
                }
            }
        }
    }

    diagnostics
}
