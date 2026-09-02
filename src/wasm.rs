//! WebAssembly bridge and evaluator interface for Spanglings in browser environments.
//!
//! Provides zero-filesystem execution of curriculum catalog introspection, exercise
//! evaluation, rapid arcade showdowns, specialized drill engines, and SM-2 spaced repetition
//! calculations. All responses serialize to JSON containing dual-layer explanations.

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};

use crate::core::arcade::{
    canonicalize_engine_slug, get_showdown_pool, list_showdown_pairs, list_specialized_engines,
    ShowdownPair,
};
use crate::core::curriculum::find_exercise_by_query;
use crate::core::embedded::get_embedded_exercises;
use crate::core::exercise::Exercise;
use crate::core::reference::{get_grammar_concept, get_mental_model_for_topic};
use crate::core::srs::{calculate_sm2_review, SrsItem};
use crate::engine::accents::{check_accent_match, AccentMode, AccentResult};
use crate::engine::diagnostics::Diagnostic;
use crate::engine::validator::{validate_submission, ValidationResult};

/// Serializable exercise catalog item containing full metadata and dual-layer explanations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmExerciseItem {
    pub id: String,
    pub level: String,
    pub topic: String,
    pub exercise_type: String,
    pub title: String,
    pub solution: String,
    pub alternatives: Vec<String>,
    pub hints: Vec<String>,
    pub concept_tags: Vec<String>,
    pub prerequisites: Vec<String>,
    pub grammar_focus: Option<String>,
    pub contrast_note: Option<String>,
    pub trigger_sentence: String,
    pub prompt_cue: String,
    pub target_verb: String,
    pub target_subject: String,
    pub raw_content: String,
    pub meaning: String,
    pub plain_english: String,
    pub rule: String,
    pub explanation: String,
}

/// Catalog payload holding the complete collection of curriculum exercises.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmCurriculumCatalog {
    pub count: usize,
    pub exercises: Vec<WasmExerciseItem>,
}

/// Result of evaluating a user submission for an exercise.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmExerciseEvaluation {
    pub is_correct: bool,
    pub user_input: String,
    pub solution: String,
    pub alternatives: Vec<String>,
    pub notice: Option<String>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub diagnostic: Option<Diagnostic>,
    pub meaning: String,
    pub plain_english: String,
    pub rule: String,
    pub explanation: String,
}

/// Serializable item for binary showdowns and 4-choice rapid arcade drills.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmArcadeItem {
    pub id: String,
    pub topic: String,
    pub trigger_sentence: String,
    pub prompt_cue: String,
    pub options: Vec<String>,
    pub correct_index: usize,
    pub correct_option: String,
    pub meaning: String,
    pub plain_english: String,
    pub rule: String,
    pub explanation: String,
}

/// Catalog payload holding available arcade modes and drill items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmArcadeCatalog {
    pub mode: String,
    pub available_modes: Vec<String>,
    pub count: usize,
    pub items: Vec<WasmArcadeItem>,
}

/// Result of evaluating an arcade question choice with speed scoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmArcadeEvaluation {
    pub is_correct: bool,
    pub item_id: String,
    pub user_choice: String,
    pub correct_option: String,
    pub score_delta: i32,
    pub elapsed_ms: u64,
    pub meaning: String,
    pub plain_english: String,
    pub rule: String,
    pub explanation: String,
}

/// Result of updating SM-2 spaced repetition state for an item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmSm2Result {
    pub repetitions: u32,
    pub interval_days: u32,
    pub ease_factor: f32,
    pub next_review_due: String,
    pub last_reviewed: Option<String>,
    pub meaning: String,
    pub plain_english: String,
    pub rule: String,
    pub explanation: String,
}

/// Converts a static or loaded Exercise into a WasmExerciseItem with dual-layer explanations.
pub fn exercise_to_wasm_item(ex: &Exercise) -> WasmExerciseItem {
    // Attempt extracting drill-level prompt metadata if available
    let drill = ex.to_drill_items().into_iter().next();
    let topic_slug = if let Some(concept) = get_grammar_concept(&ex.topic) {
        concept.slug.to_string()
    } else {
        ex.topic.clone()
    };

    // Communicative mental model layer
    let mental_model = get_mental_model_for_topic(&topic_slug)
        .or_else(|| get_mental_model_for_topic(&ex.topic))
        .unwrap_or("");

    let plain_english = if let Some(ref d) = drill {
        if !d.plain_english.is_empty() {
            d.plain_english.clone()
        } else if !mental_model.is_empty() {
            mental_model.to_string()
        } else {
            format!("Mastery concept: {}", ex.title)
        }
    } else if !mental_model.is_empty() {
        mental_model.to_string()
    } else {
        format!("Mastery concept: {}", ex.title)
    };

    // Structural grammar rule layer
    let explanation = if let Some(ref d) = drill {
        if !d.explanation.is_empty() {
            d.explanation.clone()
        } else if let Some(ref focus) = ex.grammar_focus {
            focus.clone()
        } else {
            ex.hints
                .first()
                .cloned()
                .unwrap_or_else(|| format!("{}: {}", ex.id, ex.title))
        }
    } else if let Some(ref focus) = ex.grammar_focus {
        focus.clone()
    } else {
        ex.hints
            .first()
            .cloned()
            .unwrap_or_else(|| format!("{}: {}", ex.id, ex.title))
    };

    let trigger_sentence = drill
        .as_ref()
        .map(|d| d.trigger_sentence.clone())
        .unwrap_or_default();
    let prompt_cue = drill
        .as_ref()
        .map(|d| d.formula_cue.clone())
        .unwrap_or_else(|| ex.title.clone());
    let target_verb = drill
        .as_ref()
        .map(|d| d.target_verb.clone())
        .unwrap_or_default();
    let target_subject = drill
        .as_ref()
        .map(|d| d.target_subject.clone())
        .unwrap_or_default();

    WasmExerciseItem {
        id: ex.id.clone(),
        level: ex.level.to_string(),
        topic: ex.topic.clone(),
        exercise_type: ex.exercise_type.to_string(),
        title: ex.title.clone(),
        solution: ex.solution.clone(),
        alternatives: ex.alternatives.clone(),
        hints: ex.hints.clone(),
        concept_tags: ex.concept_tags.clone(),
        prerequisites: ex.prerequisites.clone(),
        grammar_focus: ex.grammar_focus.clone(),
        contrast_note: ex.contrast_note.clone(),
        trigger_sentence,
        prompt_cue,
        target_verb,
        target_subject,
        raw_content: ex.raw_content.clone(),
        meaning: plain_english.clone(),
        plain_english,
        rule: explanation.clone(),
        explanation,
    }
}

/// Returns the complete curriculum catalog JSON embedded directly in the WebAssembly binary.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn get_curriculum_catalog_json() -> String {
    // Load embedded exercises compiled into the binary
    let exercises = get_embedded_exercises().unwrap_or_default();
    let items: Vec<WasmExerciseItem> = exercises.iter().map(exercise_to_wasm_item).collect();

    let catalog = WasmCurriculumCatalog {
        count: items.len(),
        exercises: items,
    };

    serde_json::to_string(&catalog).unwrap_or_else(|_| "{\"count\":0,\"exercises\":[]}".to_string())
}

/// Evaluates a user submission against an exercise or frame ID without filesystem access.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn evaluate_exercise_wasm(frame_id: &str, user_input: &str) -> String {
    let exercises = get_embedded_exercises().unwrap_or_default();
    let maybe_ex = find_exercise_by_query(&exercises, frame_id);

    if let Some(ex) = maybe_ex {
        let item = exercise_to_wasm_item(ex);
        // Execute strict accent and diagnostic matching
        let validation = validate_submission(ex, user_input, AccentMode::Strict);

        match validation {
            ValidationResult::Passed { notice } => {
                let eval = WasmExerciseEvaluation {
                    is_correct: true,
                    user_input: user_input.to_string(),
                    solution: ex.solution.clone(),
                    alternatives: ex.alternatives.clone(),
                    notice,
                    error_code: None,
                    error_message: None,
                    diagnostic: None,
                    meaning: item.meaning,
                    plain_english: item.plain_english,
                    rule: item.rule,
                    explanation: item.explanation,
                };
                serde_json::to_string(&eval).unwrap_or_else(|_| "{}".to_string())
            }
            ValidationResult::Failed { diagnostic, .. } => {
                let code = diagnostic.code.clone();
                let msg = diagnostic.message.clone();
                let rule_expl = if !diagnostic.message.is_empty() {
                    diagnostic.message.clone()
                } else {
                    item.rule.clone()
                };
                let eval = WasmExerciseEvaluation {
                    is_correct: false,
                    user_input: user_input.to_string(),
                    solution: ex.solution.clone(),
                    alternatives: ex.alternatives.clone(),
                    notice: None,
                    error_code: Some(code),
                    error_message: Some(msg),
                    diagnostic: Some(diagnostic),
                    meaning: item.meaning,
                    plain_english: item.plain_english,
                    rule: rule_expl.clone(),
                    explanation: rule_expl,
                };
                serde_json::to_string(&eval).unwrap_or_else(|_| "{}".to_string())
            }
        }
    } else {
        // Fallback guard when exercise ID is unknown or omitted
        let eval = WasmExerciseEvaluation {
            is_correct: false,
            user_input: user_input.to_string(),
            solution: String::new(),
            alternatives: Vec::new(),
            notice: None,
            error_code: Some("NOT_FOUND".to_string()),
            error_message: Some(format!("Exercise '{}' not found in curriculum", frame_id)),
            diagnostic: None,
            meaning: String::new(),
            plain_english: String::new(),
            rule: String::new(),
            explanation: String::new(),
        };
        serde_json::to_string(&eval).unwrap_or_else(|_| "{}".to_string())
    }
}

/// Returns a list of all available arcade mode identifiers.
fn get_all_available_modes() -> Vec<String> {
    let mut modes = Vec::new();
    for pair in list_showdown_pairs() {
        modes.push(pair.slug().to_string());
    }
    for engine in list_specialized_engines() {
        modes.push((*engine).to_string());
    }
    modes
}

/// Returns the arcade catalog JSON containing showdowns and specialized engine drills.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn get_arcade_catalog_json(mode: &str) -> String {
    let available_modes = get_all_available_modes();
    let clean_mode = mode.trim().to_lowercase();
    let mut items = Vec::new();

    // 1. Specific showdown pair requested
    if let Some(pair) = ShowdownPair::from_str(&clean_mode) {
        let pool = get_showdown_pool(pair);
        for (idx, sentence) in pool.iter().enumerate() {
            items.push(WasmArcadeItem {
                id: format!("{}_{}", pair.slug(), idx),
                topic: pair.slug().to_string(),
                trigger_sentence: sentence.sentence.to_string(),
                prompt_cue: pair.title().to_string(),
                options: vec![sentence.target.to_string(), sentence.distractor.to_string()],
                correct_index: 0,
                correct_option: sentence.target.to_string(),
                meaning: sentence.plain_english.to_string(),
                plain_english: sentence.plain_english.to_string(),
                rule: sentence.explanation.to_string(),
                explanation: sentence.explanation.to_string(),
            });
        }
    }
    // 2. Specific specialized drill engine requested
    else if let Some((canonical_slug, cue_title, pool)) = canonicalize_engine_slug(&clean_mode) {
        for (idx, entry) in pool.iter().enumerate() {
            items.push(WasmArcadeItem {
                id: format!("{}_{}", canonical_slug, idx),
                topic: canonical_slug.to_string(),
                trigger_sentence: entry.sentence.to_string(),
                prompt_cue: cue_title.to_string(),
                options: vec![
                    entry.target.to_string(),
                    entry.distractors[0].to_string(),
                    entry.distractors[1].to_string(),
                    entry.distractors[2].to_string(),
                ],
                correct_index: 0,
                correct_option: entry.target.to_string(),
                meaning: entry.plain_english.to_string(),
                plain_english: entry.plain_english.to_string(),
                rule: entry.explanation.to_string(),
                explanation: entry.explanation.to_string(),
            });
        }
    }
    // 3. Showdowns group requested
    else if clean_mode == "showdowns" || clean_mode == "showdown" {
        for pair in list_showdown_pairs() {
            let pool = get_showdown_pool(pair);
            for (idx, sentence) in pool.iter().enumerate() {
                items.push(WasmArcadeItem {
                    id: format!("{}_{}", pair.slug(), idx),
                    topic: pair.slug().to_string(),
                    trigger_sentence: sentence.sentence.to_string(),
                    prompt_cue: pair.title().to_string(),
                    options: vec![sentence.target.to_string(), sentence.distractor.to_string()],
                    correct_index: 0,
                    correct_option: sentence.target.to_string(),
                    meaning: sentence.plain_english.to_string(),
                    plain_english: sentence.plain_english.to_string(),
                    rule: sentence.explanation.to_string(),
                    explanation: sentence.explanation.to_string(),
                });
            }
        }
    }
    // 4. Specialized engines group requested
    else if clean_mode == "engines" || clean_mode == "specialized" {
        for engine_slug in list_specialized_engines() {
            if let Some((canonical_slug, cue_title, pool)) = canonicalize_engine_slug(engine_slug) {
                for (idx, entry) in pool.iter().enumerate() {
                    items.push(WasmArcadeItem {
                        id: format!("{}_{}", canonical_slug, idx),
                        topic: canonical_slug.to_string(),
                        trigger_sentence: entry.sentence.to_string(),
                        prompt_cue: cue_title.to_string(),
                        options: vec![
                            entry.target.to_string(),
                            entry.distractors[0].to_string(),
                            entry.distractors[1].to_string(),
                            entry.distractors[2].to_string(),
                        ],
                        correct_index: 0,
                        correct_option: entry.target.to_string(),
                        meaning: entry.plain_english.to_string(),
                        plain_english: entry.plain_english.to_string(),
                        rule: entry.explanation.to_string(),
                        explanation: entry.explanation.to_string(),
                    });
                }
            }
        }
    }
    // 5. Default: All showdowns and specialized engines combined
    else {
        for pair in list_showdown_pairs() {
            let pool = get_showdown_pool(pair);
            for (idx, sentence) in pool.iter().enumerate() {
                items.push(WasmArcadeItem {
                    id: format!("{}_{}", pair.slug(), idx),
                    topic: pair.slug().to_string(),
                    trigger_sentence: sentence.sentence.to_string(),
                    prompt_cue: pair.title().to_string(),
                    options: vec![sentence.target.to_string(), sentence.distractor.to_string()],
                    correct_index: 0,
                    correct_option: sentence.target.to_string(),
                    meaning: sentence.plain_english.to_string(),
                    plain_english: sentence.plain_english.to_string(),
                    rule: sentence.explanation.to_string(),
                    explanation: sentence.explanation.to_string(),
                });
            }
        }
        for engine_slug in list_specialized_engines() {
            if let Some((canonical_slug, cue_title, pool)) = canonicalize_engine_slug(engine_slug) {
                for (idx, entry) in pool.iter().enumerate() {
                    items.push(WasmArcadeItem {
                        id: format!("{}_{}", canonical_slug, idx),
                        topic: canonical_slug.to_string(),
                        trigger_sentence: entry.sentence.to_string(),
                        prompt_cue: cue_title.to_string(),
                        options: vec![
                            entry.target.to_string(),
                            entry.distractors[0].to_string(),
                            entry.distractors[1].to_string(),
                            entry.distractors[2].to_string(),
                        ],
                        correct_index: 0,
                        correct_option: entry.target.to_string(),
                        meaning: entry.plain_english.to_string(),
                        plain_english: entry.plain_english.to_string(),
                        rule: entry.explanation.to_string(),
                        explanation: entry.explanation.to_string(),
                    });
                }
            }
        }
    }

    let catalog = WasmArcadeCatalog {
        mode: if clean_mode.is_empty() {
            "all".to_string()
        } else {
            clean_mode
        },
        available_modes,
        count: items.len(),
        items,
    };

    serde_json::to_string(&catalog).unwrap_or_else(|_| {
        "{\"mode\":\"all\",\"available_modes\":[],\"count\":0,\"items\":[]}".to_string()
    })
}

/// Evaluates a user choice for an arcade drill item with speed scoring and dual-layer feedback.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn evaluate_arcade_choice_wasm(item_id: &str, user_choice: &str, elapsed_ms: u64) -> String {
    let clean_choice = user_choice.trim();

    // 1. Attempt resolving item from showdown or specialized engine pools via item_id
    if let Some((prefix, idx_str)) = item_id.split_once('_') {
        let idx = idx_str.parse::<usize>().unwrap_or(0);

        // Check if prefix corresponds to a showdown pair
        if let Some(pair) = ShowdownPair::from_str(prefix) {
            let pool = get_showdown_pool(pair);
            if let Some(sentence) = pool.get(idx).or_else(|| pool.first()) {
                let is_correct = clean_choice.eq_ignore_ascii_case(sentence.target)
                    || (clean_choice == "0")
                    || (check_accent_match(clean_choice, sentence.target, false)
                        != AccentResult::Mismatch);

                let score_delta = if is_correct {
                    let speed_bonus = if elapsed_ms < 2000 {
                        ((2000 - elapsed_ms) / 20) as i32
                    } else {
                        0
                    };
                    100 + speed_bonus
                } else {
                    0
                };

                let eval = WasmArcadeEvaluation {
                    is_correct,
                    item_id: item_id.to_string(),
                    user_choice: user_choice.to_string(),
                    correct_option: sentence.target.to_string(),
                    score_delta,
                    elapsed_ms,
                    meaning: sentence.plain_english.to_string(),
                    plain_english: sentence.plain_english.to_string(),
                    rule: sentence.explanation.to_string(),
                    explanation: sentence.explanation.to_string(),
                };
                return serde_json::to_string(&eval).unwrap_or_else(|_| "{}".to_string());
            }
        }

        // Check if prefix corresponds to a specialized drill engine
        if let Some((_, _, pool)) = canonicalize_engine_slug(prefix) {
            if let Some(entry) = pool.get(idx).or_else(|| pool.first()) {
                let is_correct = clean_choice.eq_ignore_ascii_case(entry.target)
                    || (clean_choice == "0")
                    || (check_accent_match(clean_choice, entry.target, false)
                        != AccentResult::Mismatch);

                let score_delta = if is_correct {
                    let speed_bonus = if elapsed_ms < 2000 {
                        ((2000 - elapsed_ms) / 20) as i32
                    } else {
                        0
                    };
                    100 + speed_bonus
                } else {
                    0
                };

                let eval = WasmArcadeEvaluation {
                    is_correct,
                    item_id: item_id.to_string(),
                    user_choice: user_choice.to_string(),
                    correct_option: entry.target.to_string(),
                    score_delta,
                    elapsed_ms,
                    meaning: entry.plain_english.to_string(),
                    plain_english: entry.plain_english.to_string(),
                    rule: entry.explanation.to_string(),
                    explanation: entry.explanation.to_string(),
                };
                return serde_json::to_string(&eval).unwrap_or_else(|_| "{}".to_string());
            }
        }
    }

    // 2. Fallback: check if item_id matches a showdown pair directly
    if let Some(pair) = ShowdownPair::from_str(item_id) {
        let pool = get_showdown_pool(pair);
        if let Some(sentence) = pool.first() {
            let is_correct = clean_choice.eq_ignore_ascii_case(sentence.target)
                || (clean_choice == "0")
                || (check_accent_match(clean_choice, sentence.target, false)
                    != AccentResult::Mismatch);

            let score_delta = if is_correct { 100 } else { 0 };

            let eval = WasmArcadeEvaluation {
                is_correct,
                item_id: item_id.to_string(),
                user_choice: user_choice.to_string(),
                correct_option: sentence.target.to_string(),
                score_delta,
                elapsed_ms,
                meaning: sentence.plain_english.to_string(),
                plain_english: sentence.plain_english.to_string(),
                rule: sentence.explanation.to_string(),
                explanation: sentence.explanation.to_string(),
            };
            return serde_json::to_string(&eval).unwrap_or_else(|_| "{}".to_string());
        }
    }

    // 3. Fallback: check if item_id matches an exercise in the curriculum
    let exercises = get_embedded_exercises().unwrap_or_default();
    if let Some(ex) = find_exercise_by_query(&exercises, item_id) {
        let item = exercise_to_wasm_item(ex);
        let is_correct = clean_choice.eq_ignore_ascii_case(&ex.solution)
            || ex
                .alternatives
                .iter()
                .any(|alt| clean_choice.eq_ignore_ascii_case(alt))
            || (check_accent_match(clean_choice, &ex.solution, false) != AccentResult::Mismatch);

        let score_delta = if is_correct { 100 } else { 0 };

        let eval = WasmArcadeEvaluation {
            is_correct,
            item_id: item_id.to_string(),
            user_choice: user_choice.to_string(),
            correct_option: ex.solution.clone(),
            score_delta,
            elapsed_ms,
            meaning: item.meaning,
            plain_english: item.plain_english,
            rule: item.rule,
            explanation: item.explanation,
        };
        return serde_json::to_string(&eval).unwrap_or_else(|_| "{}".to_string());
    }

    // 4. Default fallback when item ID cannot be resolved
    let eval = WasmArcadeEvaluation {
        is_correct: false,
        item_id: item_id.to_string(),
        user_choice: user_choice.to_string(),
        correct_option: String::new(),
        score_delta: 0,
        elapsed_ms,
        meaning: String::new(),
        plain_english: String::new(),
        rule: String::new(),
        explanation: String::new(),
    };
    serde_json::to_string(&eval).unwrap_or_else(|_| "{}".to_string())
}

/// Calculates next interval, repetitions, and ease factor using the SM-2 algorithm.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn calculate_sm2_review_wasm(
    ease_factor: f32,
    interval: u32,
    repetitions: u32,
    grade: u8,
) -> String {
    let now = chrono::Utc::now();
    let initial_item = SrsItem {
        repetitions,
        interval_days: interval,
        ease_factor,
        next_review_due: now,
        last_reviewed: None,
    };
    let updated = calculate_sm2_review(&initial_item, grade, now);

    let meaning = "Spaced repetition (SM-2) schedules active recall intervals to maximize long-term synaptic retention before memory decay occurs.".to_string();
    let rule = format!(
        "SM-2 Algorithm: Grade {} (0-5) adjusted Ease Factor to {:.2} and calculated next interval of {} day(s) (repetitions: {}).",
        grade.clamp(0, 5),
        updated.ease_factor,
        updated.interval_days,
        updated.repetitions
    );

    let res = WasmSm2Result {
        repetitions: updated.repetitions,
        interval_days: updated.interval_days,
        ease_factor: updated.ease_factor,
        next_review_due: updated.next_review_due.to_rfc3339(),
        last_reviewed: updated.last_reviewed.map(|dt| dt.to_rfc3339()),
        meaning: meaning.clone(),
        plain_english: meaning,
        rule: rule.clone(),
        explanation: rule,
    };

    serde_json::to_string(&res).unwrap_or_else(|_| "{}".to_string())
}
