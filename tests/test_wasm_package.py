"""Tests for the Spanglings WebAssembly package and Node.js interoperability.

Validates the compilation artifacts in `docs/assets/playground/pkg/` including
`spanglings.js`, `spanglings_bg.wasm`, and TypeScript declarations, and executes
end-to-end Wasm instantiation and function calls via Node.js subprocess.
"""

from __future__ import annotations

import json
import subprocess
from pathlib import Path
from typing import Any

import pytest


@pytest.fixture(scope="module")
def repo_root() -> Path:
    """Returns repository root directory."""
    return Path(__file__).resolve().parent.parent


@pytest.fixture(scope="module")
def pkg_dir(repo_root: Path) -> Path:
    """Returns the path to the compiled wasm pkg directory."""
    return repo_root / "docs" / "assets" / "playground" / "pkg"


def run_wasm_node_script(pkg_dir: Path, js_body: str) -> dict[str, Any]:
    """Execute a Node.js snippet against the compiled WebAssembly package.

    Args:
        pkg_dir: Path to pkg directory.
        js_body: Body of the async function inside the Node script.

    Returns:
        Parsed JSON dictionary from Node execution output.
    """
    js_path = (pkg_dir / "spanglings.js").resolve().as_posix()
    wasm_path = (pkg_dir / "spanglings_bg.wasm").resolve().as_posix()

    node_script = f"""
import fs from 'node:fs';
import init, {{
    get_curriculum_catalog_json,
    evaluate_exercise_wasm,
    get_arcade_catalog_json,
    evaluate_arcade_choice_wasm,
    calculate_sm2_review_wasm
}} from '{js_path}';

async function main() {{
    const wasmBytes = fs.readFileSync('{wasm_path}');
    await init({{ module_or_path: wasmBytes }});
    {js_body}
}}

main().catch(err => {{
    console.error(err);
    process.exit(1);
}});
"""

    result = subprocess.run(
        ["node", "--input-type=module", "-e", node_script],
        capture_output=True,
        text=True,
        check=False,
    )

    assert result.returncode == 0, (
        f"Node Wasm execution failed:\nSTDOUT:\n{result.stdout}\nSTDERR:\n{result.stderr}"
    )

    return json.loads(result.stdout)


def test_wasm_package_files_exist(pkg_dir: Path) -> None:
    """Verify that required wasm artifacts exist and are non-empty."""
    js_file = pkg_dir / "spanglings.js"
    wasm_file = pkg_dir / "spanglings_bg.wasm"
    dts_file = pkg_dir / "spanglings.d.ts"

    assert js_file.exists(), f"Missing {js_file}"
    assert wasm_file.exists(), f"Missing {wasm_file}"
    assert dts_file.exists(), f"Missing {dts_file}"

    assert js_file.stat().st_size > 0, f"File {js_file} is empty"
    assert wasm_file.stat().st_size > 100_000, (
        f"File {wasm_file} is suspiciously small ({wasm_file.stat().st_size} bytes)"
    )
    assert dts_file.stat().st_size > 0, f"File {dts_file} is empty"


def test_wasm_declarations_and_exports(pkg_dir: Path) -> None:
    """Verify key exported functions exist in TypeScript definition and JS export."""
    dts_content = (pkg_dir / "spanglings.d.ts").read_text(encoding="utf-8")
    js_content = (pkg_dir / "spanglings.js").read_text(encoding="utf-8")

    expected_exports = [
        "get_curriculum_catalog_json",
        "evaluate_exercise_wasm",
        "get_arcade_catalog_json",
        "evaluate_arcade_choice_wasm",
        "calculate_sm2_review_wasm",
        "default",
        "initSync",
    ]

    for export_name in expected_exports:
        assert export_name in dts_content, (
            f"Expected export '{export_name}' missing from spanglings.d.ts"
        )
        assert export_name in js_content, (
            f"Expected export '{export_name}' missing from spanglings.js"
        )


def test_wasm_curriculum_catalog_introspection(pkg_dir: Path) -> None:
    """Verify curriculum catalog returns >100 exercises with dual-layer explanations."""
    js_body = """
    const raw = get_curriculum_catalog_json();
    const data = JSON.parse(raw);
    console.log(JSON.stringify(data));
    """
    catalog = run_wasm_node_script(pkg_dir, js_body)

    assert catalog["count"] >= 100, f"Expected >= 100 exercises, got {catalog['count']}"
    assert len(catalog["exercises"]) == catalog["count"]

    for ex in catalog["exercises"]:
        assert ex["id"], "Exercise missing id"
        assert ex["topic"], f"Exercise {ex['id']} missing topic"
        assert ex["solution"], f"Exercise {ex['id']} missing solution"
        assert ex["meaning"], f"Exercise {ex['id']} missing meaning"
        assert ex["plain_english"], f"Exercise {ex['id']} missing plain_english"
        assert ex["rule"], f"Exercise {ex['id']} missing rule"
        assert ex["explanation"], f"Exercise {ex['id']} missing explanation"


def test_wasm_exercise_evaluation_dual_layer(pkg_dir: Path) -> None:
    """Verify exercise evaluation with correct, incorrect, and not-found submissions."""
    js_body = """
    const correctRaw = evaluate_exercise_wasm('b0_ser_estar_basics_01', 'soy');
    const incorrectRaw = evaluate_exercise_wasm('b0_ser_estar_basics_01', 'estoy');
    const notFoundRaw = evaluate_exercise_wasm('nonexistent_id_999', 'test');

    console.log(JSON.stringify({
        correct: JSON.parse(correctRaw),
        incorrect: JSON.parse(incorrectRaw),
        notFound: JSON.parse(notFoundRaw)
    }));
    """
    res = run_wasm_node_script(pkg_dir, js_body)

    # Correct submission
    correct = res["correct"]
    assert correct["is_correct"] is True
    assert correct["solution"] == "soy"
    assert correct["meaning"].strip()
    assert correct["rule"].strip()
    assert correct["error_code"] is None

    # Incorrect submission
    incorrect = res["incorrect"]
    assert incorrect["is_correct"] is False
    assert incorrect["solution"] == "soy"
    assert incorrect["error_code"] == "E0101"
    assert incorrect["diagnostic"] is not None
    assert incorrect["meaning"].strip()
    assert incorrect["rule"].strip()

    # Not found submission
    not_found = res["notFound"]
    assert not_found["is_correct"] is False
    assert not_found["error_code"] == "NOT_FOUND"


def test_wasm_arcade_catalog_and_speed_scoring(pkg_dir: Path) -> None:
    """Verify arcade catalog and choice evaluation with speed scoring."""
    js_body = """
    const allCatalog = JSON.parse(get_arcade_catalog_json('all'));
    const showdownsCatalog = JSON.parse(get_arcade_catalog_json('showdowns'));
    const enginesCatalog = JSON.parse(get_arcade_catalog_json('engines'));
    const regimenCatalog = JSON.parse(get_arcade_catalog_json('regimen'));

    // Evaluate fast showdown response (< 2000ms)
    const firstShowdown = showdownsCatalog.items[0];
    const fastEval = JSON.parse(
        evaluate_arcade_choice_wasm(firstShowdown.id, firstShowdown.correct_option, 400n)
    );

    // Evaluate wrong showdown response
    const wrongEval = JSON.parse(
        evaluate_arcade_choice_wasm(firstShowdown.id, firstShowdown.options[1], 1500n)
    );

    // Evaluate specialized engine response
    const firstRegimen = regimenCatalog.items[0];
    const engineEval = JSON.parse(
        evaluate_arcade_choice_wasm(firstRegimen.id, firstRegimen.correct_option, 800n)
    );

    console.log(JSON.stringify({
        allCount: allCatalog.count,
        showdownsCount: showdownsCatalog.count,
        enginesCount: enginesCatalog.count,
        regimenCount: regimenCatalog.count,
        fastEval,
        wrongEval,
        engineEval
    }));
    """
    res = run_wasm_node_script(pkg_dir, js_body)

    assert res["allCount"] >= 200
    assert res["showdownsCount"] >= 180
    assert res["enginesCount"] >= 80
    assert res["regimenCount"] > 0

    # Fast correct showdown
    fast = res["fastEval"]
    assert fast["is_correct"] is True
    assert fast["score_delta"] > 100, f"Speed bonus expected, got {fast['score_delta']}"
    assert fast["meaning"].strip()
    assert fast["rule"].strip()

    # Wrong showdown
    wrong = res["wrongEval"]
    assert wrong["is_correct"] is False
    assert wrong["score_delta"] == 0
    assert wrong["meaning"].strip()
    assert wrong["rule"].strip()

    # Specialized engine
    engine = res["engineEval"]
    assert engine["is_correct"] is True
    assert engine["score_delta"] > 100
    assert engine["meaning"].strip()
    assert engine["rule"].strip()


def test_wasm_sm2_spaced_repetition_calculation(pkg_dir: Path) -> None:
    """Verify SM-2 algorithm calculations in WebAssembly engine."""
    js_body = """
    const grade5 = JSON.parse(calculate_sm2_review_wasm(2.5, 1, 1, 5));
    const grade0 = JSON.parse(calculate_sm2_review_wasm(2.5, 6, 2, 0));

    console.log(JSON.stringify({ grade5, grade0 }));
    """
    res = run_wasm_node_script(pkg_dir, js_body)

    # Grade 5: ease factor increases, interval scales to 6
    g5 = res["grade5"]
    assert g5["repetitions"] == 2
    assert g5["interval_days"] == 6
    assert g5["ease_factor"] >= 2.5
    assert g5["meaning"].strip()
    assert g5["rule"].strip()

    # Grade 0: repetitions reset to 0, interval resets to 1
    g0 = res["grade0"]
    assert g0["repetitions"] == 0
    assert g0["interval_days"] == 1
    assert g0["ease_factor"] < 2.5
    assert g0["meaning"].strip()
    assert g0["rule"].strip()
