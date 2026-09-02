"""Tests for the Spanglings Interactive Split-Pane Workspace & Monaco Editor.

Validates `docs/assets/playground/playground.css` and `docs/assets/playground/playground.js`,
verifying layout styles, developer-grade slate theme, zero animation disruptions,
progressive 3-tier hints, accent toolbar math, exercise evaluation across accent modes,
and storage synchronization.
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
def playground_css_path(repo_root: Path) -> Path:
    """Returns path to playground.css."""
    return repo_root / "docs" / "assets" / "playground" / "playground.css"


@pytest.fixture(scope="module")
def playground_js_path(repo_root: Path) -> Path:
    """Returns path to playground.js."""
    return repo_root / "docs" / "assets" / "playground" / "playground.js"


@pytest.fixture(scope="module")
def storage_js_path(repo_root: Path) -> Path:
    """Returns path to storage.js."""
    return repo_root / "docs" / "assets" / "playground" / "storage.js"


@pytest.fixture(scope="module")
def bundle_path(repo_root: Path) -> Path:
    """Returns path to playground-bundle.json."""
    return repo_root / "docs" / "assets" / "playground" / "playground-bundle.json"


def run_node_playground_eval(
    playground_js_path: Path,
    storage_js_path: Path,
    bundle_path: Path,
    script: str,
) -> dict[str, Any]:
    """Helper to execute JavaScript in Node.js importing playground.js and dependencies."""
    runner_code = f"""
    // Mock localStorage for headless Node environment
    class MockLocalStorage {{
        constructor() {{
            this.store = {{}};
        }}
        getItem(key) {{
            return Object.prototype.hasOwnProperty.call(this.store, key) ? this.store[key] : null;
        }}
        setItem(key, value) {{
            this.store[key] = String(value);
        }}
        removeItem(key) {{
            delete this.store[key];
        }}
        clear() {{
            this.store = {{}};
        }}
    }}

    globalThis.localStorage = new MockLocalStorage();
    globalThis.window = globalThis;
    globalThis.document = {{
        getElementById: () => null,
        querySelector: () => null,
        querySelectorAll: () => [],
        createElement: (tag) => ({{
            tagName: tag.toUpperCase(),
            style: {{}},
            classList: {{
                add() {{}},
                remove() {{}},
                toggle() {{}},
                contains() {{ return false; }}
            }},
            appendChild() {{}},
            setAttribute() {{}},
            getAttribute() {{ return null; }},
            addEventListener() {{}},
            innerHTML: '',
            textContent: '',
        }}),
    }};

    import * as fs from 'fs';
    import {{ SpanglingsStorage }} from '{storage_js_path.as_uri()}';
    import {{
        ACCENT_CHARS,
        normalizeSpanish,
        generateProgressiveHint,
        insertAccentAtCursor,
        evaluateExercise,
        buildSyllabusModel,
        SpanglingsPlaygroundApp,
    }} from '{playground_js_path.as_uri()}';

    const bundleData = JSON.parse(fs.readFileSync('{bundle_path.as_posix()}', 'utf-8'));

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
        return output_json
    except json.JSONDecodeError as exc:
        pytest.fail(
            f"Failed to decode JSON from node output:\n{res.stdout}\nError: {exc}"
        )


def test_playground_css_exists_and_conforms_to_spec(
    playground_css_path: Path,
) -> None:
    """Verify that playground.css exists, defines slate palette, and has zero shake animations."""
    assert playground_css_path.exists(), (
        f"playground.css missing at {playground_css_path}"
    )
    content = playground_css_path.read_text(encoding="utf-8")
    assert len(content) > 200, "playground.css is too short"

    # Slate dark palette checks
    assert "#0f172a" in content.lower() or "0f172a" in content.lower(), (
        "Missing slate-900 background #0f172a"
    )
    assert "#1e293b" in content.lower() or "1e293b" in content.lower(), (
        "Missing slate-800 surface #1e293b"
    )
    assert "#334155" in content.lower() or "334155" in content.lower(), (
        "Missing slate-700 border #334155"
    )

    # Key layout selectors
    assert ".playground-container" in content
    assert ".syllabus-pane" in content or "#syllabus-pane" in content
    assert ".editor-pane" in content or "#editor-pane" in content
    assert ".diagnostics-pane" in content or "#diagnostics-pane" in content
    assert ".accent-toolbar" in content or ".accent-btn" in content
    assert ".dual-layer-card" in content

    # Zero sound/shake/flashing constraint
    assert "@keyframes shake" not in content.lower()
    assert "@keyframes flash" not in content.lower()


def test_playground_js_exists(playground_js_path: Path) -> None:
    """Verify that playground.js exists and is non-empty."""
    assert playground_js_path.exists(), f"playground.js missing at {playground_js_path}"
    content = playground_js_path.read_text(encoding="utf-8")
    assert len(content) > 300, "playground.js is too short"


def test_accent_characters_constant(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify ACCENT_CHARS array contains all standard Spanish diacritics and punctuation."""
    script = """
    return { accents: ACCENT_CHARS };
    """
    out = run_node_playground_eval(
        playground_js_path, storage_js_path, bundle_path, script
    )
    accents = out["result"]["accents"]
    expected = ["á", "é", "í", "ó", "ú", "ñ", "ü", "¿", "¡"]
    for ch in expected:
        assert ch in accents, f"Missing accent character: {ch}"


def test_insert_accent_at_cursor_math(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify insertAccentAtCursor correctly updates text content and selection indices."""
    script = """
    // 1. Insert at end of text
    const r1 = insertAccentAtCursor('est', 3, 3, 'á');
    // 2. Insert at start of text
    const r2 = insertAccentAtCursor('que', 0, 0, '¿');
    // 3. Replace 'u' with 'ü' in 'verguenza'
    const r3 = insertAccentAtCursor('verguenza', 4, 5, 'ü');
    // 4. Replace selection range
    const r4 = insertAccentAtCursor('hola mundo', 5, 10, 'amigo');

    return { r1, r2, r3, r4 };
    """
    out = run_node_playground_eval(
        playground_js_path, storage_js_path, bundle_path, script
    )
    res = out["result"]

    assert res["r1"] == {"text": "está", "selectionStart": 4, "selectionEnd": 4}
    assert res["r2"] == {"text": "¿que", "selectionStart": 1, "selectionEnd": 1}
    assert res["r3"] == {"text": "vergüenza", "selectionStart": 5, "selectionEnd": 5}
    assert res["r4"] == {
        "text": "hola amigo",
        "selectionStart": 10,
        "selectionEnd": 10,
    }


def test_progressive_3tier_hints(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify 3-tier progressive hint generation for sentence frames."""
    script = """
    const frame = bundleData.frames.find(f => f.id === 'subjunctive-06') || bundleData.frames[0];
    const hint0 = generateProgressiveHint(frame, 0);
    const hint1 = generateProgressiveHint(frame, 1);
    const hint2 = generateProgressiveHint(frame, 2);
    const hint3 = generateProgressiveHint(frame, 3);

    return { frameId: frame.id, target: frame.target, hint0, hint1, hint2, hint3 };
    """
    out = run_node_playground_eval(
        playground_js_path, storage_js_path, bundle_path, script
    )
    res = out["result"]

    assert res["hint0"] is None or res["hint0"] == ""
    # Tier 1: Grammatical category and target cue
    assert "tier" in res["hint1"] and res["hint1"]["tier"] == 1
    assert "Grammar Cue" in res["hint1"]["title"] or "Tier 1" in res["hint1"]["title"]
    assert len(res["hint1"]["content"]) > 0

    # Tier 2: Mental model communicative context
    assert res["hint2"]["tier"] == 2
    assert (
        "Communicative Context" in res["hint2"]["title"]
        or "Tier 2" in res["hint2"]["title"]
    )
    assert len(res["hint2"]["content"]) > 0

    # Tier 3: Non-spoiling structural pattern (first/last letter pattern mask)
    assert res["hint3"]["tier"] == 3
    assert (
        "Structural Pattern" in res["hint3"]["title"]
        or "Tier 3" in res["hint3"]["title"]
    )
    pattern = res["hint3"]["pattern"]
    target = res["target"]
    assert pattern.startswith(target[0])
    assert pattern.endswith(target[-1])
    assert str(len(target)) in res["hint3"]["content"] or "_" in pattern


def test_exercise_evaluation_modes(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify evaluation across Strict, Forgiving, and Off accent modes."""
    script = """
    const frame = {
        id: 'test-subjunctive-05',
        topic: 'subjunctive',
        template: 'Ojalá nos ____ una oportunidad.',
        target: 'dé',
        explanation: 'dar in present subjunctive (dé)',
        plain_english: 'Expresses hope or wish.'
    };

    // 1. Exact match in Strict mode
    const exactStrict = evaluateExercise(frame, 'dé', 'Strict');
    
    // 2. Missing accent in Strict mode
    const missingStrict = evaluateExercise(frame, 'de', 'Strict');

    // 3. Missing accent in Forgiving mode
    const missingForgiving = evaluateExercise(frame, 'de', 'Forgiving');

    // 4. Exact in Forgiving mode
    const exactForgiving = evaluateExercise(frame, 'dé', 'Forgiving');

    // 5. Wrong word in any mode
    const wrongWord = evaluateExercise(frame, 'tuviera', 'Forgiving');

    // 6. Full sentence submission with target replaced
    const fullSentence = evaluateExercise(frame, 'Ojalá nos dé una oportunidad.', 'Strict');

    // 7. Full sentence submission missing accent in Forgiving
    const fullSentenceForgiving = evaluateExercise(frame, 'Ojalá nos de una oportunidad.', 'Forgiving');

    return {
        exactStrict,
        missingStrict,
        missingForgiving,
        exactForgiving,
        wrongWord,
        fullSentence,
        fullSentenceForgiving
    };
    """
    out = run_node_playground_eval(
        playground_js_path, storage_js_path, bundle_path, script
    )
    res = out["result"]

    # 1. Exact strict
    assert res["exactStrict"]["isValid"] is True
    assert res["exactStrict"]["score"] == 100
    assert res["exactStrict"]["accentWarning"] is None

    # 2. Missing strict
    assert res["missingStrict"]["isValid"] is False
    assert res["missingStrict"]["accentError"] is True
    assert "accent" in res["missingStrict"]["feedback"].lower()

    # 3. Missing forgiving -> valid with helpful reminder
    assert res["missingForgiving"]["isValid"] is True
    assert res["missingForgiving"]["score"] == 100
    assert res["missingForgiving"]["accentWarning"] is not None
    assert (
        "accent" in res["missingForgiving"]["accentWarning"].lower()
        or "dé" in res["missingForgiving"]["accentWarning"]
    )

    # 4. Exact forgiving
    assert res["exactForgiving"]["isValid"] is True
    assert res["exactForgiving"]["accentWarning"] is None

    # 5. Wrong word
    assert res["wrongWord"]["isValid"] is False
    assert (
        "meaning" in res["wrongWord"] or "plain_english" in res["wrongWord"]
    )  # dual-layer feedback
    assert "rule" in res["wrongWord"] or "explanation" in res["wrongWord"]

    # 6. Full sentence exact
    assert res["fullSentence"]["isValid"] is True

    # 7. Full sentence forgiving
    assert res["fullSentenceForgiving"]["isValid"] is True
    assert res["fullSentenceForgiving"]["accentWarning"] is not None


def test_build_syllabus_model_groups_and_progress(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify buildSyllabusModel correctly groups all 24 topics and computes progress."""
    script = """
    const storage = new SpanglingsStorage('test_syllabus_key');
    const state = storage.load();
    // Complete 2 exercises in subjunctive
    storage.recordReview('subjunctive', 'subjunctive-01', 4, true);
    storage.recordReview('subjunctive', 'subjunctive-02', 4, true);

    const syllabus = buildSyllabusModel(bundleData, storage);
    return { syllabus, totalTopics: syllabus.length };
    """
    out = run_node_playground_eval(
        playground_js_path, storage_js_path, bundle_path, script
    )
    res = out["result"]

    assert res["totalTopics"] == 24
    subjunctive_topic = next(
        (t for t in res["syllabus"] if t["slug"] == "subjunctive"), None
    )
    assert subjunctive_topic is not None
    assert subjunctive_topic["completedCount"] == 2
    assert subjunctive_topic["totalCount"] >= 2
    assert len(subjunctive_topic["frames"]) >= 2
    assert subjunctive_topic["frames"][0]["isCompleted"] is True
    assert subjunctive_topic["frames"][1]["isCompleted"] is True


def test_playground_app_state_lifecycle(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify SpanglingsPlaygroundApp initialization, exercise selection, submission, and reset."""
    script = """
    const storage = new SpanglingsStorage('test_app_lifecycle_key');
    storage.resetAll();

    const app = new SpanglingsPlaygroundApp({
        bundle: bundleData,
        storage: storage,
    });

    // 1. Initial exercise should be first available frame
    const initialFrame = app.getCurrentFrame();
    
    // 2. Select specific exercise
    app.selectExercise('subjunctive-06');
    const selectedFrame = app.getCurrentFrame();

    // 3. Submit valid answer
    const evalResult = app.submitExercise('tuviera');
    const isCompletedAfter = storage.load().completed_exercises.includes('subjunctive-06');

    // 4. Request hint progression
    const hint1 = app.nextHint();
    const hint2 = app.nextHint();
    const hint3 = app.nextHint();

    // 5. Reset exercise
    app.resetCurrentExercise();

    return {
        initialId: initialFrame.id,
        selectedId: selectedFrame.id,
        evalValid: evalResult.isValid,
        isCompletedAfter,
        hint1Tier: hint1.tier,
        hint2Tier: hint2.tier,
        hint3Tier: hint3.tier,
    };
    """
    out = run_node_playground_eval(
        playground_js_path, storage_js_path, bundle_path, script
    )
    res = out["result"]

    assert res["initialId"] is not None
    assert res["selectedId"] == "subjunctive-06"
    assert res["evalValid"] is True
    assert res["isCompletedAfter"] is True
    assert res["hint1Tier"] == 1
    assert res["hint2Tier"] == 2
    assert res["hint3Tier"] == 3
