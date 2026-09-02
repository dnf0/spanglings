"""Tests for SpanglingsStorage browser state persistence engine.

Validates that docs/assets/playground/storage.js maintains 100% JSON schema
parity and algorithmic fidelity with the Rust CLI AppState (src/core/state.rs)
and sync backup structures (src/cli/commands/sync.rs).
"""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
from typing import Any

import pytest


@pytest.fixture(scope="module")
def repo_root() -> Path:
    """Returns repository root directory."""
    return Path(__file__).resolve().parent.parent


@pytest.fixture(scope="module")
def storage_js_path(repo_root: Path) -> Path:
    """Returns path to storage.js."""
    return repo_root / "docs" / "assets" / "playground" / "storage.js"


def run_node_eval(
    storage_js_path: Path, script: str, mock_storage_broken: bool = False
) -> dict[str, Any]:
    """Helper to execute JavaScript in Node.js importing storage.js."""
    storage_setup = (
        """
    // Broken localStorage simulating SecurityError / private browsing block
    globalThis.localStorage = {
        getItem() { throw new Error('SecurityError: Access is denied'); },
        setItem() { throw new Error('SecurityError: Access is denied'); },
        removeItem() { throw new Error('SecurityError: Access is denied'); },
        clear() { throw new Error('SecurityError: Access is denied'); }
    };
    """
        if mock_storage_broken
        else """
    // Mock localStorage for headless Node environment
    class MockLocalStorage {
        constructor() {
            this.store = {};
        }
        getItem(key) {
            return Object.prototype.hasOwnProperty.call(this.store, key) ? this.store[key] : null;
        }
        setItem(key, value) {
            this.store[key] = String(value);
        }
        removeItem(key) {
            delete this.store[key];
        }
        clear() {
            this.store = {};
        }
    }

    globalThis.localStorage = new MockLocalStorage();
    """
    )

    runner_code = f"""
    {storage_setup}
    globalThis.window = globalThis;

    import {{ SpanglingsStorage, createDefaultState, STORAGE_KEY, APP_VERSION }} from '{storage_js_path.as_uri()}';

    async function main() {{
        {script}
    }}

    main().then(res => {{
        console.log(JSON.stringify({{ success: true, result: res }}));
    }}).catch(err => {{
        console.error(JSON.stringify({{ success: false, error: err.message, stack: err.stack }}));
        process.exit(1);
    }});
    """

    res = subprocess.run(
        ["node", "--input-type=module", "-e", runner_code],
        capture_output=True,
        text=True,
        check=False,
    )

    if res.returncode != 0:
        pytest.fail(
            f"Node execution failed with code {res.returncode}:\nSTDERR:\n{res.stderr}\nSTDOUT:\n{res.stdout}"
        )

    try:
        output_json = json.loads(res.stdout.strip())
    except json.JSONDecodeError as exc:
        pytest.fail(f"Failed to decode Node runner output: {res.stdout}\nError: {exc}")

    assert output_json.get("success") is True, (
        f"Script error: {output_json.get('error')}"
    )
    return output_json["result"]


def test_storage_file_exists(storage_js_path: Path) -> None:
    """Verify that docs/assets/playground/storage.js exists."""
    assert storage_js_path.exists(), f"File does not exist: {storage_js_path}"


def test_default_state_schema_parity(storage_js_path: Path) -> None:
    """Verify default state matches Rust AppState structure."""
    script = """
    const storage = new SpanglingsStorage();
    const defaultState = storage.load();
    return defaultState;
    """
    state = run_node_eval(storage_js_path, script)

    # Validate all core fields from src/core/state.rs
    assert state["version"] == 1
    assert isinstance(state["completed_exercises"], list)
    assert state["completed_exercises"] == []
    assert state["current_exercise"] is None
    assert state["accent_mode"] == "Forgiving"
    assert isinstance(state["srs"], dict)
    assert state["srs"] == {}
    assert isinstance(state["stats"], dict)
    assert state["stats"] == {}
    assert isinstance(state["activity_history"], dict)
    assert state["activity_history"] == {}
    assert state["evaluated_level"] is None
    assert isinstance(state["concept_mastery"], dict)
    assert state["concept_mastery"] == {}
    assert state["tour_completed"] is False
    assert isinstance(state["arcade_stats"], dict)
    assert state["arcade_stats"] == {
        "high_score": 0,
        "total_duels": 0,
        "best_streak": 0,
        "accuracy": 0.0,
    }


def test_mark_and_unmark_completed(storage_js_path: Path) -> None:
    """Verify marking exercise completed updates state, stats, and activity."""
    script = """
    const storage = new SpanglingsStorage();
    const initialCompleted = storage.isCompleted("b1_subj_01");
    storage.markCompleted("b1_subj_01", 2);
    const afterCompleted = storage.isCompleted("b1_subj_01");
    const stateAfter = storage.load();

    storage.unmarkCompleted("b1_subj_01");
    const afterUnmarkCompleted = storage.isCompleted("b1_subj_01");
    const stateAfterUnmark = storage.load();

    return {
        initialCompleted,
        afterCompleted,
        statAfter: stateAfter.stats["b1_subj_01"],
        activity: stateAfter.activity_history,
        afterUnmarkCompleted,
        statAfterUnmark: stateAfterUnmark.stats["b1_subj_01"],
    };
    """
    res = run_node_eval(storage_js_path, script)

    assert res["initialCompleted"] is False
    assert res["afterCompleted"] is True
    assert res["statAfter"]["attempts"] == 1
    assert res["statAfter"]["hints_used"] == 2
    assert res["statAfter"]["completed_at"] is not None
    assert len(res["activity"]) == 1

    assert res["afterUnmarkCompleted"] is False
    assert res["statAfterUnmark"]["completed_at"] is None


def test_srs_sm2_algorithm_fidelity(storage_js_path: Path) -> None:
    """Verify SM-2 calculation in storage.js matches Rust calculate_sm2_review."""
    script = """
    const storage = new SpanglingsStorage();
    const now = new Date("2026-09-02T12:00:00Z");

    // Review 1 with quality 5 (perfect)
    storage.updateSrs("ex1", 5, now);
    const s1 = storage.load().srs["ex1"];

    // Review 2 with quality 4 (good) -> 1 day after
    const now2 = new Date("2026-09-03T12:00:00Z");
    storage.updateSrs("ex1", 4, now2);
    const s2 = storage.load().srs["ex1"];

    // Review 3 with quality 4 -> 6 days after
    const now3 = new Date("2026-09-09T12:00:00Z");
    storage.updateSrs("ex1", 4, now3);
    const s3 = storage.load().srs["ex1"];

    // Review 4 with quality 1 (lapse)
    const now4 = new Date("2026-09-24T12:00:00Z");
    storage.updateSrs("ex1", 1, now4);
    const s4 = storage.load().srs["ex1"];

    return { s1, s2, s3, s4 };
    """
    res = run_node_eval(storage_js_path, script)

    # Initial EF = 2.5
    # Quality 5: new EF = 2.5 + (0.1 - 0) = 2.6. reps: 1, interval: 1
    assert res["s1"]["repetitions"] == 1
    assert res["s1"]["interval_days"] == 1
    assert pytest.approx(res["s1"]["ease_factor"], 0.01) == 2.6

    # Quality 4: new EF = 2.6 + (0.1 - 1 * 0.10) = 2.6. reps: 2, interval: 6
    assert res["s2"]["repetitions"] == 2
    assert res["s2"]["interval_days"] == 6
    assert pytest.approx(res["s2"]["ease_factor"], 0.01) == 2.6

    # Quality 4: new EF = 2.6 + 0.0 = 2.6. reps: 3, interval: round(6 * 2.6) = 16
    assert res["s3"]["repetitions"] == 3
    assert res["s3"]["interval_days"] == 16
    assert pytest.approx(res["s3"]["ease_factor"], 0.01) == 2.6

    # Quality 1 (< 3): lapse. reps: 0, interval: 1. new EF decreases: 2.6 + (0.1 - 4 * (0.08 + 0.08)) = 2.6 + (0.1 - 0.64) = 2.06
    assert res["s4"]["repetitions"] == 0
    assert res["s4"]["interval_days"] == 1
    assert pytest.approx(res["s4"]["ease_factor"], 0.01) == 2.06


def test_is_due_for_review_logic(storage_js_path: Path) -> None:
    """Verify isDueForReview correctly evaluates review timestamps."""
    script = """
    const storage = new SpanglingsStorage();
    const reviewDate = new Date("2026-09-02T12:00:00Z");
    storage.updateSrs("ex_review", 5, reviewDate); // interval_days = 1 -> due 2026-09-03T12:00:00Z

    const beforeDue = storage.isDueForReview("ex_review", new Date("2026-09-02T18:00:00Z"));
    const onDue = storage.isDueForReview("ex_review", new Date("2026-09-03T12:00:00Z"));
    const afterDue = storage.isDueForReview("ex_review", new Date("2026-09-04T12:00:00Z"));
    const nonexistent = storage.isDueForReview("nonexistent_id");

    return { beforeDue, onDue, afterDue, nonexistent };
    """
    res = run_node_eval(storage_js_path, script)

    assert res["beforeDue"] is False
    assert res["onDue"] is True
    assert res["afterDue"] is True
    assert res["nonexistent"] is False


def test_concept_mastery_calculation_fidelity(storage_js_path: Path) -> None:
    """Verify concept mastery tracking matches Rust update_concept_mastery."""
    script = """
    const storage = new SpanglingsStorage();
    const now1 = new Date("2026-09-02T12:00:00Z");
    storage.updateConceptMastery("subjunctive_weirdo", 5, now1);
    const m1 = storage.load().concept_mastery["subjunctive_weirdo"];

    const now2 = new Date("2026-09-03T12:00:00Z");
    storage.updateConceptMastery("subjunctive_weirdo", 4, now2);
    const m2 = storage.load().concept_mastery["subjunctive_weirdo"];

    return { m1, m2 };
    """
    res = run_node_eval(storage_js_path, script)

    # Initial: quality 5 -> reps: 1, interval: 1, EF: 2.6
    assert res["m1"]["concept_id"] == "subjunctive_weirdo"
    assert res["m1"]["repetitions"] == 1
    assert res["m1"]["interval_days"] == 1
    assert res["m1"]["total_reviews"] == 1
    assert res["m1"]["lapses"] == 0
    assert res["m1"]["mastery_score"] > 0.0

    # Review 2: quality 4 -> reps: 2, interval: 6, total_reviews: 2
    assert res["m2"]["repetitions"] == 2
    assert res["m2"]["interval_days"] == 6
    assert res["m2"]["total_reviews"] == 2
    assert res["m2"]["mastery_score"] > res["m1"]["mastery_score"]


def test_arcade_session_tracking(storage_js_path: Path) -> None:
    """Verify arcade stats tracking and rolling accuracy calculation."""
    script = """
    const storage = new SpanglingsStorage();
    storage.recordArcadeSession(500, 5, 10, 8); // 80% accuracy
    const s1 = storage.load().arcade_stats;

    storage.recordArcadeSession(1000, 10, 10, 10); // 100% accuracy -> avg 90%
    const s2 = storage.load().arcade_stats;

    return { s1, s2 };
    """
    res = run_node_eval(storage_js_path, script)

    assert res["s1"]["high_score"] == 500
    assert res["s1"]["best_streak"] == 5
    assert res["s1"]["total_duels"] == 1
    assert pytest.approx(res["s1"]["accuracy"], 0.01) == 0.8

    assert res["s2"]["high_score"] == 1000
    assert res["s2"]["best_streak"] == 10
    assert res["s2"]["total_duels"] == 2
    assert pytest.approx(res["s2"]["accuracy"], 0.01) == 0.9


def test_export_json_format_and_cli_parity(storage_js_path: Path) -> None:
    """Verify exportJson generates valid JSON matching Rust state schema."""
    script = """
    const storage = new SpanglingsStorage();
    storage.markCompleted("b1_subj_01");
    storage.updateSrs("b1_subj_01", 5, new Date("2026-09-02T12:00:00Z"));
    storage.updateConceptMastery("subjunctive_weirdo", 5, new Date("2026-09-02T12:00:00Z"));

    const exportRaw = storage.exportJson();
    return { exportRaw };
    """
    res = run_node_eval(storage_js_path, script)
    export_str = res["exportRaw"]

    parsed = json.loads(export_str)
    # Validate export can be loaded as backup or state
    if "state" in parsed:
        state_data = parsed["state"]
        assert "version" in parsed
        assert "exported_at" in parsed
        assert "completed_count" in parsed
    else:
        state_data = parsed

    assert state_data["version"] == 1
    assert "b1_subj_01" in state_data["completed_exercises"]
    assert "b1_subj_01" in state_data["srs"]
    assert "subjunctive_weirdo" in state_data["concept_mastery"]


def test_import_json_roundtrip_fidelity(storage_js_path: Path) -> None:
    """Verify importing raw AppState or PortableStateBackup merges seamlessly."""
    cli_backup = {
        "version": "0.5.4",
        "exported_at": "2026-09-02T12:00:00Z",
        "completed_count": 2,
        "srs_items_count": 2,
        "state": {
            "version": 1,
            "completed_exercises": ["b1_subj_01", "b1_por_para_01"],
            "current_exercise": "b1_subj_02",
            "accent_mode": "Strict",
            "srs": {
                "b1_subj_01": {
                    "repetitions": 3,
                    "interval_days": 16,
                    "ease_factor": 2.6,
                    "next_review_due": "2026-09-18T12:00:00Z",
                    "last_reviewed": "2026-09-02T12:00:00Z",
                }
            },
            "stats": {
                "b1_subj_01": {
                    "attempts": 4,
                    "completed_at": "2026-09-02T12:00:00Z",
                    "hints_used": 1,
                }
            },
            "activity_history": {"2026-09-02": 5},
            "evaluated_level": {
                "level": "B1",
                "score_percent": 88.5,
                "evaluated_at": "2026-09-02T12:00:00Z",
            },
            "concept_mastery": {
                "subjunctive_weirdo": {
                    "concept_id": "subjunctive_weirdo",
                    "mastery_score": 0.75,
                    "repetitions": 3,
                    "interval_days": 16,
                    "ease_factor": 2.6,
                    "total_reviews": 3,
                    "lapses": 0,
                    "last_practiced": "2026-09-02T12:00:00Z",
                }
            },
            "tour_completed": True,
            "arcade_stats": {
                "high_score": 1200,
                "total_duels": 15,
                "best_streak": 8,
                "accuracy": 0.93,
            },
        },
    }

    cli_backup_json = json.dumps(cli_backup)

    script = f"""
    const storage = new SpanglingsStorage();
    const importPayload = {json.dumps(cli_backup_json)};
    const success = storage.importJson(importPayload);
    const loadedState = storage.load();

    return {{ success, loadedState }};
    """
    res = run_node_eval(storage_js_path, script)

    assert res["success"] is True
    st = res["loadedState"]
    assert "b1_subj_01" in st["completed_exercises"]
    assert "b1_por_para_01" in st["completed_exercises"]
    assert st["current_exercise"] == "b1_subj_02"
    assert st["accent_mode"] == "Strict"
    assert st["srs"]["b1_subj_01"]["repetitions"] == 3
    assert st["activity_history"]["2026-09-02"] == 5
    assert st["evaluated_level"]["level"] == "B1"
    assert st["evaluated_level"]["score_percent"] == 88.5
    assert st["concept_mastery"]["subjunctive_weirdo"]["mastery_score"] == 0.75
    assert st["tour_completed"] is True
    assert st["arcade_stats"]["high_score"] == 1200


def test_import_raw_app_state_direct(storage_js_path: Path) -> None:
    """Verify importing raw AppState (unwrapped) works seamlessly."""
    raw_app_state = {
        "version": 1,
        "completed_exercises": ["b1_imperative_01"],
        "current_exercise": None,
        "accent_mode": "Forgiving",
        "srs": {},
        "stats": {},
        "activity_history": {"2026-09-02": 1},
        "evaluated_level": None,
        "concept_mastery": {},
        "tour_completed": False,
        "arcade_stats": {
            "high_score": 300,
            "total_duels": 1,
            "best_streak": 3,
            "accuracy": 1.0,
        },
    }

    script = f"""
    const storage = new SpanglingsStorage();
    const success = storage.importJson({json.dumps(json.dumps(raw_app_state))});
    const loaded = storage.load();
    return {{ success, loaded }};
    """
    res = run_node_eval(storage_js_path, script)

    assert res["success"] is True
    assert "b1_imperative_01" in res["loaded"]["completed_exercises"]
    assert res["loaded"]["arcade_stats"]["high_score"] == 300


def test_import_json_rejects_malformed_input(storage_js_path: Path) -> None:
    """Verify importJson returns false and protects state on corrupt input."""
    script = """
    const storage = new SpanglingsStorage();
    storage.markCompleted("b1_subj_01");

    const badJson = "{ invalid: json ";
    const badSuccess = storage.importJson(badJson);

    const nonObjectJson = "12345";
    const nonObjSuccess = storage.importJson(nonObjectJson);

    const stateStillValid = storage.load();

    return {
        badSuccess,
        nonObjSuccess,
        isCompleted: stateStillValid.completed_exercises.includes("b1_subj_01"),
    };
    """
    res = run_node_eval(storage_js_path, script)

    assert res["badSuccess"] is False
    assert res["nonObjSuccess"] is False
    assert res["isCompleted"] is True


def test_reset_exercise_and_reset_all(storage_js_path: Path) -> None:
    """Verify resetExercise and resetAll clear the state properly."""
    script = """
    const storage = new SpanglingsStorage();
    storage.markCompleted("b1_subj_01");
    storage.markCompleted("b1_subj_02");
    storage.updateSrs("b1_subj_01", 5);

    storage.resetExercise("b1_subj_01");
    const afterResetEx = storage.load();

    storage.resetAll();
    const afterResetAll = storage.load();

    return { afterResetEx, afterResetAll };
    """
    res = run_node_eval(storage_js_path, script)

    assert "b1_subj_01" not in res["afterResetEx"]["completed_exercises"]
    assert "b1_subj_02" in res["afterResetEx"]["completed_exercises"]
    assert "b1_subj_01" not in res["afterResetEx"]["srs"]

    assert res["afterResetAll"]["completed_exercises"] == []
    assert res["afterResetAll"]["srs"] == {}
    assert res["afterResetAll"]["tour_completed"] is False


def test_memory_storage_fallback_on_storage_error(storage_js_path: Path) -> None:
    """Verify in-memory fallback works when localStorage throws SecurityError."""
    script = """
    const storage = new SpanglingsStorage();
    storage.markCompleted("b1_subj_fallback");
    const completed = storage.isCompleted("b1_subj_fallback");
    const loaded = storage.load();
    return { completed, isIncluded: loaded.completed_exercises.includes("b1_subj_fallback") };
    """
    res = run_node_eval(storage_js_path, script, mock_storage_broken=True)

    assert res["completed"] is True
    assert res["isIncluded"] is True
