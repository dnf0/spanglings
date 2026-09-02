//! Integration tests for Spanglings WebAssembly interface and evaluator bridge.

use serde_json::Value;
use spanglings::wasm::{
    calculate_sm2_review_wasm, evaluate_arcade_choice_wasm, evaluate_exercise_wasm,
    get_arcade_catalog_json, get_curriculum_catalog_json, WasmArcadeCatalog, WasmArcadeEvaluation,
    WasmCurriculumCatalog, WasmExerciseEvaluation, WasmSm2Result,
};

#[test]
fn test_get_curriculum_catalog_json() {
    let json_str = get_curriculum_catalog_json();
    assert!(!json_str.is_empty(), "Catalog JSON should not be empty");

    let catalog: WasmCurriculumCatalog =
        serde_json::from_str(&json_str).expect("Should parse as WasmCurriculumCatalog");

    assert!(
        catalog.count >= 50,
        "Expected at least 50 exercises, got {}",
        catalog.count
    );
    assert_eq!(catalog.count, catalog.exercises.len());

    for ex in &catalog.exercises {
        assert!(!ex.id.is_empty(), "Exercise id must not be empty");
        assert!(!ex.level.is_empty(), "Level must not be empty");
        assert!(!ex.topic.is_empty(), "Topic must not be empty");
        assert!(!ex.title.is_empty(), "Title must not be empty");
        assert!(!ex.solution.is_empty(), "Solution must not be empty");
        assert!(
            !ex.meaning.is_empty(),
            "Dual-layer meaning must not be empty for {}",
            ex.id
        );
        assert!(
            !ex.plain_english.is_empty(),
            "Dual-layer plain_english must not be empty for {}",
            ex.id
        );
        assert!(
            !ex.rule.is_empty(),
            "Dual-layer rule must not be empty for {}",
            ex.id
        );
        assert!(
            !ex.explanation.is_empty(),
            "Dual-layer explanation must not be empty for {}",
            ex.id
        );
    }
}

#[test]
fn test_evaluate_exercise_wasm_valid() {
    let catalog_json = get_curriculum_catalog_json();
    let catalog: WasmCurriculumCatalog = serde_json::from_str(&catalog_json).unwrap();
    let first = catalog
        .exercises
        .first()
        .expect("Should have at least one exercise");

    let eval_json = evaluate_exercise_wasm(&first.id, &first.solution);
    let eval: WasmExerciseEvaluation =
        serde_json::from_str(&eval_json).expect("Should parse as WasmExerciseEvaluation");

    assert!(
        eval.is_correct,
        "Expected exact solution to pass evaluation"
    );
    assert_eq!(eval.user_input, first.solution);
    assert_eq!(eval.solution, first.solution);
    assert!(eval.error_code.is_none());
    assert!(!eval.meaning.is_empty());
    assert!(!eval.rule.is_empty());
}

#[test]
fn test_evaluate_exercise_wasm_invalid() {
    let catalog_json = get_curriculum_catalog_json();
    let catalog: WasmCurriculumCatalog = serde_json::from_str(&catalog_json).unwrap();
    let first = catalog
        .exercises
        .first()
        .expect("Should have at least one exercise");

    let wrong_input = "completamente_incorrecto_xyz";
    let eval_json = evaluate_exercise_wasm(&first.id, wrong_input);
    let eval: WasmExerciseEvaluation =
        serde_json::from_str(&eval_json).expect("Should parse as WasmExerciseEvaluation");

    assert!(
        !eval.is_correct,
        "Expected wrong solution to fail evaluation"
    );
    assert_eq!(eval.user_input, wrong_input);
    assert_eq!(eval.solution, first.solution);
    assert!(
        eval.error_code.is_some(),
        "Diagnostic error code should be present on failure"
    );
    assert!(!eval.meaning.is_empty());
    assert!(!eval.rule.is_empty());
}

#[test]
fn test_evaluate_exercise_wasm_not_found() {
    let eval_json = evaluate_exercise_wasm("nonexistent_exercise_id_99999", "test");
    let eval: WasmExerciseEvaluation =
        serde_json::from_str(&eval_json).expect("Should parse as WasmExerciseEvaluation");

    assert!(!eval.is_correct);
    assert_eq!(eval.error_code.as_deref(), Some("NOT_FOUND"));
}

#[test]
fn test_get_arcade_catalog_json_all_and_modes() {
    // 1. All modes
    let all_json = get_arcade_catalog_json("all");
    let all_cat: WasmArcadeCatalog = serde_json::from_str(&all_json).unwrap();
    assert!(
        all_cat.count > 100,
        "Expected arcade catalog to have >100 items, got {}",
        all_cat.count
    );
    assert!(all_cat.available_modes.contains(&"por-para".to_string()));
    assert!(all_cat.available_modes.contains(&"regimen".to_string()));

    // 2. Specific showdown pair
    let pp_json = get_arcade_catalog_json("por-para");
    let pp_cat: WasmArcadeCatalog = serde_json::from_str(&pp_json).unwrap();
    assert!(pp_cat.count > 0);
    for item in &pp_cat.items {
        assert_eq!(item.topic, "por-para");
        assert_eq!(item.options.len(), 2);
        assert!(!item.meaning.is_empty());
        assert!(!item.rule.is_empty());
    }

    // 3. Specific specialized engine
    let reg_json = get_arcade_catalog_json("regimen");
    let reg_cat: WasmArcadeCatalog = serde_json::from_str(&reg_json).unwrap();
    assert!(reg_cat.count > 0);
    for item in &reg_cat.items {
        assert_eq!(item.topic, "regimen");
        assert_eq!(item.options.len(), 4);
        assert!(!item.meaning.is_empty());
        assert!(!item.rule.is_empty());
    }

    // 4. Showdowns group
    let sd_json = get_arcade_catalog_json("showdowns");
    let sd_cat: WasmArcadeCatalog = serde_json::from_str(&sd_json).unwrap();
    assert!(sd_cat.count > 100);

    // 5. Engines group
    let eng_json = get_arcade_catalog_json("engines");
    let eng_cat: WasmArcadeCatalog = serde_json::from_str(&eng_json).unwrap();
    assert!(eng_cat.count > 30);
}

#[test]
fn test_evaluate_arcade_choice_wasm_showdown() {
    let pp_json = get_arcade_catalog_json("por-para");
    let pp_cat: WasmArcadeCatalog = serde_json::from_str(&pp_json).unwrap();
    let first_item = pp_cat.items.first().unwrap();

    // Correct choice with fast response time (300ms)
    let eval_json = evaluate_arcade_choice_wasm(&first_item.id, &first_item.correct_option, 300);
    let eval: WasmArcadeEvaluation = serde_json::from_str(&eval_json).unwrap();
    assert!(eval.is_correct);
    assert_eq!(eval.correct_option, first_item.correct_option);
    assert!(
        eval.score_delta > 100,
        "Expected score with speed bonus > 100, got {}",
        eval.score_delta
    );
    assert!(!eval.meaning.is_empty());
    assert!(!eval.rule.is_empty());

    // Incorrect choice
    let wrong_choice = if first_item.correct_option == "para" {
        "por"
    } else {
        "para"
    };
    let eval_wrong_json = evaluate_arcade_choice_wasm(&first_item.id, wrong_choice, 300);
    let eval_wrong: WasmArcadeEvaluation = serde_json::from_str(&eval_wrong_json).unwrap();
    assert!(!eval_wrong.is_correct);
    assert_eq!(eval_wrong.score_delta, 0);
}

#[test]
fn test_evaluate_arcade_choice_wasm_engine() {
    let reg_json = get_arcade_catalog_json("regimen");
    let reg_cat: WasmArcadeCatalog = serde_json::from_str(&reg_json).unwrap();
    let first_item = reg_cat.items.first().unwrap();

    // Correct choice
    let eval_json = evaluate_arcade_choice_wasm(&first_item.id, &first_item.correct_option, 500);
    let eval: WasmArcadeEvaluation = serde_json::from_str(&eval_json).unwrap();
    assert!(eval.is_correct);
    assert_eq!(eval.correct_option, first_item.correct_option);
    assert!(eval.score_delta >= 100);
    assert!(!eval.meaning.is_empty());
    assert!(!eval.rule.is_empty());

    // Incorrect choice
    let eval_wrong_json = evaluate_arcade_choice_wasm(&first_item.id, "opcion_erronea_xyz", 500);
    let eval_wrong: WasmArcadeEvaluation = serde_json::from_str(&eval_wrong_json).unwrap();
    assert!(!eval_wrong.is_correct);
    assert_eq!(eval_wrong.score_delta, 0);
}

#[test]
fn test_calculate_sm2_review_wasm() {
    // 1. Initial review with quality = 5 (perfect recall)
    let res_json = calculate_sm2_review_wasm(2.5, 0, 0, 5);
    let res: WasmSm2Result = serde_json::from_str(&res_json).unwrap();
    assert_eq!(res.repetitions, 1);
    assert_eq!(res.interval_days, 1);
    assert!(
        res.ease_factor > 2.5,
        "Ease factor should increase on grade 5"
    );
    assert!(!res.meaning.is_empty());
    assert!(!res.rule.is_empty());
    assert!(res.last_reviewed.is_some());

    // 2. Second review with quality = 4
    let res_json_2 = calculate_sm2_review_wasm(res.ease_factor, 1, 1, 4);
    let res_2: WasmSm2Result = serde_json::from_str(&res_json_2).unwrap();
    assert_eq!(res_2.repetitions, 2);
    assert_eq!(res_2.interval_days, 6);

    // 3. Failed review with quality = 1 (lapse)
    let res_json_fail = calculate_sm2_review_wasm(2.6, 15, 3, 1);
    let res_fail: WasmSm2Result = serde_json::from_str(&res_json_fail).unwrap();
    assert_eq!(
        res_fail.repetitions, 0,
        "Repetitions should reset on grade < 3"
    );
    assert_eq!(
        res_fail.interval_days, 1,
        "Interval should reset to 1 on lapse"
    );
    assert!(
        res_fail.ease_factor < 2.6,
        "Ease factor should decrease on grade 1"
    );

    // Verify valid JSON object serialization
    let raw_val: Value = serde_json::from_str(&res_json).unwrap();
    assert!(raw_val.get("next_review_due").is_some());
    assert!(raw_val.get("meaning").is_some());
    assert!(raw_val.get("plain_english").is_some());
    assert!(raw_val.get("rule").is_some());
    assert!(raw_val.get("explanation").is_some());
}

#[test]
fn test_evaluate_exercise_wasm_accent_handling() {
    let catalog_json = get_curriculum_catalog_json();
    let catalog: WasmCurriculumCatalog = serde_json::from_str(&catalog_json).unwrap();

    // Find an exercise that contains accented characters in the solution and does not list the unaccented form in alternatives
    let accented_ex = catalog.exercises.iter().find(|e| {
        let has_accent = e.solution.contains('á')
            || e.solution.contains('é')
            || e.solution.contains('í')
            || e.solution.contains('ó')
            || e.solution.contains('ú');
        if !has_accent {
            return false;
        }
        let unaccented = e
            .solution
            .replace('á', "a")
            .replace('é', "e")
            .replace('í', "i")
            .replace('ó', "o")
            .replace('ú', "u");
        !e.alternatives
            .iter()
            .any(|alt| alt.eq_ignore_ascii_case(&unaccented))
    });

    if let Some(ex) = accented_ex {
        // Exact submission must pass
        let eval_exact: WasmExerciseEvaluation =
            serde_json::from_str(&evaluate_exercise_wasm(&ex.id, &ex.solution)).unwrap();
        assert!(eval_exact.is_correct);

        // Stripped accent submission should fail under strict accent mode in evaluate_exercise_wasm
        let unaccented = ex
            .solution
            .replace('á', "a")
            .replace('é', "e")
            .replace('í', "i")
            .replace('ó', "o")
            .replace('ú', "u");
        if unaccented != ex.solution {
            let eval_unaccented: WasmExerciseEvaluation =
                serde_json::from_str(&evaluate_exercise_wasm(&ex.id, &unaccented)).unwrap();
            assert!(
                !eval_unaccented.is_correct,
                "Strict accent mode should reject missing accents for {}",
                ex.id
            );
            assert!(eval_unaccented.error_code.is_some());
        }
    }
}

#[test]
fn test_all_wasm_exports_json_structure() {
    // 1. Curriculum catalog JSON
    let cat_val: Value = serde_json::from_str(&get_curriculum_catalog_json()).unwrap();
    assert!(cat_val.get("count").is_some());
    assert!(cat_val.get("exercises").is_some());

    // 2. Exercise evaluation JSON
    let eval_val: Value =
        serde_json::from_str(&evaluate_exercise_wasm("ser_estar_1", "soy")).unwrap();
    assert!(eval_val.get("is_correct").is_some());
    assert!(eval_val.get("meaning").is_some());
    assert!(eval_val.get("plain_english").is_some());
    assert!(eval_val.get("rule").is_some());
    assert!(eval_val.get("explanation").is_some());

    // 3. Arcade catalog JSON
    let arc_val: Value = serde_json::from_str(&get_arcade_catalog_json("por-para")).unwrap();
    assert!(arc_val.get("mode").is_some());
    assert!(arc_val.get("available_modes").is_some());
    assert!(arc_val.get("items").is_some());

    // 4. Arcade choice evaluation JSON
    let choice_val: Value =
        serde_json::from_str(&evaluate_arcade_choice_wasm("por-para_0", "por", 250)).unwrap();
    assert!(choice_val.get("is_correct").is_some());
    assert!(choice_val.get("score_delta").is_some());
    assert!(choice_val.get("meaning").is_some());
    assert!(choice_val.get("rule").is_some());

    // 5. SM-2 review JSON
    let sm2_val: Value = serde_json::from_str(&calculate_sm2_review_wasm(2.5, 1, 1, 4)).unwrap();
    assert!(sm2_val.get("repetitions").is_some());
    assert!(sm2_val.get("interval_days").is_some());
    assert!(sm2_val.get("ease_factor").is_some());
}
