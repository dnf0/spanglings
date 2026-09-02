"""Tests for the Spanglings Rapid Arcade Arena Engine.

Validates `docs/assets/playground/playground.js` and `docs/assets/playground/playground.css`,
verifying arcade pool filtering (all drills, 16 showdown pairs, 5 specialized drill engines),
single-key hotkeys, speed bonus calculations, live dual-layer pedagogical feedback,
end-of-round mistake reviews, replay missed items, and storage state synchronization.
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


def run_node_arcade_eval(
    playground_js_path: Path,
    storage_js_path: Path,
    bundle_path: Path,
    script: str,
) -> dict[str, Any]:
    """Helper to execute JavaScript in Node.js importing arcade engine from playground.js."""
    runner_code = f"""
    // Mock browser environment for headless Node testing
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
        calculateSpeedBonus,
        filterArcadePool,
        evaluateArcadeChoice,
        HOTKEY_MAP,
        SHOWDOWN_PAIRS,
        SPECIALIZED_ENGINES,
        SpanglingsArcadeEngine,
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


def test_calculate_speed_bonus(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify speed bonus formula (+1 point per 15ms under 1500ms, max 100)."""
    script = """
    return {
        bonus0: calculateSpeedBonus(0),
        bonus300: calculateSpeedBonus(300),
        bonus750: calculateSpeedBonus(750),
        bonus1200: calculateSpeedBonus(1200),
        bonus1485: calculateSpeedBonus(1485),
        bonus1500: calculateSpeedBonus(1500),
        bonus2000: calculateSpeedBonus(2000),
        bonusNegative: calculateSpeedBonus(-50),
    };
    """
    out = run_node_arcade_eval(playground_js_path, storage_js_path, bundle_path, script)
    res = out["result"]

    assert res["bonus0"] == 100
    assert res["bonus300"] == 80  # (1500 - 300) / 15 = 80
    assert res["bonus750"] == 50  # (1500 - 750) / 15 = 50
    assert res["bonus1200"] == 20  # (1500 - 1200) / 15 = 20
    assert res["bonus1485"] == 1  # (1500 - 1485) / 15 = 1
    assert res["bonus1500"] == 0
    assert res["bonus2000"] == 0
    assert res["bonusNegative"] == 100


def test_arcade_pool_filtering(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify filtering arcade pool by 'all', 16 showdown pairs, and 5 specialized engines."""
    script = """
    const items = bundleData.arcade_items || [];
    
    // 1. All items
    const allPool = filterArcadePool(items, 'all');
    
    // 2. Showdown pairs
    const serEstar = filterArcadePool(items, 'ser-estar');
    const porPara = filterArcadePool(items, 'por-para');
    const pretImp = filterArcadePool(items, 'pret-imp');
    const subjInd = filterArcadePool(items, 'subj-ind');
    const saberConocer = filterArcadePool(items, 'saber-conocer');
    
    // 3. Specialized engines
    const regimen = filterArcadePool(items, 'regimen');
    const irregulars = filterArcadePool(items, 'irregulars');
    const falseFriends = filterArcadePool(items, 'false-friends');
    const seMatrix = filterArcadePool(items, 'se-matrix');
    const connectors = filterArcadePool(items, 'connectors');

    return {
        totalCount: items.length,
        allCount: allPool.length,
        serEstarCount: serEstar.length,
        porParaCount: porPara.length,
        pretImpCount: pretImp.length,
        subjIndCount: subjInd.length,
        saberConocerCount: saberConocer.length,
        regimenCount: regimen.length,
        irregularsCount: irregulars.length,
        falseFriendsCount: falseFriends.length,
        seMatrixCount: seMatrix.length,
        connectorsCount: connectors.length,
    };
    """
    out = run_node_arcade_eval(playground_js_path, storage_js_path, bundle_path, script)
    res = out["result"]

    assert res["totalCount"] >= 200
    assert res["allCount"] == res["totalCount"]
    assert res["serEstarCount"] > 0
    assert res["porParaCount"] > 0
    assert res["pretImpCount"] > 0
    assert res["subjIndCount"] > 0
    assert res["saberConocerCount"] > 0
    assert res["regimenCount"] > 0
    assert res["irregularsCount"] > 0
    assert res["falseFriendsCount"] > 0
    assert res["seMatrixCount"] > 0
    assert res["connectorsCount"] > 0


def test_evaluate_arcade_choice_dual_layer(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify evaluation of correct and incorrect arcade choices with dual-layer explanations."""
    script = """
    const item = {
        id: 'test-item-01',
        topic: 'ser-estar',
        trigger_sentence: 'Hoy ____ muy nublado en la costa.',
        options: ['está', 'es'],
        correct_index: 0,
        correct_option: 'está',
        explanation: 'Estar is used for weather conditions and states.',
        plain_english: 'Points to current condition / state right now.',
        meaning: 'Points to current condition / state right now.',
        rule: 'Estar is used for weather conditions and states.'
    };

    // 1. Correct choice with 450ms response time
    const correctRes = evaluateArcadeChoice(item, 0, 450);

    // 2. Incorrect choice with 300ms response time
    const incorrectRes = evaluateArcadeChoice(item, 1, 300);

    return { correctRes, incorrectRes };
    """
    out = run_node_arcade_eval(playground_js_path, storage_js_path, bundle_path, script)
    res = out["result"]

    # Correct response checks
    corr = res["correctRes"]
    assert corr["isCorrect"] is True
    assert corr["baseScore"] == 100
    assert corr["speedBonus"] == 70  # (1500 - 450) / 15 = 70
    assert corr["totalScore"] == 170
    assert "current condition" in corr["meaning"]
    assert "weather conditions" in corr["rule"]
    assert corr["triggerSentence"] == "Hoy ____ muy nublado en la costa."

    # Incorrect response checks
    inc = res["incorrectRes"]
    assert inc["isCorrect"] is False
    assert inc["baseScore"] == 0
    assert inc["speedBonus"] == 0
    assert inc["totalScore"] == 0
    assert inc["correctIndex"] == 0
    assert inc["correctOption"] == "está"
    assert inc["selectedIndex"] == 1
    assert inc["selectedOption"] == "es"


def test_arcade_engine_lifecycle_and_summary(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify full arcade round lifecycle, scoring, streak tracking, summary recap, and storage sync."""
    script = """
    const storage = new SpanglingsStorage('test_arcade_lifecycle_key');
    storage.resetAll();

    const engine = new SpanglingsArcadeEngine({
        bundle: bundleData,
        storage: storage,
    });

    // Start a 5-item round of 'ser-estar'
    engine.startRound('ser-estar', 5);
    const initialItem = engine.getCurrentItem();
    const stateAtStart = engine.state;

    // Item 1: Correct (speed bonus: 60) -> Streak 1
    const r1 = engine.submitChoice(initialItem.correct_index, 600);
    engine.nextQuestion();

    // Item 2: Correct (speed bonus: 80) -> Streak 2
    const item2 = engine.getCurrentItem();
    const r2 = engine.submitChoice(item2.correct_index, 300);
    engine.nextQuestion();

    // Item 3: Incorrect (streak resets to 0) -> Streak 0
    const item3 = engine.getCurrentItem();
    const wrongIdx = item3.correct_index === 0 ? 1 : 0;
    const r3 = engine.submitChoice(wrongIdx, 500);
    engine.nextQuestion();

    // Item 4: Correct (speed bonus: 50) -> Streak 1
    const item4 = engine.getCurrentItem();
    const r4 = engine.submitChoice(item4.correct_index, 750);
    engine.nextQuestion();

    // Item 5: Correct (speed bonus: 70) -> Streak 2
    const item5 = engine.getCurrentItem();
    const r5 = engine.submitChoice(item5.correct_index, 450);
    const summary = engine.nextQuestion(); // Finishes round

    const savedArcadeStats = storage.getState().arcade_stats;

    return {
        stateAtStart,
        stateAtEnd: engine.state,
        score: engine.score,
        bestStreak: engine.bestStreak,
        summary,
        savedArcadeStats,
        missedCount: summary.missedItems.length,
        isPerfect: summary.isPerfect,
    };
    """
    out = run_node_arcade_eval(playground_js_path, storage_js_path, bundle_path, script)
    res = out["result"]

    assert res["stateAtStart"] == "question"
    assert res["stateAtEnd"] == "summary"
    # Total score: (100+60) + (100+80) + 0 + (100+50) + (100+70) = 160 + 180 + 0 + 150 + 170 = 660
    assert res["score"] == 660
    assert res["bestStreak"] == 2
    assert res["summary"]["totalQuestions"] == 5
    assert res["summary"]["correctQuestions"] == 4
    assert res["summary"]["accuracy"] == 80.0
    assert res["missedCount"] == 1
    assert res["isPerfect"] is False

    # Check missed item details
    missed = res["summary"]["missedItems"][0]
    assert len(missed["triggerSentence"]) > 0
    assert len(missed["meaning"]) > 0
    assert len(missed["rule"]) > 0

    # Check storage synchronization
    stats = res["savedArcadeStats"]
    assert stats["total_duels"] >= 1
    assert stats["high_score"] >= 660
    assert stats["best_streak"] >= 2


def test_arcade_engine_perfect_run(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify perfect run (100% accuracy) produces celebratory summary with no mistakes."""
    script = """
    const storage = new SpanglingsStorage('test_arcade_perfect_key');
    const engine = new SpanglingsArcadeEngine({
        bundle: bundleData,
        storage: storage,
    });

    engine.startRound('por-para', 3);
    for (let i = 0; i < 3; i++) {
        const item = engine.getCurrentItem();
        engine.submitChoice(item.correct_index, 300);
        engine.nextQuestion();
    }

    const summary = engine.getSummary();
    return { summary };
    """
    out = run_node_arcade_eval(playground_js_path, storage_js_path, bundle_path, script)
    summary = out["result"]["summary"]

    assert summary["accuracy"] == 100.0
    assert summary["isPerfect"] is True
    assert len(summary["missedItems"]) == 0


def test_arcade_engine_replay_missed_items(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify replaying missed items starts a new round exclusively with missed questions."""
    script = """
    const storage = new SpanglingsStorage('test_arcade_replay_key');
    const engine = new SpanglingsArcadeEngine({
        bundle: bundleData,
        storage: storage,
    });

    engine.startRound('subj-ind', 4);
    // 1. Wrong
    let it = engine.getCurrentItem();
    engine.submitChoice(it.correct_index === 0 ? 1 : 0, 400);
    engine.nextQuestion();

    // 2. Correct
    it = engine.getCurrentItem();
    engine.submitChoice(it.correct_index, 400);
    engine.nextQuestion();

    // 3. Wrong
    it = engine.getCurrentItem();
    engine.submitChoice(it.correct_index === 0 ? 1 : 0, 400);
    engine.nextQuestion();

    // 4. Correct
    it = engine.getCurrentItem();
    engine.submitChoice(it.correct_index, 400);
    const sum1 = engine.nextQuestion();

    // Replay missed items (should have 2 items)
    engine.replayMissedItems();
    const replayCount = engine.items.length;
    const replayItem1 = engine.getCurrentItem();

    return {
        initialMissedCount: sum1.missedItems.length,
        replayCount,
        replayState: engine.state,
        firstReplayId: replayItem1.id,
    };
    """
    out = run_node_arcade_eval(playground_js_path, storage_js_path, bundle_path, script)
    res = out["result"]

    assert res["initialMissedCount"] == 2
    assert res["replayCount"] == 2
    assert res["replayState"] == "question"
    assert res["firstReplayId"] is not None


def test_arcade_hotkey_inputs(
    playground_js_path: Path, storage_js_path: Path, bundle_path: Path
) -> None:
    """Verify single-key hotkey mapping (1/j/J, 2/k/K, 3/l/L, 4/;/:, Space/Enter)."""
    script = """
    const storage = new SpanglingsStorage('test_arcade_hotkeys_key');
    const engine = new SpanglingsArcadeEngine({
        bundle: bundleData,
        storage: storage,
    });

    engine.startRound('all', 10);

    // 1. Hotkey 'j' chooses index 0
    const item1 = engine.getCurrentItem();
    const resJ = engine.handleKey('j');
    const stateAfterJ = engine.state; // should be 'feedback'

    // Advance with Space
    const resSpace = engine.handleKey(' ');
    const stateAfterSpace = engine.state; // should be 'question'

    // 2. Hotkey '2' chooses index 1
    const res2 = engine.handleKey('2');
    const stateAfter2 = engine.state; // should be 'feedback'

    // Advance with Enter
    engine.handleKey('Enter');
    const stateAfterEnter = engine.state; // should be 'question'

    return {
        HOTKEY_MAP,
        stateAfterJ,
        stateAfterSpace,
        stateAfter2,
        stateAfterEnter,
        chosenOption1: resJ.selectedOption,
        expectedOption1: item1.options[0],
    };
    """
    out = run_node_arcade_eval(playground_js_path, storage_js_path, bundle_path, script)
    res = out["result"]

    assert res["stateAfterJ"] == "feedback"
    assert res["stateAfterSpace"] == "question"
    assert res["stateAfter2"] == "feedback"
    assert res["stateAfterEnter"] == "question"
    assert res["chosenOption1"] == res["expectedOption1"]


def test_arcade_css_structure_and_constraints(playground_css_path: Path) -> None:
    """Verify playground.css contains arcade styling rules and satisfies zero animation constraints."""
    content = playground_css_path.read_text(encoding="utf-8")

    # Arcade structural components
    assert ".arcade-arena-container" in content
    assert ".arcade-card" in content or ".arcade-hud" in content
    assert ".arcade-choice-btn" in content or ".arcade-option-btn" in content
    assert ".arcade-hotkey-badge" in content or ".arcade-key" in content
    assert ".arcade-feedback-banner" in content or ".arcade-status" in content
    assert ".arcade-mistakes-card" in content or ".arcade-mistake" in content
    assert ".arcade-streak-badge" in content
    assert ".arcade-speed-bonus" in content

    # Zero sound/shake/flash animations constraint
    assert "@keyframes shake" not in content.lower()
    assert "@keyframes flash" not in content.lower()


def test_arcade_css_selector_targets_spanglings_playground(
    playground_css_path: Path,
) -> None:
    """Verify arcade-mode CSS selectors target .spanglings-playground.arcade-mode and #spanglings-app.arcade-mode."""
    content = playground_css_path.read_text(encoding="utf-8")
    assert (
        ".spanglings-playground.arcade-mode" in content
        or "#spanglings-app.arcade-mode" in content
    )
    assert (
        ".spanglings-playground.arcade-mode .syllabus-pane" in content
        or "#spanglings-app.arcade-mode .syllabus-pane" in content
    )
    assert (
        ".spanglings-playground.arcade-mode .arcade-arena-container" in content
        or "#spanglings-app.arcade-mode .arcade-arena-container" in content
    )

